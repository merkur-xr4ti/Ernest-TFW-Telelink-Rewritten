//! The MSComm control (Telelink.Comm1) and the payphone call it carries.
//!
//! Like the original, the serial port is opened when the program starts (Module8 sub03BF:
//! "1200,E,7,1", ATZ, the init string) and stays open; a call to a phone is made from the
//! Connect form and stays up until Hang Up, while the Direct windows' Transfer menus move data
//! over it. All port work happens on one worker thread so the windows never freeze.

use crate::proto::{self, Clock, Line, Map, Session};
use crate::sip::{SipConfig, SipModem};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// The phone to call, as frmConnect takes it.
#[derive(Clone, Debug, Default)]
pub struct Target {
    /// Digits to dial (PIC, 1+, area code and number as typed).
    pub dial: String,
    /// The number as shown ("555-010-1234").
    pub shown: String,
    pub id: String,
    pub d3: bool,
}

#[derive(Clone, Debug)]
pub struct DialConf {
    pub init_addons: String,
    pub d3_addons: String,
    pub d1_addons: String,
    pub pbx: String,
    pub dcw: String,
    pub timeout: u32,
}

#[derive(Clone, Debug)]
pub enum Op {
    ReadSpeedDial,
    WriteSpeedDial(Vec<String>),
    ReadAreas(Vec<(u16, usize)>),
    WriteRecords(Vec<(u16, Vec<u8>)>),
    ReadClock,
    WriteClock(Clock),
    ReadScroll,
    WriteScroll(String),
    ReadFirmware,
    ReadCollection,
    ClearCounters,
    ReadCsn,
    WriteCsn(Vec<Vec<u8>>),
    WriteAos(usize, Vec<u8>),
    WriteRate(Vec<u8>),
    ReadRate,
    ReadSmdr,
    ClearSmdr,
    DownloadFirmware(proto::FirmwarePlan, bool),
}

#[derive(Clone, Debug, Default)]
pub struct Collection {
    pub coin_box_cents: Option<u64>,
    pub cumulative_cents: Option<u64>,
    pub refund_calls: Option<u64>,
    pub refund_cents: Option<u64>,
    pub raw: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum Res {
    Connected { firmware: String },
    Done,
    SpeedDial(Vec<String>),
    Areas(Vec<(u16, Vec<u8>)>),
    Clock(Clock),
    Scroll(String),
    Firmware { primary: String, secondary: String, running: String },
    Collection(Collection),
    Csn(Vec<(String, String, String, String, String, String)>),
    Smdr(Vec<String>),
    Rate(Vec<u8>),
}

/// What a running job shows (frmConnect's status frame, the batch log).
#[derive(Default)]
pub struct JobState {
    pub status: String,
    pub lines: Vec<String>,
    pub running: bool,
    pub outcome: Option<Result<Res, String>>,
    pub taken: bool,
}

#[derive(Clone)]
pub struct Job {
    pub state: Arc<Mutex<JobState>>,
    pub cancel: Arc<AtomicBool>,
}

impl Job {
    fn new() -> Job {
        Job { state: Arc::new(Mutex::new(JobState { running: true, ..Default::default() })), cancel: Arc::new(AtomicBool::new(false)) }
    }
    pub fn take(&self) -> Option<Result<Res, String>> {
        let mut s = self.state.lock().unwrap();
        if s.running || s.taken {
            return None;
        }
        s.taken = true;
        s.outcome.clone()
    }
    pub fn status(&self) -> String {
        self.state.lock().unwrap().status.clone()
    }
    pub fn running(&self) -> bool {
        self.state.lock().unwrap().running
    }
}

enum Cmd {
    Open(LineChoice),
    Close,
    Send(Vec<u8>),
    Baud(u32),
    Dial(Target, DialConf, Job),
    Run(Op, Job),
    HangUp,
}

#[derive(Default)]
pub struct Shared {
    pub open: bool,
    /// The open line, for messages ("COM2", "SIP modem (...)").
    pub line: String,
    /// Carrier detect (`Comm1.CDHolding`).
    pub cd: bool,
    /// Text received outside a job (`Comm1.Input`), for the incoming-call code.
    pub rx: String,
    pub open_error: Option<String>,
    /// The phone this program is connected to.
    pub call: Option<(Target, String)>,
    pub busy: bool,
}

#[derive(Clone)]
pub struct Modem {
    tx: Sender<Cmd>,
    pub sh: Arc<Mutex<Shared>>,
}

impl Modem {
    pub fn start() -> Modem {
        let (tx, rx) = mpsc::channel();
        let sh = Arc::new(Mutex::new(Shared::default()));
        let s2 = sh.clone();
        std::thread::spawn(move || worker(rx, s2));
        Modem { tx, sh }
    }
    pub fn open(&self, line: LineChoice) {
        let _ = self.tx.send(Cmd::Open(line));
    }
    pub fn close(&self) {
        let _ = self.tx.send(Cmd::Close);
    }
    /// `Comm1.Output = s`.
    pub fn send(&self, s: &str) {
        let _ = self.tx.send(Cmd::Send(s.as_bytes().to_vec()));
    }
    /// `Comm1.Settings = "1200,E,7,1"` / `"300,E,7,1"`.
    pub fn baud(&self, b: u32) {
        let _ = self.tx.send(Cmd::Baud(b));
    }
    pub fn dial(&self, t: Target, conf: DialConf) -> Job {
        let j = Job::new();
        let _ = self.tx.send(Cmd::Dial(t, conf, j.clone()));
        j
    }
    pub fn run(&self, op: Op) -> Job {
        let j = Job::new();
        let _ = self.tx.send(Cmd::Run(op, j.clone()));
        j
    }
    pub fn hang_up(&self) {
        let _ = self.tx.send(Cmd::HangUp);
    }
    pub fn connected(&self) -> Option<(Target, String)> {
        self.sh.lock().unwrap().call.clone()
    }
    pub fn take_rx(&self) -> String {
        std::mem::take(&mut self.sh.lock().unwrap().rx)
    }
}

/// Where the modem is: a Hayes modem on a COM port, or the built-in SIP modem.
#[derive(Clone, Debug, PartialEq)]
pub enum LineChoice {
    Com(u8),
    Sip(SipConfig),
}

impl LineChoice {
    /// From TFW.INI: `[Modem] Line=SIP` picks the SIP modem, otherwise `SerialPort`.
    pub fn from_profile(p: &crate::text::Profile) -> LineChoice {
        if p.get("Modem", "Line").trim().eq_ignore_ascii_case("SIP") {
            LineChoice::Sip(SipConfig::load(p))
        } else {
            LineChoice::Com(p.get("Modem", "SerialPort").trim().parse().unwrap_or(0))
        }
    }
    pub fn label(&self) -> String {
        match self {
            LineChoice::Com(n) => format!("COM{n}"),
            LineChoice::Sip(c) => format!("SIP modem ({}@{})", c.user, c.server),
        }
    }
}

/// The open modem.
enum Port {
    Serial(Box<dyn serialport::SerialPort>),
    Sip(SipModem),
}

impl Port {
    fn open(choice: &LineChoice, baud: u32) -> Result<Port, String> {
        match choice {
            LineChoice::Sip(cfg) => Ok(Port::Sip(SipModem::start(cfg.clone()))),
            LineChoice::Com(n) => {
                let name = format!("COM{n}");
                serialport::new(&name, baud)
                    .parity(serialport::Parity::Even)
                    .data_bits(serialport::DataBits::Seven)
                    .stop_bits(serialport::StopBits::One)
                    .timeout(Duration::from_millis(50))
                    .open()
                    .map(Port::Serial)
                    .map_err(|e| format!("That serial port is in use. ({name}: {e})"))
            }
        }
    }
    fn write(&mut self, data: &[u8]) -> Result<(), String> {
        match self {
            Port::Serial(p) => p.write_all(data).map_err(|e| e.to_string()),
            Port::Sip(m) => {
                m.write(data);
                Ok(())
            }
        }
    }
    fn read(&mut self) -> Vec<u8> {
        match self {
            Port::Serial(p) => {
                let n = p.bytes_to_read().unwrap_or(0) as usize;
                let mut b = vec![0u8; n];
                match p.read(&mut b) {
                    Ok(k) if n > 0 => {
                        b.truncate(k);
                        b
                    }
                    _ => Vec::new(),
                }
            }
            Port::Sip(m) => m.read(),
        }
    }
    fn carrier(&mut self) -> bool {
        match self {
            Port::Serial(p) => p.read_carrier_detect().unwrap_or(false),
            Port::Sip(m) => m.carrier(),
        }
    }
    /// `Comm1.Settings`: the SIP modem picks its speed from ATS37 instead.
    fn set_baud(&mut self, b: u32) {
        if let Port::Serial(p) = self {
            let _ = p.set_baud_rate(b);
        }
    }
}

struct PortLine<'a>(&'a mut Port);

impl Line for PortLine<'_> {
    fn send(&mut self, data: &[u8]) -> Result<(), String> {
        self.0.write(data).map_err(|e| format!("Lost Carrier ({e})"))
    }
    fn take(&mut self) -> Vec<u8> {
        self.0.read()
    }
    fn hang_up(&mut self) {
        // Module8 sub03B5: "!0", "+++", "ATH0".
        std::thread::sleep(Duration::from_millis(1100));
        let _ = self.0.write(b"+++");
        std::thread::sleep(Duration::from_millis(1200));
        let _ = self.0.write(b"ATH0\r");
        std::thread::sleep(Duration::from_millis(300));
    }
}

fn worker(rx: Receiver<Cmd>, sh: Arc<Mutex<Shared>>) {
    let mut port: Option<Port> = None;
    let mut choice: Option<LineChoice> = None;
    let mut baud = 1200u32;
    let mut call: Option<(Target, String)> = None;
    loop {
        match rx.recv_timeout(Duration::from_millis(25)) {
            Ok(Cmd::Open(c)) => {
                drop(port.take()); // close the old line first (a COM port can only be open once)
                let r = Port::open(&c, baud);
                let mut s = sh.lock().unwrap();
                s.open = r.is_ok();
                s.line = c.label();
                s.open_error = r.as_ref().err().cloned();
                port = r.ok();
                choice = Some(c);
            }
            Ok(Cmd::Close) => {
                port = None;
                sh.lock().unwrap().open = false;
            }
            Ok(Cmd::Send(b)) => {
                if let Some(p) = port.as_mut() {
                    let _ = p.write(&b);
                }
            }
            Ok(Cmd::Baud(b)) => {
                baud = b;
                if let Some(p) = port.as_mut() {
                    p.set_baud(b);
                }
            }
            Ok(Cmd::HangUp) => {
                if let Some(p) = port.as_mut() {
                    let mut l = PortLine(p);
                    let _ = l.send(b"!0");
                    std::thread::sleep(Duration::from_millis(500));
                    l.hang_up();
                }
                call = None;
                sh.lock().unwrap().call = None;
            }
            Ok(Cmd::Dial(t, conf, job)) => {
                sh.lock().unwrap().busy = true;
                if port.is_none() {
                    if let Some(c) = &choice {
                        port = Port::open(c, baud).ok();
                    }
                }
                let r = match port.as_mut() {
                    None => Err(format!("That serial port is in use. ({})", choice.as_ref().map(|c| c.label()).unwrap_or_default())),
                    Some(p) => dial(p, &t, &conf, &job),
                };
                if let Ok(Res::Connected { firmware }) = &r {
                    call = Some((t.clone(), firmware.clone()));
                }
                sh.lock().unwrap().call = call.clone();
                sh.lock().unwrap().busy = false;
                finish(&job, r);
            }
            Ok(Cmd::Run(op, job)) => {
                sh.lock().unwrap().busy = true;
                let r = match (port.as_mut(), &call) {
                    (Some(p), Some((t, fw))) => {
                        let r = run_op(p, Map { d3: t.d3 }, fw, &op, &job);
                        if matches!(&r, Err(e) if e.starts_with("Lost Carrier")) {
                            call = None;
                        }
                        r
                    }
                    _ => Err("Not Connected To A Phone".into()),
                };
                sh.lock().unwrap().call = call.clone();
                sh.lock().unwrap().busy = false;
                finish(&job, r);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if let Some(p) = port.as_mut() {
                    let cd = p.carrier();
                    let got: String = p.read().iter().map(|&c| c as char).collect();
                    let mut s = sh.lock().unwrap();
                    // The phone hung up (it does after about a minute without commands): the
                    // modem reports NO CARRIER and drops carrier detect.
                    if call.is_some() && (got.contains("NO CARRIER") || (s.cd && !cd)) {
                        call = None;
                        s.call = None;
                    }
                    s.cd = cd;
                    s.rx.push_str(&got);
                    if s.rx.len() > 8192 {
                        let cut = s.rx.len() - 8192;
                        s.rx.drain(..cut);
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => return,
        }
    }
}

fn finish(job: &Job, r: Result<Res, String>) {
    let mut s = job.state.lock().unwrap();
    s.status = match &r {
        Ok(_) => "Successful Transfer".into(),
        Err(e) => e.clone(),
    };
    s.outcome = Some(r);
    s.running = false;
}

fn progress_of(job: &Job) -> impl FnMut(&str) + '_ {
    move |m: &str| {
        let mut s = job.state.lock().unwrap();
        s.status = m.to_string();
        s.lines.push(format!("{}  {m}", chrono::Local::now().format("%H:%M:%S")));
    }
}

/// frmConnect: dial, wait for CONNECT, send the ID, then read the firmware version and the
/// status word, as the recorded calls of the original show.
fn dial(p: &mut Port, t: &Target, conf: &DialConf, job: &Job) -> Result<Res, String> {
    p.set_baud(if t.d3 { 1200 } else { 300 });
    let mut number = String::new();
    if !conf.pbx.trim().is_empty() {
        number.push_str(conf.pbx.trim());
        number.push(',');
    }
    if !conf.dcw.trim().is_empty() {
        number.push('*');
        number.push_str(conf.dcw.trim());
        number.push(',');
    }
    number.push_str(&t.dial);
    let setup = proto::DialSetup {
        number,
        init_addons: conf.init_addons.clone(),
        type_addons: if t.d3 { conf.d3_addons.clone() } else { conf.d1_addons.clone() },
        dial_timeout: conf.timeout,
    };
    let mut line = PortLine(p);
    let mut prog = progress_of(job);
    let c = job.cancel.clone();
    let cancel = move || c.load(Ordering::SeqCst);
    let mut s = Session::new(&mut line, &mut prog, &cancel);
    if let Err(e) = s.dial(&setup) {
        s.line.hang_up();
        return Err(e);
    }
    let r = (|| -> Result<String, String> {
        s.log_in(&t.id)?;
        let fw = proto::firmware_text(&s.read(proto::FIRMWARE_ADDR, proto::FIRMWARE_LEN)?);
        let map = Map { d3: t.d3 };
        let _status = s.read(map.status(), 2)?;
        Ok(fw)
    })();
    match r {
        Ok(fw) => Ok(Res::Connected { firmware: fw }),
        Err(e) => {
            s.finish();
            Err(e)
        }
    }
}

fn chunks(addr: u16, data: &[u8], size: usize) -> Vec<(u16, Vec<u8>)> {
    data.chunks(size).enumerate().map(|(i, c)| (addr.wrapping_add((i * size) as u16), c.to_vec())).collect()
}

fn run_op(p: &mut Port, map: Map, fw: &str, op: &Op, job: &Job) -> Result<Res, String> {
    let mut line = PortLine(p);
    let mut prog = progress_of(job);
    let c = job.cancel.clone();
    let cancel = move || c.load(Ordering::SeqCst);
    let mut s = Session::new(&mut line, &mut prog, &cancel);
    let _ = fw;
    let t0 = Instant::now();
    let r = match op {
        Op::ReadSpeedDial => {
            let b = s.read_area(map.speed_dial(), proto::SPEED_DIAL_DIGITS * proto::SPEED_DIAL_COUNT)?;
            Res::SpeedDial(proto::speed_dial_from_bytes(&b))
        }
        Op::WriteSpeedDial(n) => {
            let b = proto::speed_dial_to_bytes(n)?;
            s.write_area(map.speed_dial(), &b)?;
            Res::Done
        }
        Op::ReadAreas(areas) => {
            let mut out = Vec::new();
            let total: usize = areas.iter().map(|x| x.1).sum::<usize>().max(1);
            let mut done = 0;
            for (a, n) in areas {
                (s.progress)(&format!("Uploading {a:04X}... {}%", done * 100 / total));
                out.push((*a, s.read_area(*a, *n)?));
                done += n;
            }
            (s.progress)("Uploading... 100%");
            Res::Areas(out)
        }
        Op::WriteRecords(recs) => {
            write_progress(&mut s, recs)?;
            Res::Done
        }
        Op::ReadClock => {
            let b = s.read(proto::CLOCK_ADDR, proto::CLOCK_LEN)?;
            Res::Clock(Clock::from_bytes(&b).ok_or("Checksum Error")?)
        }
        Op::WriteClock(c) => {
            s.set_clock(c)?;
            Res::Done
        }
        Op::ReadScroll => {
            let b = s.read_area(proto::SCROLL_ADDR, proto::SCROLL_READ_LEN)?;
            Res::Scroll(proto::scroll_text(&b))
        }
        Op::WriteScroll(text) => {
            let b = proto::scroll_bytes(text)?;
            write_progress(&mut s, &chunks(proto::SCROLL_ADDR, &b, proto::RECORD_MAX))?;
            Res::Done
        }
        Op::ReadFirmware => {
            let primary = proto::firmware_text(&s.read(proto::PRIMARY_FW_ADDR, 6)?);
            let secondary = proto::firmware_text(&s.read(proto::SECONDARY_FW_ADDR, 6)?);
            let running = proto::firmware_text(&s.read(proto::FIRMWARE_ADDR, proto::FIRMWARE_LEN)?);
            Res::Firmware { primary, secondary, running }
        }
        Op::ReadCollection => {
            let coins = s.read(map.coins(), 6)?;
            let refunds = s.read(map.refunds(), 5)?;
            let (a, n) = map.collection();
            let raw = s.read_area(a, n)?;
            Res::Collection(Collection {
                coin_box_cents: proto::bcd_cents(&coins[0..3]),
                cumulative_cents: proto::bcd_cents(&coins[3..6]),
                refund_calls: proto::bcd_cents(&refunds[0..2]),
                refund_cents: proto::bcd_cents(&refunds[2..5]),
                raw,
            })
        }
        Op::ClearCounters => {
            // Module1 sub0366.
            let (a, n) = map.collection();
            let mut recs = chunks(a, &vec![0u8; n], proto::BLOCK);
            recs.extend(chunks(map.refunds(), &[0u8; 5], proto::BLOCK));
            write_progress(&mut s, &recs)?;
            Res::Done
        }
        Op::ReadCsn => {
            let (a, max) = map.csn_read();
            let mut out = Vec::new();
            let mut addr = a as usize;
            'outer: for _ in 0..(max / 4 + 1) {
                if out.len() >= max {
                    break;
                }
                (s.progress)(&format!("Uploading CSN entry {} of up to {max}... {}%", out.len() + 1, out.len() * 100 / max.max(1)));
                let block = s.read_area(addr as u16, proto::CSN_ENTRY * 4)?;
                for e in block.chunks(proto::CSN_ENTRY) {
                    if e[0] == 0xFF || out.len() >= max {
                        break 'outer;
                    }
                    if let Some(x) = proto::csn_from_bytes(e) {
                        out.push(x);
                    }
                }
                addr += proto::CSN_ENTRY * 4;
            }
            Res::Csn(out)
        }
        Op::WriteCsn(entries) => {
            let mut b: Vec<u8> = entries.iter().flatten().copied().collect();
            b.push(0xFF);
            write_progress(&mut s, &chunks(map.csn_write(), &b, proto::BLOCK))?;
            Res::Done
        }
        Op::WriteAos(n, b) => {
            if !map.d3 {
                return Err("Program Error (AOS scripts are for D3 phones)".into());
            }
            write_progress(&mut s, &chunks(proto::aos_addr(*n), b, proto::BLOCK))?;
            Res::Done
        }
        Op::WriteRate(b) => {
            s.write_records_in("!1", &chunks(proto::RATE_ADDR, b, proto::RECORD_MAX), Some(("Downloading", 0, b.len().div_ceil(proto::RECORD_MAX))))?;
            Res::Done
        }
        Op::ReadRate => {
            let total = 8192usize;
            let mut out = Vec::with_capacity(total);
            while out.len() < total {
                (s.progress)(&format!("Uploading... {}%", out.len() * 100 / total));
                let n = (total - out.len()).min(proto::READ_MAX);
                out.extend(s.read(proto::RATE_ADDR + out.len() as u16, n)?);
            }
            Res::Rate(out)
        }
        Op::ReadSmdr => {
            let n = proto::smdr_count(&s.read(proto::SMDR_COUNT_ADDR, 8)?);
            (s.progress)(&format!("{n} calls stored."));
            let mut out = Vec::new();
            let mut k = 0;
            while k < n {
                (s.progress)(&format!("Uploading call {} of {n}... {}%", k + 1, k * 100 / n));
                let take = (n - k).min(3);
                let b = s.read(map.smdr() + (k * proto::SMDR_RECORD) as u16, take * proto::SMDR_RECORD)?;
                for r in b.chunks(proto::SMDR_RECORD) {
                    out.push(proto::smdr_record_hex(r, !map.d3));
                }
                k += take;
            }
            Res::Smdr(out)
        }
        Op::ClearSmdr => {
            s.command("!7")?;
            s.command(":00")?;
            Res::Done
        }
        Op::DownloadFirmware(plan, run_secondary) => {
            if !map.d3 {
                return Err("Wrong Firmware Version".into());
            }
            let all = plan.records();
            s.write_records_in("!8", &plan.first, Some(("Downloading", 0, all)))?;
            s.write_records_in("!9", &plan.second, Some(("Downloading", plan.first.len(), all)))?;
            s.write_records(&[(proto::RUN_SELECT_ADDR, vec![if *run_secondary { 0x55 } else { 0x00 }])])?;
            Res::Done
        }
    };
    let _ = t0;
    Ok(r)
}

/// Write records after one `!1`, showing "Downloading... n%" as each record is acknowledged.
fn write_progress(s: &mut Session, recs: &[(u16, Vec<u8>)]) -> Result<(), String> {
    s.write_records_in("!1", recs, Some(("Downloading", 0, recs.len())))?;
    (s.progress)("Downloading... 100%");
    Ok(())
}

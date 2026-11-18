//! A built-in modem that places its calls over SIP instead of a phone line.
//!
//! It behaves like the Hayes modem on a COM port as far as the rest of the program can tell:
//! it takes AT commands (`ATZ`, `ATS37=5`, `ATDT<number>`, `ATH0`...), answers `OK`, and on
//! `ATD` makes a SIP call (G.711 u-law audio), carries real modem tones over it with the
//! software modem in `softmodem` (Bell 212A at 1200 bit/s or Bell 103 at 300, characters 7E1)
//! and reports `CONNECT 1200`, `BUSY` or `NO CARRIER`. In a call, bytes pass straight through
//! until `+++` brings it back to commands.
//!
//! Only outgoing calls are supported. The provider must accept calls from the account without
//! a registration (most do; they challenge each INVITE with a digest instead).

pub mod msg;
pub mod softmodem;

use crate::text::Profile;
use msg::{Challenge, RequestSpec, SipMsg, build_response, build_sdp, new_branch, param, parse_sdp, random_hex, tag_of, uri_of};
use softmodem::{CharFormat, Event, SoftModem, Speed, linear_to_ulaw};
use std::net::{IpAddr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// The SIP account to call from, kept in TFW.INI under `[SipModem]`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SipConfig {
    pub server: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

impl SipConfig {
    pub fn load(p: &Profile) -> SipConfig {
        SipConfig {
            server: p.get("SipModem", "Server"),
            port: p.get("SipModem", "Port").trim().parse().unwrap_or(5060),
            user: p.get("SipModem", "User"),
            password: p.get("SipModem", "Password"),
        }
    }
    pub fn save(&self, p: &Profile) -> bool {
        p.set("SipModem", "Server", self.server.trim())
            & p.set("SipModem", "Port", &self.port.to_string())
            & p.set("SipModem", "User", self.user.trim())
            & p.set("SipModem", "Password", &self.password)
    }
}

#[derive(Default)]
struct Shared {
    /// Bytes from the program, not yet handled.
    input: Vec<u8>,
    /// Bytes for the program (command results, or data from the far end).
    output: Vec<u8>,
    carrier: bool,
    stop: bool,
}

/// The modem as the rest of the program sees it: write bytes, read bytes, carrier detect.
pub struct SipModem {
    sh: Arc<Mutex<Shared>>,
}

impl SipModem {
    pub fn start(cfg: SipConfig) -> SipModem {
        let sh = Arc::new(Mutex::new(Shared::default()));
        let s2 = sh.clone();
        std::thread::spawn(move || Hayes::new(cfg, s2).run());
        SipModem { sh }
    }
    pub fn write(&self, b: &[u8]) {
        self.sh.lock().unwrap().input.extend_from_slice(b);
    }
    pub fn read(&self) -> Vec<u8> {
        std::mem::take(&mut self.sh.lock().unwrap().output)
    }
    pub fn carrier(&self) -> bool {
        self.sh.lock().unwrap().carrier
    }
}

impl Drop for SipModem {
    fn drop(&mut self) {
        self.sh.lock().unwrap().stop = true;
    }
}

// ------------------------------------------------------------------------- AT commands

struct Hayes {
    cfg: SipConfig,
    sh: Arc<Mutex<Shared>>,
    line: Vec<u8>,
    /// S7: seconds to wait for the call to connect.
    s7: u64,
    /// S37: 5 = 1200 bit/s, 3 = 300 bit/s (as the ATS37 addons in TFW.INI pick).
    speed: Speed,
    data_mode: bool,
    call: Option<Call>,
}

impl Hayes {
    fn new(cfg: SipConfig, sh: Arc<Mutex<Shared>>) -> Hayes {
        Hayes { cfg, sh, line: Vec::new(), s7: 60, speed: Speed::Auto, data_mode: false, call: None }
    }

    fn say(&self, text: &str) {
        self.sh.lock().unwrap().output.extend_from_slice(format!("\r\n{text}\r\n").as_bytes());
    }

    fn run(mut self) {
        loop {
            let (input, stop) = {
                let mut s = self.sh.lock().unwrap();
                (std::mem::take(&mut s.input), s.stop)
            };
            if stop {
                self.hang_up();
                return;
            }
            if !input.is_empty() {
                self.take_input(&input);
            }
            if let Some(call) = self.call.as_mut() {
                match call.poll() {
                    Ok(Some(Event::Connected(speed))) => {
                        self.data_mode = true;
                        self.sh.lock().unwrap().carrier = true;
                        self.say(&format!("CONNECT {speed}"));
                    }
                    Ok(Some(_)) | Ok(None) => {}
                    Err(result) => {
                        self.call = None;
                        self.data_mode = false;
                        self.sh.lock().unwrap().carrier = false;
                        self.say(&result);
                    }
                }
                if let Some(call) = self.call.as_mut() {
                    let got = std::mem::take(&mut call.received);
                    if !got.is_empty() {
                        self.sh.lock().unwrap().output.extend(got);
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    fn take_input(&mut self, input: &[u8]) {
        if self.data_mode {
            // The escape: "+++" on its own (the program sends it after a pause).
            if input == b"+++" {
                self.data_mode = false;
                self.say("OK");
            } else if let Some(c) = self.call.as_mut() {
                c.modem.send(input);
            }
            return;
        }
        for &b in input {
            match b {
                b'\r' | b'\n' => {
                    let line = String::from_utf8_lossy(&std::mem::take(&mut self.line)).trim().to_ascii_uppercase();
                    if !line.is_empty() {
                        self.command(&line);
                    }
                }
                8 => {
                    self.line.pop();
                }
                _ => self.line.push(b),
            }
        }
    }

    /// One command line (`AT...`).
    fn command(&mut self, line: &str) {
        let Some(rest) = line.strip_prefix("AT") else {
            return self.say("ERROR");
        };
        let c: Vec<char> = rest.chars().collect();
        let mut i = 0;
        let number = |from: usize| -> (u32, usize) {
            let mut j = from;
            while j < c.len() && c[j].is_ascii_digit() {
                j += 1;
            }
            (c[from..j].iter().collect::<String>().parse().unwrap_or(0), j)
        };
        while i < c.len() {
            let ch = c[i];
            i += 1;
            match ch {
                'D' => {
                    let target: String = c[i..].iter().collect::<String>().trim_start_matches(['T', 'P']).to_string();
                    return self.dial(target.trim());
                }
                'H' => {
                    self.hang_up();
                    i = number(i).1;
                }
                'Z' => {
                    self.hang_up();
                    self.s7 = 60;
                    self.speed = Speed::Auto;
                    i = number(i).1;
                }
                'O' => {
                    return if self.call.as_ref().is_some_and(|c| c.modem.connected().is_some()) {
                        self.data_mode = true;
                        self.say("CONNECT")
                    } else {
                        self.say("NO CARRIER")
                    };
                }
                'S' => {
                    let (reg, j) = number(i);
                    i = j;
                    if c.get(i) == Some(&'=') {
                        let (v, j) = number(i + 1);
                        i = j;
                        match reg {
                            7 => self.s7 = v.clamp(5, 255) as u64,
                            37 => self.speed = if v <= 3 { Speed::B300 } else { Speed::B1200 },
                            _ => {}
                        }
                    }
                }
                '&' => i = number(i + 1).1,
                c if c.is_ascii_alphabetic() => i = number(i).1,
                _ => {}
            }
        }
        self.say("OK");
    }

    fn dial(&mut self, target: &str) {
        self.hang_up();
        let target: String = target.chars().filter(|c| !matches!(c, ' ' | '-' | '(' | ')' | ',' | ';')).collect();
        if target.is_empty() {
            return self.say("ERROR");
        }
        match Call::start(&self.cfg, &target, self.speed, Duration::from_secs(self.s7)) {
            Ok(c) => self.call = Some(c),
            Err(e) => {
                eprintln!("SIP modem: {e}");
                self.say("NO CARRIER");
            }
        }
    }

    fn hang_up(&mut self) {
        if let Some(mut c) = self.call.take() {
            c.end();
        }
        self.data_mode = false;
        self.sh.lock().unwrap().carrier = false;
    }
}

// ------------------------------------------------------------------------- the SIP call

#[derive(PartialEq)]
enum CallState {
    /// INVITE sent, waiting for the answer.
    Inviting,
    /// Answered; audio flowing.
    Up,
}

struct Call {
    cfg: SipConfig,
    sip: UdpSocket,
    rtp: UdpSocket,
    host: String,
    /// The address others see us at (learned from the server's `received`/`rport`).
    public: (String, u16),
    invite: RequestSpec,
    branch: String,
    to_tag: Option<String>,
    remote_target: String,
    routes: Vec<String>,
    remote_rtp: Option<SocketAddr>,
    state: CallState,
    authorized: bool,
    started: Instant,
    timeout: Duration,
    modem: SoftModem,
    next_frame: Instant,
    seq: u16,
    stamp: u32,
    ssrc: u32,
    received: Vec<u8>,
}

impl Call {
    fn start(cfg: &SipConfig, number: &str, speed: Speed, timeout: Duration) -> Result<Call, String> {
        if cfg.server.trim().is_empty() || cfg.user.trim().is_empty() {
            return Err("no SIP account set up (System Configuration)".into());
        }
        let host = cfg.server.trim().to_string();
        let server = (host.as_str(), cfg.port)
            .to_socket_addrs()
            .map_err(|e| format!("{host}: {e}"))?
            .find(|a| a.is_ipv4())
            .ok_or(format!("{host}: no address"))?;
        let sip = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
        sip.connect(server).map_err(|e| e.to_string())?;
        let rtp = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
        sip.set_nonblocking(true).map_err(|e| e.to_string())?;
        rtp.set_nonblocking(true).map_err(|e| e.to_string())?;
        let local = sip.local_addr().map_err(|e| e.to_string())?;
        let ip = match local.ip() {
            IpAddr::V4(v) => v.to_string(),
            other => other.to_string(),
        };
        let uri = if number.starts_with("sip:") { number.to_string() } else { format!("sip:{number}@{host}") };
        let user = cfg.user.trim();
        let invite = RequestSpec {
            method: "INVITE".into(),
            uri: uri.clone(),
            from: format!("<sip:{user}@{host}>;tag={}", random_hex(10)),
            to: format!("<{uri}>"),
            call_id: format!("{}@{ip}", random_hex(16)),
            cseq: 1,
            contact: Some(format!("<sip:{user}@{ip}:{}>", local.port())),
            extra: Vec::new(),
            content_type: Some("application/sdp".into()),
            body: Vec::new(),
            auth: None,
        };
        let rtp_port = rtp.local_addr().map_err(|e| e.to_string())?.port();
        let mut call = Call {
            cfg: cfg.clone(),
            sip,
            rtp,
            host,
            public: (ip, local.port()),
            invite,
            branch: new_branch(),
            to_tag: None,
            remote_target: uri,
            routes: Vec::new(),
            remote_rtp: None,
            state: CallState::Inviting,
            authorized: false,
            started: Instant::now(),
            timeout,
            modem: SoftModem::new(true, speed, CharFormat::E7),
            next_frame: Instant::now(),
            seq: 0,
            stamp: 0,
            ssrc: u32::from_str_radix(&random_hex(8), 16).unwrap_or(1),
            received: Vec::new(),
        };
        call.invite.body = build_sdp(&call.public.0, rtp_port, 0, false, 1);
        call.send_invite();
        Ok(call)
    }

    fn send(&self, bytes: &[u8]) {
        let _ = self.sip.send(bytes);
    }

    fn send_invite(&mut self) {
        let b = self.invite.render(&self.public.0, self.public.1, &self.branch);
        self.send(&b);
    }

    /// ACK for a final answer to our INVITE (in the INVITE's transaction for failures).
    fn send_ack(&self, resp: &SipMsg, success: bool) {
        let mut ack = self.invite.clone();
        ack.method = "ACK".into();
        ack.to = resp.header("to").unwrap_or(&self.invite.to).to_string();
        ack.body.clear();
        ack.content_type = None;
        let branch = if success {
            ack.uri = self.remote_target.clone();
            ack.extra = self.routes.iter().map(|r| ("Route".to_string(), r.clone())).collect();
            new_branch()
        } else {
            self.branch.clone()
        };
        let b = ack.render(&self.public.0, self.public.1, &branch);
        self.send(&b);
    }

    /// Hang up: BYE when answered, CANCEL while ringing.
    fn end(&mut self) {
        let mut req = self.invite.clone();
        req.body.clear();
        req.content_type = None;
        match self.state {
            CallState::Up => {
                req.method = "BYE".into();
                req.uri = self.remote_target.clone();
                req.cseq += 1;
                if let Some(t) = &self.to_tag {
                    req.to = format!("{};tag={t}", self.invite.to);
                }
                req.extra = self.routes.iter().map(|r| ("Route".to_string(), r.clone())).collect();
                let b = req.render(&self.public.0, self.public.1, &new_branch());
                self.send(&b);
            }
            CallState::Inviting => {
                req.method = "CANCEL".into();
                let b = req.render(&self.public.0, self.public.1, &self.branch);
                self.send(&b);
            }
        }
    }

    /// Handle SIP and audio. `Err(result code)` ends the call.
    fn poll(&mut self) -> Result<Option<Event>, String> {
        let mut buf = [0u8; 4096];
        while let Ok(n) = self.sip.recv(&mut buf) {
            if let Some(m) = SipMsg::parse(&buf[..n]) {
                self.on_sip(&m)?;
            }
        }
        if self.state == CallState::Inviting && self.started.elapsed() > self.timeout {
            self.end();
            return Err("NO CARRIER".into());
        }
        if self.state != CallState::Up {
            return Ok(None);
        }
        // Audio in.
        while let Ok((n, from)) = self.rtp.recv_from(&mut buf) {
            if n <= 12 || buf[1] & 0x7F != 0 {
                continue;
            }
            // Symmetric RTP: answer where the audio comes from (helps behind NAT).
            self.remote_rtp = Some(from);
            let samples: Vec<i16> = buf[12..n].iter().map(|&u| ulaw_to_linear(u)).collect();
            let bytes = self.modem.receive(&samples);
            self.received.extend(bytes);
        }
        // Audio out: a 20 ms frame every 20 ms.
        let now = Instant::now();
        if now >= self.next_frame {
            if now - self.next_frame > Duration::from_millis(200) {
                self.next_frame = now;
            }
            self.next_frame += Duration::from_millis(20);
            let samples = self.modem.generate(160);
            let mut pkt = Vec::with_capacity(172);
            pkt.extend_from_slice(&[0x80, 0]);
            pkt.extend_from_slice(&self.seq.to_be_bytes());
            pkt.extend_from_slice(&self.stamp.to_be_bytes());
            pkt.extend_from_slice(&self.ssrc.to_be_bytes());
            pkt.extend(samples.iter().map(|&s| linear_to_ulaw(s)));
            self.seq = self.seq.wrapping_add(1);
            self.stamp = self.stamp.wrapping_add(160);
            if let Some(to) = self.remote_rtp {
                let _ = self.rtp.send_to(&pkt, to);
            }
        }
        for e in self.modem.take_events() {
            match e {
                Event::Connected(_) => return Ok(Some(e)),
                Event::Failed(_) | Event::CarrierLost => {
                    self.end();
                    return Err("NO CARRIER".into());
                }
                Event::Status(_) => {}
            }
        }
        Ok(None)
    }

    fn on_sip(&mut self, m: &SipMsg) -> Result<(), String> {
        if m.is_request() {
            // The far end hung up, or the server checks on us.
            let to_tag = self.invite.from.split("tag=").nth(1).map(|s| s.to_string());
            let ok = build_response(m, 200, "OK", to_tag.as_deref(), &[], None, &[]);
            self.send(&ok);
            return match m.method().as_str() {
                "BYE" | "CANCEL" => Err("NO CARRIER".into()),
                _ => Ok(()),
            };
        }
        if m.method() != "INVITE" || m.call_id() != self.invite.call_id {
            return Ok(());
        }
        let status = m.status();
        // Learn the address the server sees us at, for Contact and SDP.
        if let Some(via) = m.header("via") {
            if let (Some(ip), Some(port)) = (param(via, "received"), param(via, "rport").and_then(|p| p.parse().ok())) {
                self.public = (ip, port);
            }
        }
        match status {
            100..=199 => Ok(()),
            200..=299 => {
                if self.state == CallState::Up {
                    self.send_ack(m, true); // a repeated 200 OK
                    return Ok(());
                }
                self.to_tag = m.header("to").and_then(tag_of);
                if let Some(c) = m.header("contact") {
                    self.remote_target = uri_of(c);
                }
                self.routes = m.header_all("record-route").into_iter().rev().collect();
                if let Some(sdp) = parse_sdp(&m.body) {
                    self.remote_rtp = format!("{}:{}", sdp.addr, sdp.port).parse().ok();
                }
                self.send_ack(m, true);
                self.state = CallState::Up;
                self.next_frame = Instant::now();
                Ok(())
            }
            401 | 407 if !self.authorized => {
                self.send_ack(m, false);
                let mut ch = Challenge::from_response(m).ok_or("NO CARRIER")?;
                self.authorized = true;
                self.invite.auth = Some(ch.answer(self.cfg.user.trim(), &self.cfg.password, "INVITE", &self.invite.uri));
                self.invite.cseq += 1;
                let user = self.cfg.user.trim().to_string();
                self.invite.contact = Some(format!("<sip:{user}@{}:{}>", self.public.0, self.public.1));
                let rtp_port = self.rtp.local_addr().map(|a| a.port()).unwrap_or(0);
                self.invite.body = build_sdp(&self.public.0, rtp_port, 0, false, 2);
                self.branch = new_branch();
                self.send_invite();
                Ok(())
            }
            _ => {
                self.send_ack(m, false);
                eprintln!("SIP modem: call to {} failed: {} {} ({})", self.host, status, m.reason(), self.invite.uri);
                Err(if matches!(status, 486 | 600) { "BUSY".into() } else { "NO CARRIER".into() })
            }
        }
    }
}

/// A G.711 u-law byte to a 16-bit sample.
fn ulaw_to_linear(u: u8) -> i16 {
    let u = !u;
    let sign = u & 0x80;
    let exp = (u >> 4) & 0x07;
    let mant = (u & 0x0F) as i32;
    let mag = (((mant << 3) + 0x84) << exp) - 0x84;
    (if sign != 0 { -mag } else { mag }) as i16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ulaw_round_trip() {
        for s in [-30000i16, -1000, -8, 0, 8, 1000, 30000] {
            let back = ulaw_to_linear(linear_to_ulaw(s));
            assert!((back as i32 - s as i32).abs() <= (s as i32).abs() / 16 + 8, "{s} -> {back}");
        }
    }

    #[test]
    fn answers_at_commands_without_a_call() {
        let m = SipModem::start(SipConfig::default());
        m.write(b"ATZ\rATE1Q0V1S37=5L3\r");
        std::thread::sleep(Duration::from_millis(100));
        let t = String::from_utf8(m.read()).unwrap();
        assert_eq!(t.matches("OK").count(), 2);
        m.write(b"ATDT5550101234\r");
        std::thread::sleep(Duration::from_millis(100));
        assert!(String::from_utf8(m.read()).unwrap().contains("NO CARRIER"), "no account set up");
    }
}

//! A software modem for 8 kHz phone audio: Bell 103 (300 bit/s FSK) and Bell 212A
//! (1200 bit/s DPSK), in originate (we dialed) or answer (we were called) mode, with the
//! start-up handshake. Bytes go in and out as asynchronous characters (8N1, 7E1, 7O1, 7N1).


use std::collections::VecDeque;
use std::f64::consts::PI;

pub const RATE: f64 = 8000.0;

fn ms(x: u64) -> u64 {
    x * 8
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Speed {
    /// Try 1200 bit/s, fall back to 300.
    Auto,
    /// Bell 212A only.
    B1200,
    /// Bell 103 only.
    B300,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CharFormat {
    /// 8 data bits, no parity.
    N8,
    /// 7 data bits, even parity (payphones).
    E7,
}

impl CharFormat {
    fn data_bits(self) -> u32 {
        if self == CharFormat::N8 { 8 } else { 7 }
    }
    fn parity(self) -> Option<bool> {
        match self {
            CharFormat::E7 => Some(false),
            CharFormat::N8 => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    /// Trained up: the link speed in bit/s.
    Connected(u32),
    /// The handshake failed.
    Failed(String),
    /// The far end's carrier went away.
    CarrierLost,
    /// Progress to show the person.
    Status(String),
}

// ------------------------------------------------------------------ building blocks

#[derive(Clone)]
struct Biquad {
    b0: f64,
    b1: f64,
    b2: f64,
    a1: f64,
    a2: f64,
    z1: f64,
    z2: f64,
}

impl Biquad {
    fn new(kind: char, f: f64, q: f64) -> Self {
        let w0 = 2.0 * PI * f / RATE;
        let (c, s) = (w0.cos(), w0.sin());
        let alpha = s / (2.0 * q);
        let (b0, b1, b2) = if kind == 'l' {
            ((1.0 - c) / 2.0, 1.0 - c, (1.0 - c) / 2.0)
        } else {
            ((1.0 + c) / 2.0, -(1.0 + c), (1.0 + c) / 2.0)
        };
        let a0 = 1.0 + alpha;
        Biquad { b0: b0 / a0, b1: b1 / a0, b2: b2 / a0, a1: -2.0 * c / a0, a2: (1.0 - alpha) / a0, z1: 0.0, z2: 0.0 }
    }
    fn run(&mut self, x: f64) -> f64 {
        let y = self.b0 * x + self.z1;
        self.z1 = self.b1 * x - self.a1 * y + self.z2;
        self.z2 = self.b2 * x - self.a2 * y;
        y
    }
}

/// Sliding single-frequency correlator: how much of the signal is a tone at `f`.
struct ToneDet {
    w: f64,
    phase: f64,
    n: usize,
    buf: VecDeque<(f64, f64, f64)>,
    si: f64,
    sq: f64,
    sp: f64,
}

impl ToneDet {
    fn new(f: f64, n: usize) -> Self {
        ToneDet { w: 2.0 * PI * f / RATE, phase: 0.0, n, buf: VecDeque::with_capacity(n + 1), si: 0.0, sq: 0.0, sp: 0.0 }
    }
    fn push(&mut self, x: f64) {
        let (i, q, p) = (x * self.phase.cos(), -x * self.phase.sin(), x * x);
        self.phase = (self.phase + self.w) % (2.0 * PI);
        self.buf.push_back((i, q, p));
        self.si += i;
        self.sq += q;
        self.sp += p;
        if self.buf.len() > self.n {
            let (i, q, p) = self.buf.pop_front().unwrap();
            self.si -= i;
            self.sq -= q;
            self.sp -= p;
        }
    }
    /// 1.0 for a pure tone at the frequency, near 0 for something else.
    fn ratio(&self) -> f64 {
        let n = self.buf.len().max(1) as f64;
        let m2 = (self.si * self.si + self.sq * self.sq) / (n * n);
        let p = self.sp / n;
        if p <= 1e-9 { 0.0 } else { (2.0 * m2 / p).min(1.5) }
    }
}

/// Characters to an asynchronous bit stream.
struct UartTx {
    fmt: CharFormat,
    queue: VecDeque<u8>,
    bits: VecDeque<u8>,
    hold: bool,
}

impl UartTx {
    fn next_bit(&mut self) -> u8 {
        if self.bits.is_empty() && !self.hold {
            if let Some(c) = self.queue.pop_front() {
                let n = self.fmt.data_bits();
                self.bits.push_back(0);
                let mut ones = 0;
                for i in 0..n {
                    let b = (c >> i) & 1;
                    ones += b as u32;
                    self.bits.push_back(b);
                }
                if let Some(odd) = self.fmt.parity() {
                    let even_bit = (ones % 2) as u8; // makes the total even
                    self.bits.push_back(if odd { even_bit ^ 1 } else { even_bit });
                }
                self.bits.push_back(1);
            }
        }
        self.bits.pop_front().unwrap_or(1)
    }
}

/// Asynchronous bit stream (one value per bit) to characters.
struct UartRxBits {
    fmt: CharFormat,
    active: bool,
    acc: u32,
    count: u32,
    last: u8,
}

impl UartRxBits {
    fn new(fmt: CharFormat) -> Self {
        UartRxBits { fmt, active: false, acc: 0, count: 0, last: 1 }
    }
    fn frame_len(&self) -> u32 {
        self.fmt.data_bits() + if self.fmt.parity().is_some() { 1 } else { 0 }
    }
    fn push(&mut self, bit: u8) -> Option<u8> {
        if !self.active {
            if bit == 0 && self.last == 1 {
                self.active = true;
                self.acc = 0;
                self.count = 0;
            }
            self.last = bit;
            return None;
        }
        self.last = bit;
        let need = self.frame_len();
        if self.count < need {
            self.acc |= (bit as u32) << self.count;
            self.count += 1;
            return None;
        }
        // Stop bit.
        self.active = false;
        if bit == 1 {
            let mask = if self.fmt.data_bits() == 8 { 0xFF } else { 0x7F };
            Some((self.acc & mask) as u8)
        } else {
            self.last = 0; // framing error: wait for the line to go idle again
            None
        }
    }
}

/// Asynchronous characters from a per-sample bit decision (for 300 bit/s FSK).
struct UartRxSamples {
    spb: f64,
    bits: UartRxBits,
    active: bool,
    until: f64,
    last: u8,
    sampled_start: bool,
}

impl UartRxSamples {
    fn new(fmt: CharFormat, spb: f64) -> Self {
        UartRxSamples { spb, bits: UartRxBits::new(fmt), active: false, until: 0.0, last: 1, sampled_start: false }
    }
    fn push(&mut self, bit: u8) -> Option<u8> {
        let mut out = None;
        if !self.active {
            if bit == 0 && self.last == 1 {
                self.active = true;
                self.until = self.spb * 0.5;
                self.sampled_start = false;
            }
        } else {
            self.until -= 1.0;
            if self.until <= 0.0 {
                self.until += self.spb;
                if !self.sampled_start {
                    self.sampled_start = true;
                    if bit != 0 {
                        self.active = false; // a glitch, not a start bit
                    } else {
                        self.bits.last = 1;
                        self.bits.push(0);
                    }
                } else {
                    out = self.bits.push(bit);
                    if !self.bits.active {
                        self.active = false;
                    }
                }
            }
        }
        self.last = bit;
        out
    }
}

struct FskTx {
    mark: f64,
    space: f64,
    phase: f64,
    spb: f64,
    left: f64,
    cur: u8,
}

impl FskTx {
    fn new(mark: f64, space: f64, baud: f64) -> Self {
        FskTx { mark, space, phase: 0.0, spb: RATE / baud, left: 0.0, cur: 1 }
    }
    fn sample(&mut self, uart: &mut UartTx) -> f64 {
        if self.left <= 0.0 {
            self.cur = uart.next_bit();
            self.left += self.spb;
        }
        self.left -= 1.0;
        let f = if self.cur == 1 { self.mark } else { self.space };
        self.phase = (self.phase + 2.0 * PI * f / RATE) % (2.0 * PI);
        self.phase.sin()
    }
}

struct FskRx {
    mark: ToneDet,
    space: ToneDet,
}

impl FskRx {
    fn new(mark: f64, space: f64, baud: f64) -> Self {
        let n = (RATE / baud).round() as usize;
        FskRx { mark: ToneDet::new(mark, n), space: ToneDet::new(space, n) }
    }
    fn push(&mut self, x: f64) -> u8 {
        self.mark.push(x);
        self.space.push(x);
        let m = self.mark.si * self.mark.si + self.mark.sq * self.mark.sq;
        let s = self.space.si * self.space.si + self.space.sq * self.space.sq;
        if m >= s { 1 } else { 0 }
    }
}

fn scramble(reg: &mut u32, bit: u8) -> u8 {
    let out = bit ^ ((*reg >> 13) & 1) as u8 ^ ((*reg >> 16) & 1) as u8;
    *reg = ((*reg << 1) | out as u32) & 0x1FFFF;
    out
}

fn descramble(reg: &mut u32, bit: u8) -> u8 {
    let out = bit ^ ((*reg >> 13) & 1) as u8 ^ ((*reg >> 16) & 1) as u8;
    *reg = ((*reg << 1) | bit as u32) & 0x1FFFF;
    out
}

/// Raised-cosine pulse (beta 0.75), time in symbol periods.
fn raised_cosine(t: f64) -> f64 {
    let beta = 0.75;
    let sinc = if t.abs() < 1e-9 { 1.0 } else { (PI * t).sin() / (PI * t) };
    let d = 1.0 - (2.0 * beta * t).powi(2);
    let c = if d.abs() < 1e-6 { PI / 4.0 } else { (PI * beta * t).cos() / d };
    sinc * c
}

const SYMBOL: f64 = RATE / 600.0;

struct DpskTx {
    w: f64,
    cphase: f64,
    n: u64,
    syms: VecDeque<(f64, f64, f64)>,
    next_sym: f64,
    abs_phase: f64,
    scr: u32,
}

impl DpskTx {
    fn new(fc: f64) -> Self {
        DpskTx { w: 2.0 * PI * fc / RATE, cphase: 0.0, n: 0, syms: VecDeque::new(), next_sym: 0.0, abs_phase: 0.0, scr: 0 }
    }
    fn sample(&mut self, uart: &mut UartTx) -> f64 {
        let t = self.n as f64;
        while self.next_sym <= t + 3.0 * SYMBOL {
            let b1 = scramble(&mut self.scr, uart.next_bit());
            let b2 = scramble(&mut self.scr, uart.next_bit());
            let d = match (b1, b2) {
                (0, 0) => 90.0,
                (0, 1) => 0.0,
                (1, 1) => 270.0,
                _ => 180.0,
            };
            self.abs_phase = (self.abs_phase + d) % 360.0;
            let r = self.abs_phase.to_radians();
            self.syms.push_back((self.next_sym, r.cos(), r.sin()));
            self.next_sym += SYMBOL;
        }
        while self.syms.front().map(|s| s.0 < t - 3.0 * SYMBOL).unwrap_or(false) {
            self.syms.pop_front();
        }
        let (mut re, mut im) = (0.0, 0.0);
        for &(st, a, b) in &self.syms {
            let h = raised_cosine((t - st) / SYMBOL);
            re += a * h;
            im += b * h;
        }
        let out = re * self.cphase.cos() - im * self.cphase.sin();
        self.cphase = (self.cphase + self.w) % (2.0 * PI);
        self.n += 1;
        out
    }
}

struct DpskRx {
    w: f64,
    phase: f64,
    fir: Vec<f64>,
    hi: VecDeque<f64>,
    hq: VecDeque<f64>,
    bb: VecDeque<(f64, f64)>,
    base: u64,
    n: u64,
    next: f64,
    prev: Option<(f64, f64)>,
    desc: u32,
    ones_run: u32,
}

impl DpskRx {
    fn new(fc: f64) -> Self {
        let taps = 31;
        let m = (taps - 1) as f64 / 2.0;
        let fcut = 750.0 / RATE;
        let mut fir: Vec<f64> = (0..taps)
            .map(|i| {
                let x = i as f64 - m;
                let s = if x == 0.0 { 2.0 * fcut } else { (2.0 * PI * fcut * x).sin() / (PI * x) };
                s * (0.54 - 0.46 * (2.0 * PI * i as f64 / (taps - 1) as f64).cos())
            })
            .collect();
        let sum: f64 = fir.iter().sum();
        fir.iter_mut().for_each(|v| *v /= sum);
        DpskRx {
            w: 2.0 * PI * fc / RATE,
            phase: 0.0,
            fir,
            hi: VecDeque::new(),
            hq: VecDeque::new(),
            bb: VecDeque::new(),
            base: 0,
            n: 0,
            next: SYMBOL,
            prev: None,
            desc: 0,
            ones_run: 0,
        }
    }

    fn at(&self, t: f64) -> (f64, f64) {
        let i0 = t.floor();
        let f = t - i0;
        let idx = (i0 as u64 - self.base) as usize;
        let a = self.bb[idx];
        let b = self.bb[idx + 1];
        (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f)
    }

    fn push(&mut self, x: f64, bits: &mut Vec<u8>) {
        self.hi.push_back(x * self.phase.cos());
        self.hq.push_back(-x * self.phase.sin());
        self.phase = (self.phase + self.w) % (2.0 * PI);
        if self.hi.len() > self.fir.len() {
            self.hi.pop_front();
            self.hq.pop_front();
        }
        let (mut i, mut q) = (0.0, 0.0);
        for (k, c) in self.fir.iter().enumerate() {
            if let (Some(a), Some(b)) = (self.hi.get(k), self.hq.get(k)) {
                i += a * c;
                q += b * c;
            }
        }
        self.bb.push_back((i, q));
        self.n += 1;
        // Symbol strobes we now have the samples for.
        while self.next.floor() as u64 + 1 < self.n {
            let mid_t = self.next - SYMBOL / 2.0;
            if mid_t < self.base as f64 {
                self.next += SYMBOL;
                continue;
            }
            let y = self.at(self.next);
            let m = self.at(mid_t);
            if let Some(p) = self.prev {
                let dr = y.0 * p.0 + y.1 * p.1;
                let di = y.1 * p.0 - y.0 * p.1;
                let a = di.atan2(dr).to_degrees();
                let (b1, b2) = if a > -45.0 && a <= 45.0 {
                    (0, 1)
                } else if a > 45.0 && a <= 135.0 {
                    (0, 0)
                } else if a > -135.0 && a <= -45.0 {
                    (1, 1)
                } else {
                    (1, 0)
                };
                for b in [b1, b2] {
                    let o = descramble(&mut self.desc, b);
                    self.ones_run = if o == 1 { self.ones_run.saturating_add(1) } else { 0 };
                    bits.push(o);
                }
                // Gardner timing error: positive means we sample late.
                let e = m.0 * (y.0 - p.0) + m.1 * (y.1 - p.1);
                let norm = y.0 * y.0 + y.1 * y.1 + p.0 * p.0 + p.1 * p.1 + 1e-9;
                let en = (e / norm).clamp(-1.0, 1.0);
                self.next += SYMBOL - 0.5 * en;
            } else {
                self.next += SYMBOL;
            }
            self.prev = Some(y);
        }
        // Forget what no strobe will need again.
        let keep_from = (self.next - SYMBOL).floor().max(0.0) as u64;
        while self.base + 2 < keep_from && self.bb.len() > 2 {
            self.bb.pop_front();
            self.base += 1;
        }
    }
}

// ------------------------------------------------------------------ the modem

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum St {
    OrigWait,
    OrigDelay,
    OrigDpsk,
    OrigFsk,
    AnsSilence,
    AnsTone,
    AnsDpsk,
    Connected,
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tx {
    Silent,
    Fsk,
    Dpsk,
}

const LEVEL_MIN: f64 = 250.0 * 250.0 / 2.0;
const AMPLITUDE: f64 = 6000.0;

pub struct SoftModem {
    originate: bool,
    pref: Speed,
    st: St,
    in_state: u64,
    uart_tx: UartTx,
    tx: Tx,
    fsk_tx: FskTx,
    dpsk_tx: DpskTx,
    band: Vec<Biquad>,
    level: f64,
    tone: ToneDet,
    ans2100: ToneDet,
    fsk_rx: FskRx,
    uart_rx_s: UartRxSamples,
    dpsk_rx: DpskRx,
    uart_rx_b: UartRxBits,
    tone_run: u64,
    quiet_run: u64,
    speed: u32,
    events: Vec<Event>,
    said_2100: bool,
    bitbuf: Vec<u8>,
}

impl SoftModem {
    pub fn new(originate: bool, pref: Speed, fmt: CharFormat) -> Self {
        // Originate sends in the low band and listens in the high band; answer is the reverse.
        let (tx_mark, tx_space, rx_mark, rx_space, tx_c, rx_c) = if originate {
            (1270.0, 1070.0, 2225.0, 2025.0, 1200.0, 2400.0)
        } else {
            (2225.0, 2025.0, 1270.0, 1070.0, 2400.0, 1200.0)
        };
        let band = if originate {
            vec![Biquad::new('h', 1600.0, 0.707), Biquad::new('h', 1600.0, 0.707), Biquad::new('l', 3000.0, 0.707)]
        } else {
            vec![Biquad::new('l', 1900.0, 0.707), Biquad::new('l', 1900.0, 0.707), Biquad::new('h', 500.0, 0.707)]
        };
        SoftModem {
            originate,
            pref,
            st: if originate { St::OrigWait } else { St::AnsSilence },
            in_state: 0,
            uart_tx: UartTx { fmt, queue: VecDeque::new(), bits: VecDeque::new(), hold: true },
            tx: Tx::Silent,
            fsk_tx: FskTx::new(tx_mark, tx_space, 300.0),
            dpsk_tx: DpskTx::new(tx_c),
            band,
            level: 0.0,
            tone: ToneDet::new(rx_mark, 160),
            ans2100: ToneDet::new(2100.0, 160),
            fsk_rx: FskRx::new(rx_mark, rx_space, 300.0),
            uart_rx_s: UartRxSamples::new(fmt, RATE / 300.0),
            dpsk_rx: DpskRx::new(rx_c),
            uart_rx_b: UartRxBits::new(fmt),
            tone_run: 0,
            quiet_run: 0,
            speed: 0,
            events: vec![Event::Status(if originate { "Listening for the far end's modem".into() } else { "Answering with modem tones".into() })],
            said_2100: false,
            bitbuf: Vec::new(),
        }
    }

    pub fn connected(&self) -> Option<u32> {
        if self.st == St::Connected { Some(self.speed) } else { None }
    }

    pub fn take_events(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }

    /// Bytes to send once connected.
    pub fn send(&mut self, data: &[u8]) {
        self.uart_tx.queue.extend(data);
    }

    fn goto(&mut self, st: St) {
        self.st = st;
        self.in_state = 0;
        self.tone_run = 0;
        match st {
            St::OrigDelay | St::AnsSilence => self.tx = Tx::Silent,
            St::OrigFsk | St::AnsTone => self.tx = Tx::Fsk,
            St::OrigDpsk | St::AnsDpsk => {
                self.tx = Tx::Dpsk;
                self.dpsk_rx.ones_run = 0;
            }
            _ => {}
        }
    }

    fn connect(&mut self, speed: u32) {
        self.speed = speed;
        self.tx = if speed == 300 { Tx::Fsk } else { Tx::Dpsk };
        self.uart_tx.hold = false;
        self.goto(St::Connected);
        self.quiet_run = 0;
        self.events.push(Event::Connected(speed));
    }

    fn fail(&mut self, why: &str) {
        self.goto(St::Done);
        self.tx = Tx::Silent;
        self.events.push(Event::Failed(why.to_string()));
    }

    /// The next `n` samples to send.
    pub fn generate(&mut self, n: usize) -> Vec<i16> {
        (0..n)
            .map(|_| {
                let v = match self.tx {
                    Tx::Silent => 0.0,
                    Tx::Fsk => self.fsk_tx.sample(&mut self.uart_tx),
                    Tx::Dpsk => self.dpsk_tx.sample(&mut self.uart_tx) * 0.9,
                };
                (v * AMPLITUDE).clamp(-32000.0, 32000.0) as i16
            })
            .collect()
    }

    /// Feed received samples; returns decoded bytes (only once connected).
    pub fn receive(&mut self, samples: &[i16]) -> Vec<u8> {
        let mut out = Vec::new();
        for &s in samples {
            let mut x = s as f64;
            for b in &mut self.band {
                x = b.run(x);
            }
            self.level = self.level * 0.99 + 0.01 * x * x;
            self.tone.push(x);
            if self.originate {
                self.ans2100.push(x);
            }
            let fsk_bit = self.fsk_rx.push(x);
            self.bitbuf.clear();
            let mut bits = std::mem::take(&mut self.bitbuf);
            self.dpsk_rx.push(x, &mut bits);
            self.in_state += 1;
            let present = self.level > LEVEL_MIN;
            let tone = present && self.tone.ratio() > 0.6;
            self.tone_run = if tone { self.tone_run + 1 } else { 0 };
            match self.st {
                St::OrigWait => {
                    if !self.said_2100 && present && self.ans2100.ratio() > 0.6 {
                        self.said_2100 = true;
                        self.events.push(Event::Status("Heard a 2100 Hz answer tone".into()));
                    }
                    if self.tone_run >= ms(150) {
                        self.events.push(Event::Status("Heard the far end's modem answer tone".into()));
                        self.goto(St::OrigDelay);
                    } else if self.in_state > ms(30_000) {
                        self.fail("no modem answered (no answer tone heard)");
                    }
                }
                St::OrigDelay => {
                    if self.in_state >= ms(450) {
                        if self.pref == Speed::B300 {
                            self.events.push(Event::Status("Trying 300 bit/s (Bell 103)".into()));
                            self.goto(St::OrigFsk);
                        } else {
                            self.events.push(Event::Status("Trying 1200 bit/s (Bell 212A)".into()));
                            self.goto(St::OrigDpsk);
                        }
                    }
                }
                St::OrigDpsk => {
                    if present && self.tone.ratio() < 0.5 && self.dpsk_rx.ones_run >= 96 {
                        self.connect(1200);
                    } else if self.in_state > ms(4000) {
                        if self.pref == Speed::Auto {
                            self.events.push(Event::Status("No answer at 1200: trying 300 bit/s (Bell 103)".into()));
                            self.goto(St::OrigFsk);
                        } else {
                            self.fail("the far end's modem did not take 1200 bit/s");
                        }
                    }
                }
                St::OrigFsk => {
                    if self.tone_run >= ms(300) && self.in_state > ms(300) {
                        self.connect(300);
                    } else if self.in_state > ms(5000) {
                        self.fail("the far end's modem did not take 300 bit/s");
                    }
                }
                St::AnsSilence => {
                    if self.in_state >= ms(1000) {
                        self.events.push(Event::Status("Sending the answer tone".into()));
                        self.goto(St::AnsTone);
                    }
                }
                St::AnsTone => {
                    if self.pref != Speed::B1200 && self.tone_run >= ms(150) {
                        self.connect(300);
                    } else if self.pref != Speed::B300 && present && self.tone.ratio() < 0.5 && self.dpsk_rx.ones_run >= 60 {
                        self.events.push(Event::Status("The caller wants 1200 bit/s".into()));
                        self.goto(St::AnsDpsk);
                    } else if self.in_state > ms(15_000) {
                        self.fail("the caller's modem never answered our tone");
                    }
                }
                St::AnsDpsk => {
                    if self.in_state >= ms(765) {
                        self.connect(1200);
                    }
                }
                St::Connected => {
                    self.quiet_run = if present { 0 } else { self.quiet_run + 1 };
                    if self.quiet_run == ms(1500) {
                        self.events.push(Event::CarrierLost);
                        self.goto(St::Done);
                        self.tx = Tx::Silent;
                    } else if self.speed == 300 {
                        if let Some(c) = self.uart_rx_s.push(fsk_bit) {
                            out.push(c);
                        }
                    } else {
                        for &b in &bits {
                            if let Some(c) = self.uart_rx_b.push(b) {
                                out.push(c);
                            }
                        }
                    }
                }
                St::Done => {}
            }
            self.bitbuf = bits;
        }
        out
    }
}

/// A 16-bit sample to a G.711 u-law byte.
pub fn linear_to_ulaw(s: i16) -> u8 {
    let mut x = s as i32;
    let sign = if x < 0 {
        x = -x;
        0x80
    } else {
        0
    };
    x = x.min(32635) + 0x84;
    let mut exp = 7;
    let mut mask = 0x4000;
    while x & mask == 0 && exp > 0 {
        exp -= 1;
        mask >>= 1;
    }
    let mantissa = (x >> (exp + 3)) & 0x0F;
    !((sign | (exp << 4) | mantissa) as u8)
}


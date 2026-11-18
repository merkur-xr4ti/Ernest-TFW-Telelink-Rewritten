//! The payphone's data protocol, as recorded on real calls between the original program and
//! a payphone (modem transaction logs, November 2026):
//!
//! * before dialing: `ATZ`, `AT<init addons>` (L3), `ATS0=0`, `AT<D3/D1 addons>` (S37=5L3),
//!   `ATS7=75`, `ATDT<number>`; the line is 1200 bit/s Bell 212A, characters 7E1.
//! * log in: `~` + 6-digit ID + CR → the phone answers ACK (0x06).
//! * read: `!2` + count (2 hex) + address (4 hex), no CR → the phone answers one Intel-HEX data
//!   record `:LLAAAA00<data>CC` CR LF; the program answers ACK (or NAK + the request again),
//!   then the phone sends SUB + `:00` + next address + `01` + two more hex digits.
//! * write: `!1` then Intel-HEX data records (no CR), 0x37 bytes each; the phone ACKs each one;
//!   the program ends with `:00`, then reads the area back to check it.
//! * end: `!0`, then the modem escape and hang-up.
//! Speed dial numbers: address 0x0D00, ten numbers of 11 digits, one digit per byte.

use std::time::{Duration, Instant};

pub const ACK: u8 = 0x06;
pub const NAK: u8 = 0x15;
pub const SUB: u8 = 0x1A;
pub const BLOCK: usize = 0x37;
/// Largest read the original made (feature upload, !2500800).
pub const READ_MAX: usize = 0x50;
/// Characters a second on the line (1200 bit/s, 10 bits a character), less a margin.
const LINE_CPS: u64 = 105;
const POLL: Duration = Duration::from_millis(5);

/// The data record line the phone sends for a block (to spot repeats).
fn cmd_record_line(addr: u16, data: &[u8]) -> String {
    hex_record(addr, data)
}
pub const FIRMWARE_ADDR: u16 = 0xFFF0;
pub const FIRMWARE_LEN: usize = 8;
#[allow(dead_code)] // D3 value; Map picks per phone type
pub const STATUS_ADDR: u16 = 0x06A1;
pub const SPEED_DIAL_DIGITS: usize = 11;
pub const SPEED_DIAL_COUNT: usize = 10;

/// One Intel-HEX data record (type 00), without line end.
pub fn hex_record(addr: u16, data: &[u8]) -> String {
    let mut sum: u8 = data.len() as u8;
    sum = sum.wrapping_add((addr >> 8) as u8).wrapping_add(addr as u8);
    let mut s = format!(":{:02X}{:04X}00", data.len(), addr);
    for &b in data {
        s.push_str(&format!("{b:02X}"));
        sum = sum.wrapping_add(b);
    }
    s.push_str(&format!("{:02X}", sum.wrapping_neg()));
    s
}

/// Parse `:LLAAAATT<data>CC`. Returns (address, type, data) when the checksum is right.
pub fn parse_record(line: &str) -> Option<(u16, u8, Vec<u8>)> {
    let t = line.trim().strip_prefix(':')?;
    if t.len() < 10 || t.len() % 2 != 0 {
        return None;
    }
    let bytes: Vec<u8> = (0..t.len()).step_by(2).map(|i| u8::from_str_radix(&t[i..i + 2], 16)).collect::<Result<_, _>>().ok()?;
    let n = bytes[0] as usize;
    if bytes.len() != n + 5 {
        return None;
    }
    if bytes.iter().fold(0u8, |a, &b| a.wrapping_add(b)) != 0 {
        return None;
    }
    Some((u16::from_be_bytes([bytes[1], bytes[2]]), bytes[3], bytes[4..4 + n].to_vec()))
}

/// Speed dial memory (110 bytes) → ten numbers as digit strings (0x0A shows as *, 0x0C as #).
pub fn speed_dial_from_bytes(b: &[u8]) -> Vec<String> {
    b.chunks(SPEED_DIAL_DIGITS)
        .take(SPEED_DIAL_COUNT)
        .map(|c| {
            let s: String = c
                .iter()
                .map(|&d| match d {
                    0..=9 => (b'0' + d) as char,
                    0x0A => '*',
                    0x0C => '#',
                    _ => '?',
                })
                .collect();
            // An all-zero entry is an empty speed dial; shorter numbers are kept right-aligned
            // with zeros in front, which are dropped for display.
            if s.chars().all(|c| c == '0') { String::new() } else { s.trim_start_matches('0').to_string() }
        })
        .collect()
}

/// Ten numbers → 110 bytes, each right-aligned in 11 digits and padded with 0 in front, as the
/// recorded write did (an entry of "3" was sent as 00000000003).
pub fn speed_dial_to_bytes(numbers: &[String]) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(SPEED_DIAL_DIGITS * SPEED_DIAL_COUNT);
    for i in 0..SPEED_DIAL_COUNT {
        let n = numbers.get(i).map(|s| s.as_str()).unwrap_or("");
        let digits: Vec<u8> = n
            .chars()
            .filter(|c| !matches!(c, '-' | ' '))
            .map(|c| match c {
                '0'..='9' => Ok(c as u8 - b'0'),
                '*' => Ok(0x0A),
                '#' => Ok(0x0C),
                _ => Err(format!("Speed Dial {i}: only digits, * and # can be sent.")),
            })
            .collect::<Result<_, _>>()?;
        if digits.len() > SPEED_DIAL_DIGITS {
            return Err(format!("Speed Dial {i}: at most 11 digits."));
        }
        out.extend(std::iter::repeat_n(0u8, SPEED_DIAL_DIGITS - digits.len()));
        out.extend(digits);
    }
    Ok(out)
}

// ------------------------------------------------ more areas, from further recorded calls

/// The clock: 16 bytes at 0x0000; bytes 2..16 are the digits YYMMDD WW hhmmss in reverse order.
pub const CLOCK_ADDR: u16 = 0x0000;
pub const CLOCK_LEN: usize = 16;
/// Primary and secondary firmware versions (6 ASCII bytes each).
pub const PRIMARY_FW_ADDR: u16 = 0x0564;
pub const SECONDARY_FW_ADDR: u16 = 0x056B;
/// Scrolling message: 20 spaces, FF, the text, FF, up to the OCC Code #1 option at 0x0980.
pub const SCROLL_ADDR: u16 = 0x095D;
/// The original reads 128 bytes here (ScrollIt `sub0303 128`).
pub const SCROLL_READ_LEN: usize = 128;
pub const SCROLL_LEAD: usize = 20;
/// Coin box total (3 BCD bytes) and cumulative total (3 BCD bytes), options 17 and 18.
#[allow(dead_code)] // D3 value; Map picks per phone type
pub const COIN_ADDR: u16 = 0x088C;
/// Instant refund calls (2 BCD bytes) and amount (3 BCD bytes), options 52b and 52c.
#[allow(dead_code)] // D3 value; Map picks per phone type
pub const REFUND_ADDR: u16 = 0x0923;
/// The collection block the original reads (35 bytes; its layout isn't decoded yet).
#[allow(dead_code)] // D3 value; Map picks per phone type
pub const COLLECTION_ADDR: u16 = 0x0C80;
#[allow(dead_code)] // D3 value; Map picks per phone type
pub const COLLECTION_LEN: usize = 0x23;

#[derive(Clone, Debug, PartialEq)]
pub struct Clock {
    pub yy: u8,
    pub mm: u8,
    pub dd: u8,
    /// The phone's day code (the original sent 03 on a Sunday).
    pub day: u8,
    pub hh: u8,
    pub mi: u8,
    pub ss: u8,
}

fn two(d: &[u8]) -> u8 {
    (d[0] & 0x0F) * 10 + (d[1] & 0x0F)
}

impl Clock {
    pub fn from_bytes(b: &[u8]) -> Option<Clock> {
        if b.len() < CLOCK_LEN {
            return None;
        }
        // frmTimeDate sub18D1 takes the low digit of each byte and checks nothing, so a phone
        // whose clock was never set (the simulated D32.13: 00 08 00 08...) still shows a value.
        let d: Vec<u8> = b[2..16].iter().rev().map(|x| (x & 0x0F).min(9)).collect();
        Some(Clock { yy: two(&d[0..]), mm: two(&d[2..]), dd: two(&d[4..]), day: two(&d[6..]), hh: two(&d[8..]), mi: two(&d[10..]), ss: two(&d[12..]) })
    }
    /// The 14 digits `!5` sends: YY MM DD WW hh mm ss, one digit per byte.
    pub fn digits(&self) -> Vec<u8> {
        [self.yy, self.mm, self.dd, self.day, self.hh, self.mi, self.ss].iter().flat_map(|v| [v / 10, v % 10]).collect()
    }
}

/// Day code from a weekday counted from Sunday (0..6). Only Sunday = 03 is recorded; the rest
/// follow on from it (Friday = 01) and are estimated.
pub fn day_code(from_sunday: u32) -> u8 {
    ((from_sunday + 2) % 7 + 1) as u8
}
#[cfg(test)]
pub fn day_from_code(code: u8) -> u32 {
    ((code as u32 + 7 - 1) + 7 - 2) % 7
}

pub fn scroll_bytes(text: &str) -> Result<Vec<u8>, String> {
    // The original form takes up to 96 characters and writes them all (Module1 sub01CB).
    if text.chars().count() > 96 {
        return Err("A scrolling message holds at most 96 characters.".into());
    }
    if !text.is_ascii() {
        return Err("Only plain letters, digits and punctuation can be shown on the display.".into());
    }
    let mut b = vec![b' '; SCROLL_LEAD];
    b.push(0xFF);
    b.extend(text.as_bytes());
    b.push(0xFF);
    Ok(b)
}

pub fn scroll_text(b: &[u8]) -> String {
    let parts: Vec<&[u8]> = b.split(|&c| c == 0xFF).collect();
    // 20 spaces, FF, text, FF: the text is the second part; an empty area is all FF.
    let t = parts.get(1).copied().unwrap_or(&[]);
    String::from_utf8_lossy(t).to_string()
}

/// Packed BCD bytes as a number of cents.
pub fn bcd_cents(b: &[u8]) -> Option<u64> {
    let mut v = 0u64;
    for &x in b {
        let (h, l) = (x >> 4, x & 0x0F);
        if h > 9 || l > 9 {
            return None;
        }
        v = v * 100 + (h as u64) * 10 + l as u64;
    }
    Some(v)
}

// ------------------------------------------------ from the decompiled VB3 code (Module1.bas)

/// Addresses that differ between D3 (phone type 3) and D1 (phone type 1) phones.
#[derive(Clone, Copy, Debug)]
pub struct Map {
    pub d3: bool,
}

impl Map {
    /// Speed dial: sub020B `If gv03B0 = gv0306 Then &HD00 Else &H855`.
    pub fn speed_dial(&self) -> u16 {
        if self.d3 { 0x0D00 } else { 0x0855 }
    }
    /// The 2-byte word read after the firmware: frmConnect `&H6A1` (D3) / `&H208` (D1).
    pub fn status(&self) -> u16 {
        if self.d3 { 0x06A1 } else { 0x0208 }
    }
    /// Coin box + cumulative totals (6 BCD bytes): sub0443 `&H88C` / sub0457 `&H84A`.
    pub fn coins(&self) -> u16 {
        if self.d3 { 0x088C } else { 0x084A }
    }
    /// Instant refund calls + amount (5 BCD bytes): `&H923` / `&HA92`.
    pub fn refunds(&self) -> u16 {
        if self.d3 { 0x0923 } else { 0x0A92 }
    }
    /// Collection block: 35 bytes at `&HC80` (D3) / 20 bytes at `&H8C4` (D1).
    pub fn collection(&self) -> (u16, usize) {
        if self.d3 { (0x0C80, 35) } else { (0x08C4, 20) }
    }
    /// CSN table: sub01AC writes at `&H1000` (D3) / `&HC00` (D1); frmUPTTable reads from
    /// `&H1000` (up to +&H7F8) / `&H13A7` (up to +&H3FC). 17 bytes an entry, FF after the last.
    pub fn csn_write(&self) -> u16 {
        if self.d3 { 0x1000 } else { 0x0C00 }
    }
    pub fn csn_read(&self) -> (u16, usize) {
        if self.d3 { (0x1000, 0x7F8 / CSN_ENTRY) } else { (0x13A7, 0x3FC / CSN_ENTRY) }
    }
    /// Feature upload (sub046D): 128-byte pages from 0x0800; D3 reads the first 80 bytes of
    /// each page that holds options (16 pages), D1 reads all 8 pages whole.
    pub fn feature_pages(&self, addrs: &[(u16, usize)]) -> Vec<(u16, usize)> {
        if self.d3 {
            let mut pages: Vec<u16> = addrs
                .iter()
                .filter(|(a, _)| (0x0800..0x1000).contains(a))
                .map(|(a, _)| 0x0800 + ((a - 0x0800) / 128) * 128)
                .collect();
            pages.sort();
            pages.dedup();
            pages.into_iter().map(|p| (p, 80)).collect()
        } else {
            (0..8).map(|i| (0x0800 + i * 128, 128)).collect()
        }
    }
}

pub const CSN_ENTRY: usize = 17;
/// Rate table: the binary .RTF file (8192 bytes) goes to 0x4000 in 80-byte records (sub01FB).
pub const RATE_ADDR: u16 = 0x4000;
pub const RECORD_MAX: usize = 80;

/// AOS script `n` (1..3, D3 phones only): sub014A `&HD80 + (n-1)*42 + ((n-1)\3)*2`; 11 phone
/// digits + 16 authorization digits + 15 instruction bytes (FF-padded) = 42 bytes.
pub fn aos_addr(n: usize) -> u16 {
    (0x0D80 + (n - 1) * 42 + ((n - 1) / 3) * 2) as u16
}

/// One digit a byte, as sub0128 encodes: 0-9, * → 0x0A, _ → 0x00; other characters dropped.
pub fn digits_to_bytes(s: &str) -> Vec<u8> {
    s.chars()
        .filter_map(|c| match c {
            '0'..='9' => Some(c as u8 - b'0'),
            '*' => Some(0x0A),
            '#' => Some(0x0C),
            '_' => Some(0),
            _ => None,
        })
        .collect()
}

pub fn bytes_to_digits(b: &[u8]) -> String {
    b.iter()
        .map(|&d| match d {
            0..=9 => (b'0' + d) as char,
            0x0A => '*',
            0x0C => '#',
            _ => '?',
        })
        .collect()
}

/// CSN rate types and their codes for USA phones (sub00F0, `gv0300 = gv02EC` branch; that
/// this branch is the USA one is inferred).
pub const CSN_RATES: [(&str, u8); 6] =
    [("Default", 0x00), ("Blocked", 0x01), ("Interstate", 0x04), ("Intralata", 0x06), ("Interlata", 0x08), ("Local", 0x0A)];

/// A CSN entry as the phone keeps it (sub00F0): 10 digit bytes, the rate code, then
/// dollars, cents and period of the initial and the second charge as BCD bytes.
pub fn csn_bytes(phone: &str, rate: &str, a1: &str, p1: &str, a2: &str, p2: &str) -> Result<Vec<u8>, String> {
    let mut digits: Vec<u8> = digits_to_bytes(&phone.replace('-', ""));
    if digits.len() > 10 {
        return Err(format!("{phone}: at most 10 digits."));
    }
    while digits.len() < 10 {
        digits.push(0);
    }
    let code = CSN_RATES
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(rate))
        .map(|(_, c)| *c)
        .ok_or(format!("{phone}: choose one of the rate types {}.", CSN_RATES.map(|r| r.0).join(", ")))?;
    let money = |a: &str, p: &str| -> Result<[u8; 3], String> {
        let t = a.trim().trim_start_matches('$');
        let (d, c) = t.split_once('.').unwrap_or((t, "00"));
        let d: u8 = d.trim().parse().map_err(|_| format!("{phone}: bad amount {a}"))?;
        let c: u8 = format!("{c:0<2}")[..2].parse().map_err(|_| format!("{phone}: bad amount {a}"))?;
        let p: u8 = p.trim().parse().map_err(|_| format!("{phone}: bad period {p}"))?;
        if d > 99 || p > 99 {
            return Err(format!("{phone}: amounts up to $99.99, periods up to 99."));
        }
        Ok([d / 10 << 4 | d % 10, c / 10 << 4 | c % 10, p / 10 << 4 | p % 10])
    };
    digits.push(code);
    if code == 0x01 {
        digits.extend([0u8; 6]);
    } else {
        digits.extend(money(a1, p1)?);
        digits.extend(money(a2, p2)?);
    }
    Ok(digits)
}

/// (phone, rate type, initial amount, initial period, second amount, second period)
pub fn csn_from_bytes(b: &[u8]) -> Option<(String, String, String, String, String, String)> {
    if b.len() < CSN_ENTRY || b[0] == 0xFF {
        return None;
    }
    let d = bytes_to_digits(&b[..10]);
    let phone = format!("{}-{}-{}", &d[..3], &d[3..6], &d[6..]);
    let rate = CSN_RATES.iter().find(|(_, c)| *c == b[10]).map(|r| r.0.to_string()).unwrap_or(format!("Code {:02X}", b[10]));
    let bcd = |x: u8| format!("{:X}{:X}", x >> 4, x & 0x0F);
    Some((
        phone,
        rate,
        format!("${}.{}", bcd(b[11]).parse::<u32>().unwrap_or(0), bcd(b[12])),
        bcd(b[13]),
        format!("${}.{}", bcd(b[14]).parse::<u32>().unwrap_or(0), bcd(b[15])),
        bcd(b[16]),
    ))
}

/// AOS instruction codes (frmAOSScript menu handlers): low digit = instruction, high digit = its
/// setting. Instructions 7 and 10-13 are *estimated*.
pub fn aos_code(instruction: &str) -> Option<u8> {
    let (head, param) = instruction.split_once(':').map(|(h, p)| (h.trim(), Some(p.trim()))).unwrap_or((instruction.trim(), None));
    let n: u32 = head.split(' ').next()?.parse().ok()?;
    let digit = |p: &str| -> Option<u8> {
        match p {
            "0" => Some(0),
            "*" => Some(0x0A),
            "#" => Some(0x0C),
            d if d.len() == 1 && d.as_bytes()[0].is_ascii_digit() => Some(d.as_bytes()[0] - b'0'),
            _ => None,
        }
    };
    Some(match n {
        1 => 0x10,
        2 => 0x11,
        3 => 0x02,
        4 => digit(param?)? << 4 | 0x03,
        5 => 0x05,
        6 => 0x06,
        7 => 0x07,
        8 => {
            let tenth: u8 = param?.chars().nth(2)?.to_digit(10)? as u8;
            tenth << 4 | 0x07
        }
        9 => digit(param?)? << 4 | 0x0C,
        10 => 0x08,
        11 => match param? {
            "Number" => 0x09,
            "Expiry Date MM/YR" => 0x0A,
            "Expiry Date Month" => 0x0B,
            "Expiry Date Year" => 0x0D,
            _ => return None,
        },
        12 => 0xE0,
        13 => 0xE1,
        _ => return None,
    })
}

// ------------------------------------------------ SMDR (Module1 sub0479 / sub02E3)

/// 8 bytes whose 4th and 5th bytes hold the number of stored calls as 4 decimal digits.
pub const SMDR_COUNT_ADDR: u16 = 0x0400;
pub const SMDR_RECORD: usize = 25;

impl Map {
    /// Call records start at `&H2000` (D3) / `&H1000` (D1).
    pub fn smdr(&self) -> u16 {
        if self.d3 { 0x2000 } else { 0x1000 }
    }
}

/// Number of stored calls, from the 8 bytes at 0x0400 (`Val(Mid$(reply, 16, 4))`).
pub fn smdr_count(b: &[u8]) -> usize {
    if b.len() < 5 {
        return 0;
    }
    format!("{:02X}{:02X}", b[3], b[4]).parse().unwrap_or(0)
}

/// A call record as the old program stores it: 50 hex characters. On D1 phones the length's
/// seconds are dropped (`Mid$(rec, 7, 3) = Mid$(rec, 8, 2) & "0"`).
pub fn smdr_record_hex(b: &[u8], d1: bool) -> String {
    let mut h: Vec<char> = b.iter().take(SMDR_RECORD).flat_map(|x| format!("{x:02X}").chars().collect::<Vec<_>>()).collect();
    if d1 && h.len() >= 9 {
        let (a, c) = (h[7], h[8]);
        h[6] = a;
        h[7] = c;
        h[8] = '0';
    }
    h.into_iter().collect()
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SmdrCall {
    pub date: String,
    pub time: String,
    pub number: String,
    pub length: String,
    pub amount: String,
    pub call_type: String,
    pub code: String,
    pub posted: String,
}

/// Decode one 50-hex-character record (sub02E3). `year` is this year, for the decade.
pub fn smdr_decode(r: &str, year: i32) -> Option<SmdrCall> {
    let c: Vec<char> = r.chars().collect();
    if c.len() < 50 || r.chars().all(|x| x == 'F') {
        return None;
    }
    let at = |i: usize| c[i - 1]; // 1-based, like Mid$
    let hex = |i: usize, n: usize| u32::from_str_radix(&r[i - 1..i - 1 + n], 16).ok();
    // A record with bit 8 of character 49 set is the data/fax kind.
    let data_kind = hex(49, 1)? & 0x8 == 8;
    let month = match at(2) {
        '1'..='9' => at(2).to_digit(10)?,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        _ => return None,
    };
    let b34 = hex(3, 2)?;
    let b56 = hex(5, 2)?;
    let day = (b34 & 0xF8) >> 3;
    let hour = (b34 & 0x07) * 4 + ((b56 & 0xC0) >> 6);
    let minute = b56 & 0x3F;
    if day > 31 || hour > 23 || minute > 59 {
        return None;
    }
    let ydigit = at(1).to_digit(10)? as i32;
    let mut decade = (year % 100) / 10;
    if year % 10 < ydigit {
        decade = (decade + 9) % 10;
    }
    let date = format!("{month:02}-{day:02}-{decade}{ydigit}");
    let time = format!("{hour:02}:{minute:02}");
    // Dialed digits: from character 15 (19 for 10XXX calls) up to the first D; A and F are
    // fillers, C is #.
    let kind = at(48);
    let start = if kind == '9' { 19 } else { 15 };
    let mut number = String::new();
    for i in start..=28 {
        match at(i) {
            'D' => break,
            'A' | 'F' => {}
            'C' => number.push('#'),
            d => number.push(d),
        }
    }
    let length = format!("{}:{}", &r[6..8], &r[8..10]);
    let (amount, call_type, code, posted);
    if data_kind {
        call_type = match kind {
            '1' => "Direct",
            '2' => "Send_Fax",
            '3' => "Recieve_Fax",
            '4' => "Data_Port",
            _ => return None,
        }
        .to_string();
        amount = format!("${}.{}", &r[9..12], &r[12..14]);
        let c23 = &r[22..28];
        code = if c23.contains('D') { "######".into() } else { c23.to_string() };
        posted = if hex(49, 1)? & 0x4 == 4 { "N" } else { "Y" }.to_string();
    } else {
        call_type = match kind {
            '1' if at(15) == '1' && at(16) != '0' => "OCC".to_string(),
            '1' => "Direct".into(),
            '2' => "00".into(),
            '3' => "0-".into(),
            '4' => "0+".into(),
            '5' => "211".into(),
            '6' => "611".into(),
            '7' => "Dir_Assist".into(),
            '8' => "Instant_Refund".into(),
            '9' => {
                let t = format!("10{}0", &r[14..17]);
                if t.starts_with("101") { format!("101{}0", &r[15..19]) } else { t }
            }
            'A' => "AOS_1".into(),
            'B' => "AOS_2".into(),
            'C' => "AOS_3".into(),
            'D' => "Incoming".into(),
            'E' => "Data_Port".into(),
            _ => return None,
        };
        amount = format!("${}.{}", &r[10..12], &r[12..14]);
        code = String::new();
        posted = String::new();
    }
    Some(SmdrCall { date, time, number, length, amount, call_type, code, posted })
}

/// The first record of an uploaded block in the .SMD file: the phone, padded with dots.
pub fn smdr_header(phone_digits: &str) -> String {
    let d: String = phone_digits.chars().filter(|c| c.is_ascii_digit()).collect();
    let d = if d.len() == 11 && d.starts_with('1') { d[1..].to_string() } else { d };
    let h = if d.len() == 10 { format!("{}-{}", &d[..3], &d[3..]) } else { format!("S{d}") };
    format!("{h:.<50}")
}

/// Read an old .SMD file: 50-character records; a record that isn't hex starts a new phone.
pub fn smdr_file_rows(bytes: &[u8], year: i32) -> Vec<(String, SmdrCall)> {
    let mut phone = String::new();
    let mut rows = Vec::new();
    for rec in bytes.chunks(50) {
        let t: String = rec.iter().map(|&c| c as char).collect();
        if t.len() == 50 && t.chars().all(|c| c.is_ascii_hexdigit()) {
            if let Some(call) = smdr_decode(&t, year) {
                rows.push((phone.clone(), call));
            }
        } else {
            phone = t.trim_end_matches(['.', ' ', '\0']).trim_start_matches('S').chars().take(12).collect();
        }
    }
    rows
}

/// The instruction text for a phone code (the reverse of `aos_code`).
pub fn aos_text(code: u8) -> String {
    use crate::docs::{AOS_INSTRUCTIONS as I, CARD_PARTS, DTMF_KEYS, PAUSES};
    let key = |d: u8| -> &str {
        match d {
            0 => "0",
            1..=9 => DTMF_KEYS[(d - 1) as usize],
            0x0A => "*",
            0x0C => "#",
            _ => "?",
        }
    };
    let (hi, lo) = (code >> 4, code & 0x0F);
    match (hi, lo) {
        (1..=3, 0) => I[0].to_string(),
        (1..=3, 1) => I[1].to_string(),
        (0, 2) => I[2].to_string(),
        (_, 3) => format!("{}: {}", I[3], key(hi)),
        (0, 5) => I[4].to_string(),
        (0, 6) => I[5].to_string(),
        (0, 7) => I[6].to_string(),
        (1..=9, 7) => format!("{}: {}", I[7], PAUSES[(hi - 1) as usize]),
        (_, 0x0C) => format!("{}: {}", I[8], key(hi)),
        (0, 8) => I[9].to_string(),
        (0, 9) => format!("{}: {}", I[10], CARD_PARTS[0]),
        (0, 0x0A) => format!("{}: {}", I[10], CARD_PARTS[1]),
        (0, 0x0B) => format!("{}: {}", I[10], CARD_PARTS[2]),
        (0, 0x0D) => format!("{}: {}", I[10], CARD_PARTS[3]),
        (0x0E, 0) => I[11].to_string(),
        (0x0E, 1) => I[12].to_string(),
        _ => format!("Code {code:02X}"),
    }
}

pub fn aos_bytes(phone: &str, auth: &str, instructions: &[String]) -> Result<Vec<u8>, String> {
    let mut b = digits_to_bytes(phone);
    if b.len() > 11 {
        return Err("The AOS phone number has at most 11 digits.".into());
    }
    while b.len() < 11 {
        b.insert(0, 0);
    }
    let mut a = digits_to_bytes(auth);
    if a.len() > 16 {
        return Err("The authorization code has at most 16 digits.".into());
    }
    while a.len() < 16 {
        a.push(0);
    }
    b.extend(a);
    if instructions.len() > 15 {
        return Err("A script holds at most 15 instructions.".into());
    }
    let mut codes = Vec::new();
    for i in instructions {
        codes.push(aos_code(i).ok_or(format!("No phone code is known for \"{i}\"."))?);
    }
    codes.resize(15, 0xFF);
    b.extend(codes);
    Ok(b)
}

/// The ID as it is sent: the six digits in reverse order (123456 → 654321).
pub fn line_id(id: &str) -> String {
    id.trim().chars().rev().collect()
}

/// Turn a firmware block (`E40.10` + 2 bytes) into its text.
pub fn firmware_text(b: &[u8]) -> String {
    b.iter().take(6).map(|&c| if c.is_ascii_graphic() { c as char } else { ' ' }).collect::<String>().trim().to_string()
}

/// The byte that picks which firmware runs: 0x55 secondary, 0x00 primary (Module1 sub03F1).
pub const RUN_SELECT_ADDR: u16 = 0x0A25;

/// A firmware file split into what Download New Firmware sends (Module1 sub01BB).
#[derive(Clone, Debug)]
pub struct FirmwarePlan {
    /// The version written at 0xFFF0 of the secondary image ("E40.10").
    pub version: String,
    /// The `!8` session: the first half's code at 0x8000 up to its end address, then its last 32
    /// bytes at 0xFFE0.
    pub first: Vec<(u16, Vec<u8>)>,
    /// The `!9` session: the whole second half at 0x8000-0xFFFF.
    pub second: Vec<(u16, Vec<u8>)>,
}

impl FirmwarePlan {
    pub fn records(&self) -> usize {
        self.first.len() + self.second.len()
    }
}

fn chunk_at(start: u16, data: &[u8]) -> Vec<(u16, Vec<u8>)> {
    data.chunks(RECORD_MAX).enumerate().map(|(i, c)| (start.wrapping_add((i * RECORD_MAX) as u16), c.to_vec())).collect()
}

/// Read a firmware file the way the original does: 64 KB, two 32 KB halves for 0x8000-0xFFFF.
/// The second half must hold an "E" (secondary) version at 0xFFF0; the first half names the end
/// of its code as four hex characters at 0xFFE0.
pub fn firmware_plan(file: &[u8]) -> Result<FirmwarePlan, String> {
    const HALF: usize = 0x8000;
    if file.len() < 2 * HALF {
        return Err(format!(
            "Wrong Firmware Version: this file is {} bytes. Download New Firmware sends a 64 KB secondary (E) firmware file; a 32 KB file is a primary chip image, which can't be sent down the phone line.",
            file.len()
        ));
    }
    let (lo, hi) = (&file[..HALF], &file[HALF..2 * HALF]);
    if hi[0x7FF0] != b'E' {
        return Err(format!(
            "Wrong Firmware Version: the file holds {:?} at FFF0; only secondary firmware (starting with E) can be downloaded.",
            firmware_text(&hi[0x7FF0..])
        ));
    }
    let end_text = String::from_utf8_lossy(&lo[0x7FE0..0x7FE4]).trim_start().to_string();
    let end = u16::from_str_radix(end_text.trim_end_matches('\0'), 16)
        .map_err(|_| format!("Bad firmware file: the end address at FFE0 should be four hex characters, not {end_text:?}."))?;
    let n = end.wrapping_sub(0x8000) as usize;
    if n == 0 || n > 0x7FE0 {
        return Err(format!("Bad firmware file: its end address {end:04X} is outside 8001-FFE0."));
    }
    let mut first = chunk_at(0x8000, &lo[..n]);
    first.push((0xFFE0, lo[0x7FE0..HALF].to_vec()));
    Ok(FirmwarePlan { version: firmware_text(&hi[0x7FF0..]), first, second: chunk_at(0x8000, hi) })
}

/// Where the modem bytes go (the COM port; the tests use a pretend phone).
pub trait Line {
    fn send(&mut self, data: &[u8]) -> Result<(), String>;
    /// Bytes that arrived since the last call.
    fn take(&mut self) -> Vec<u8>;
    /// End the call.
    fn hang_up(&mut self);
}

pub struct Session<'a> {
    pub line: &'a mut dyn Line,
    buf: Vec<u8>,
    pub progress: &'a mut dyn FnMut(&str),
    pub cancel: &'a dyn Fn() -> bool,
}

pub struct DialSetup {
    pub number: String,
    pub init_addons: String,
    pub type_addons: String,
    pub dial_timeout: u32,
}

fn wait(d: Duration) {
    std::thread::sleep(d);
}

impl<'a> Session<'a> {
    pub fn new(line: &'a mut dyn Line, progress: &'a mut dyn FnMut(&str), cancel: &'a dyn Fn() -> bool) -> Self {
        Self { line, buf: Vec::new(), progress, cancel }
    }

    fn pump(&mut self) {
        let b = self.line.take();
        self.buf.extend(b);
    }

    fn check_cancel(&self) -> Result<(), String> {
        if (self.cancel)() { Err("Cancelled By User".into()) } else { Ok(()) }
    }

    /// Wait until `done(buffer)` says yes, or the time runs out.
    fn wait_for(&mut self, timeout: Duration, mut done: impl FnMut(&[u8]) -> bool) -> bool {
        let start = Instant::now();
        loop {
            self.pump();
            if done(&self.buf) {
                return true;
            }
            if start.elapsed() > timeout || (self.cancel)() {
                return false;
            }
            wait(POLL);
        }
    }

    fn drain(&mut self, quiet: Duration) {
        let start = Instant::now();
        while start.elapsed() < quiet {
            self.pump();
            wait(Duration::from_millis(20));
        }
        self.buf.clear();
    }

    fn at(&mut self, cmd: &str, timeout: Duration) -> Result<(), String> {
        self.buf.clear();
        self.line.send(format!("{cmd}\r").as_bytes())?;
        let ok = self.wait_for(timeout, |b| {
            let t = String::from_utf8_lossy(b);
            t.contains("OK") || t.contains("ERROR")
        });
        let text = String::from_utf8_lossy(&self.buf).to_string();
        if !ok || text.contains("ERROR") {
            return Err(format!("The modem didn't accept {cmd}."));
        }
        Ok(())
    }

    /// Set the modem up and dial, as the original program did. Returns the CONNECT line.
    pub fn dial(&mut self, s: &DialSetup) -> Result<String, String> {
        (self.progress)("Initializing the modem...");
        self.at("ATZ", Duration::from_secs(5))?;
        wait(Duration::from_millis(300));
        if !s.init_addons.trim().is_empty() {
            self.at(&format!("AT{}", s.init_addons.trim()), Duration::from_secs(3))?;
        }
        self.at("ATS0=0", Duration::from_secs(3))?;
        if !s.type_addons.trim().is_empty() {
            self.at(&format!("AT{}", s.type_addons.trim()), Duration::from_secs(3))?;
        }
        self.at(&format!("ATS7={}", s.dial_timeout), Duration::from_secs(3))?;
        self.check_cancel()?;
        (self.progress)(&format!("Dialing {}...  Waiting for connection...", s.number));
        self.buf.clear();
        self.line.send(format!("ATDT{}\r", s.number).as_bytes())?;
        let results = ["CONNECT", "NO CARRIER", "BUSY", "NO ANSWER", "NO DIALTONE", "ERROR"];
        let got = self.wait_for(Duration::from_secs(s.dial_timeout as u64 + 30), |b| {
            let t = String::from_utf8_lossy(b);
            results.iter().any(|r| t.contains(r)) && t.ends_with('\n')
        });
        let text = String::from_utf8_lossy(&self.buf).to_string();
        self.check_cancel()?;
        if !got {
            return Err("Connect Timer Expired.".into());
        }
        if text.contains("BUSY") {
            return Err("Phone Was Busy".into());
        }
        if let Some(l) = text.lines().find(|l| l.contains("CONNECT")) {
            let l = l.trim().to_string();
            (self.progress)(&format!("Connected ({l})."));
            return Ok(l);
        }
        if text.contains("NO DIALTONE") {
            return Err("Did Not Connect (no dial tone).".into());
        }
        Err("Did Not Connect".into())
    }

    /// Send the ID; the phone answers ACK when it is right. The ID goes down the line with its
    /// digits reversed, as the original sent it (ID 123456 goes as `~654321`).
    pub fn log_in(&mut self, id: &str) -> Result<(), String> {
        wait(Duration::from_millis(1800));
        self.drain(Duration::from_millis(200));
        (self.progress)("Checking ID...");
        self.line.send(format!("~{}\r", line_id(id)).as_bytes())?;
        if self.wait_for(Duration::from_secs(6), |b| b.contains(&ACK) || b.contains(&NAK)) && self.buf.contains(&ACK) {
            (self.progress)("ID Acknowledged.");
            self.drain(Duration::from_millis(400));
            Ok(())
        } else {
            Err("Incorrect ID.".into())
        }
    }

    /// Seconds a run of characters takes on the line (1200 bit/s, 10 bits a character, with room
    /// for the SIP modem's delay).
    fn line_time(chars: usize) -> Duration {
        Duration::from_millis((chars as u64 * 1000) / LINE_CPS + 700)
    }

    /// Wait until nothing has arrived for `quiet` (at most `max`), so the next request doesn't
    /// run into the end of a repeated block. Keeps what arrived in the buffer.
    fn wait_quiet(&mut self, quiet: Duration, max: Duration) {
        let start = Instant::now();
        let mut last = Instant::now();
        while start.elapsed() < max {
            let before = self.buf.len();
            self.pump();
            if self.buf.len() != before {
                last = Instant::now();
            } else if last.elapsed() >= quiet {
                break;
            }
            wait(POLL);
        }
    }

    /// Read `len` bytes (up to 0x50) at `addr`. Answers each copy of the block with ACK at once,
    /// asks again as soon as a reply is late, wrong or garbled, and waits for the line to go
    /// quiet before the next request.
    pub fn read(&mut self, addr: u16, len: usize) -> Result<Vec<u8>, String> {
        assert!(len > 0 && len <= 0xFF);
        let cmd = format!("!2{len:02X}{addr:04X}");
        let reply_chars = 2 * len + 13;
        for attempt in 0..6 {
            self.check_cancel()?;
            self.wait_quiet(Duration::from_millis(120), Duration::from_millis(1500));
            self.buf.clear();
            let mut out = Vec::new();
            if attempt > 0 {
                out.push(NAK);
            }
            out.extend(cmd.as_bytes());
            self.line.send(&out)?;
            let sent = Instant::now();
            let mut started: Option<Instant> = None;
            let mut found: Option<Vec<u8>> = None;
            let mut bad = false;
            loop {
                self.pump();
                if started.is_none() && self.buf.contains(&b':') {
                    started = Some(Instant::now());
                }
                // Every complete line that has arrived.
                let text = String::from_utf8_lossy(&self.buf).to_string();
                let complete = match text.rfind(['\r', '\n']) {
                    Some(p) => &text[..p],
                    None => "",
                };
                for l in complete.split(['\r', '\n']) {
                    let Some(p) = l.find(':') else { continue };
                    let l = &l[p..];
                    match parse_record(l) {
                        Some((a, 0, d)) if a == addr && d.len() == len => {
                            found = Some(d);
                            break;
                        }
                        Some((_, 1, _)) => {}
                        // A whole line for some other place, or a damaged one: ask again now.
                        _ if l.len() >= 11 => bad = true,
                        _ => {}
                    }
                }
                if found.is_some() || bad || (self.cancel)() {
                    break;
                }
                let late = match started {
                    None => sent.elapsed() > Duration::from_millis(1800),
                    Some(t) => t.elapsed() > Self::line_time(reply_chars),
                };
                if late {
                    break;
                }
                wait(POLL);
            }
            if let Some(d) = found {
                self.line.send(&[ACK])?;
                self.after_block(&cmd_record_line(addr, &d));
                return Ok(d);
            }
            self.check_cancel()?;
            let why = if bad { "a damaged or wrong block" } else { "no reply in time" };
            (self.progress)(&format!("Reading {addr:04X}: {why}; asking again..."));
        }
        Err("Timed Out Error".into())
    }

    /// After ACKing a block: ACK any repeat of it, and stop at the phone's SUB end record.
    fn after_block(&mut self, record: &str) {
        self.buf.clear();
        let start = Instant::now();
        let mut acked_repeats = 0;
        while start.elapsed() < Duration::from_millis(2500) {
            self.pump();
            if let Some(p) = self.buf.iter().position(|&c| c == SUB) {
                if self.buf.len() >= p + 12 {
                    break;
                }
            }
            let text = String::from_utf8_lossy(&self.buf).to_string();
            let repeats = text.matches(&format!("{record}\r\n")).count();
            if repeats > acked_repeats {
                acked_repeats = repeats;
                let _ = self.line.send(&[ACK]);
            }
            wait(POLL);
        }
        self.wait_quiet(Duration::from_millis(100), Duration::from_millis(1200));
        self.buf.clear();
    }

    /// Read any length, in blocks of 0x50 bytes like the original's feature upload.
    pub fn read_area(&mut self, addr: u16, len: usize) -> Result<Vec<u8>, String> {
        let mut out = Vec::with_capacity(len);
        let mut a = addr as usize;
        while out.len() < len {
            let n = (len - out.len()).min(READ_MAX);
            out.extend(self.read(a as u16, n)?);
            a += n;
        }
        Ok(out)
    }

    /// Wait for the phone's ACK to one written record.
    fn record_acked(&mut self, chars: usize) -> bool {
        let deadline = Self::line_time(chars) + Duration::from_millis(4000);
        self.wait_for(deadline, |b| b.contains(&ACK) || b.contains(&NAK)) && self.buf.contains(&ACK)
    }

    /// Write data records after one `!1`, with the original's rule (Module1 sub0168): wait for the
    /// phone's ACK; on NAK or silence send `!1` and the same record again, up to 10 tries. Ends
    /// with `:00`.
    pub fn write_records(&mut self, records: &[(u16, Vec<u8>)]) -> Result<(), String> {
        self.write_records_in("!1", records, None)
    }

    /// The same as `write_records`, but the session opens with `mode` (`!1` settings, `!8` and
    /// `!9` firmware, as Module1 sub01BB) and every retry resends `mode` with the record. With a
    /// `report` (label, records done before, records in all), progress is shown every 10 records.
    pub fn write_records_in(&mut self, mode: &str, records: &[(u16, Vec<u8>)], report: Option<(&str, usize, usize)>) -> Result<(), String> {
        self.wait_quiet(Duration::from_millis(60), Duration::from_millis(800));
        self.line.send(mode.as_bytes())?;
        for (i, (addr, data)) in records.iter().enumerate() {
            self.check_cancel()?;
            if let Some((label, before, all)) = report {
                if all > 0 {
                    (self.progress)(&format!("{label}... {}%", (before + i) * 100 / all));
                }
            }
            let rec = hex_record(*addr, data);
            let mut sent_ok = false;
            for attempt in 0..10 {
                self.wait_quiet(Duration::from_millis(60), Duration::from_millis(800));
                self.buf.clear();
                let mut out = Vec::new();
                if attempt > 0 {
                    out.extend(mode.as_bytes());
                }
                out.extend(rec.as_bytes());
                self.line.send(&out)?;
                if self.record_acked(out.len()) {
                    sent_ok = true;
                    break;
                }
                self.check_cancel()?;
                (self.progress)(&format!("The phone didn't confirm {addr:04X}; sending {mode} and the record again..."));
            }
            if !sent_ok {
                let _ = self.line.send(b":00");
                return Err(format!("Acknowledge Error at {addr:04X}"));
            }
        }
        // End of the records, as the recorded program sent it.
        self.line.send(b":00")?;
        self.wait_quiet(Duration::from_millis(400), Duration::from_millis(2000));
        self.buf.clear();
        Ok(())
    }

    /// Write an area in 0x37-byte records (speed dial, as the original), then check it.
    pub fn write_area(&mut self, addr: u16, data: &[u8]) -> Result<(), String> {
        self.write_area_in(addr, data, BLOCK)
    }

    /// Write an area in records of `size` bytes, then read it back and compare.
    pub fn write_area_in(&mut self, addr: u16, data: &[u8], size: usize) -> Result<(), String> {
        let records: Vec<(u16, Vec<u8>)> =
            data.chunks(size).enumerate().map(|(i, c)| (addr + (i * size) as u16, c.to_vec())).collect();
        (self.progress)(&format!("Sending {} bytes to {addr:04X}...", data.len()));
        self.write_records(&records)?;
        (self.progress)("Checking what the phone stored...");
        let back = self.read_area(addr, data.len())?;
        if back != data {
            return Err("Checksum Error (the phone's memory doesn't match what was sent)".into());
        }
        Ok(())
    }

    /// Send a bare command such as `!7` (clear SMDR) or `:00`, between quiet moments.
    pub fn command(&mut self, cmd: &str) -> Result<(), String> {
        self.wait_quiet(Duration::from_millis(100), Duration::from_millis(1000));
        self.line.send(cmd.as_bytes())?;
        self.wait_quiet(Duration::from_millis(400), Duration::from_millis(2000));
        self.buf.clear();
        Ok(())
    }

    /// Set the clock: `!5` + the 14 digits as hex bytes; the phone doesn't answer, so the
    /// clock is read back afterwards.
    pub fn set_clock(&mut self, c: &Clock) -> Result<(), String> {
        let mut s = String::from("!5");
        for d in c.digits() {
            s.push_str(&format!("{d:02X}"));
        }
        self.buf.clear();
        self.line.send(s.as_bytes())?;
        self.drain(Duration::from_millis(2000));
        Ok(())
    }

    /// End the session and hang up.
    pub fn finish(&mut self) {
        let _ = self.line.send(b"!0");
        wait(Duration::from_millis(500));
        self.line.hang_up();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_from_the_recorded_call() {
        // The phone's firmware reply and the program's first speed dial write, byte for byte.
        let (a, t, d) = parse_record(":08FFF0004534302E31300306C8").unwrap();
        assert_eq!((a, t), (0xFFF0, 0));
        assert_eq!(firmware_text(&d), "E40.10");
        let w0 = ":370D00000105080506000405040706010806060809070807060700000000000000000000030000000000000000000004000000000000000000000534";
        let (a, _, d) = parse_record(w0).unwrap();
        assert_eq!(a, 0x0D00);
        assert_eq!(hex_record(a, &d), w0);
        let w37 = ":370D37000000000000000000000006000000000000000000000700000000000000000000080000000000000000000009000000000000000000010066";
        let (_, _, d2) = parse_record(w37).unwrap();
        let mut all = d.clone();
        all.extend(d2);
        let nums = speed_dial_from_bytes(&all);
        assert_eq!(nums[0], "15856045476");
        assert_eq!(nums[1], "18668978767");
        assert_eq!(nums[2], "3");
        assert_eq!(nums[9], "10");
        assert_eq!(speed_dial_to_bytes(&nums).unwrap(), all);
        assert!(parse_record(":0206A100400018").is_none());
    }

    #[test]
    fn clock_message_and_totals_from_the_recorded_calls() {
        // 09:33 log: the clock before and after `!50206010101050003000903050509`.
        let (_, _, before) = parse_record(":1000000000110004010006010600060101000100C4").unwrap();
        let c = Clock::from_bytes(&before).unwrap();
        assert_eq!((c.yy, c.mm, c.dd, c.day, c.hh, c.mi, c.ss), (1, 1, 16, 6, 16, 1, 40));
        let (_, _, after) = parse_record(":1000000000110400070309000300050101010602B5").unwrap();
        let c = Clock::from_bytes(&after).unwrap();
        assert_eq!((c.yy, c.mm, c.dd, c.day, c.hh, c.mi), (26, 11, 15, 3, 9, 37));
        let sent = Clock { yy: 26, mm: 11, dd: 15, day: day_code(0), hh: 9, mi: 35, ss: 59 };
        let hex: String = sent.digits().iter().map(|d| format!("{d:02X}")).collect();
        assert_eq!(format!("!5{hex}"), "!50206010101050003000903050509");
        assert_eq!(day_from_code(day_code(4)), 4);
        // The scrolling message write, byte for byte.
        let rec = ":20095D002020202020202020202020202020202020202020FF7363726F6C6C6C6C6C20FF09";
        let (a, _, d) = parse_record(rec).unwrap();
        assert_eq!(a, SCROLL_ADDR);
        assert_eq!(scroll_text(&d), "scrolllll ");
        assert_eq!(hex_record(SCROLL_ADDR, &scroll_bytes("scrolllll ").unwrap()), rec);
        // Coin box $0037.55, cumulative $5369.85 (option 17/18 at 0x088C).
        let (_, _, coins) = parse_record(":06088C0000375553698599").unwrap();
        assert_eq!(bcd_cents(&coins[..3]), Some(3755));
        assert_eq!(bcd_cents(&coins[3..]), Some(536985));
        assert_eq!(line_id("123456"), "654321");
    }

    #[test]
    fn smdr_records() {
        // Year 6, November (B), day 15, 09:35, length 02:30, $01.25, 555-010-1234, Direct.
        let rec = format!("6B7A6302300125{}{}{}", "5550101234DFFF", "F".repeat(19), "100");
        assert_eq!(rec.len(), 50);
        let c = smdr_decode(&rec, 2026).unwrap();
        assert_eq!(
            (c.date.as_str(), c.time.as_str(), c.number.as_str(), c.length.as_str(), c.amount.as_str(), c.call_type.as_str()),
            ("11-15-26", "09:35", "5550101234", "02:30", "$01.25", "Direct")
        );
        // A year digit later than this year's belongs to the previous decade.
        assert_eq!(smdr_decode(&format!("9{}", &rec[1..]), 2026).unwrap().date, "11-15-19");
        // D1: the seconds are dropped as the original does.
        let bytes: Vec<u8> = (0..25).map(|i| u8::from_str_radix(&rec[i * 2..i * 2 + 2], 16).unwrap()).collect();
        assert_eq!(smdr_decode(&smdr_record_hex(&bytes, true), 2026).unwrap().length, "23:00");
        assert_eq!(smdr_record_hex(&bytes, false), rec);
        // Count: 4th and 5th bytes as decimal digits.
        assert_eq!(smdr_count(&[0, 0, 0, 0x00, 0x12, 0, 0, 0]), 12);
        // A saved file: header, then records.
        let mut file = smdr_header("15550101234").into_bytes();
        file.extend(rec.as_bytes());
        let rows = smdr_file_rows(&file, 2026);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "555-0101234");
    }

    #[test]
    fn layouts_from_the_decompiled_code() {
        // D1 / D3 addresses (Module1 sub020B, sub0443/0457, sub0366, frmConnect).
        let (d3, d1) = (Map { d3: true }, Map { d3: false });
        assert_eq!((d3.speed_dial(), d1.speed_dial()), (0x0D00, 0x0855));
        assert_eq!((d3.status(), d1.status()), (0x06A1, 0x0208));
        assert_eq!((d3.collection(), d1.collection()), ((0x0C80, 35), (0x08C4, 20)));
        assert_eq!((d3.csn_write(), d1.csn_write()), (0x1000, 0x0C00));
        assert_eq!(d1.feature_pages(&[(0x0800, 6)]).len(), 8);
        assert_eq!(d3.feature_pages(&[(0x0800, 6), (0x0843, 1), (0x0A80, 11)]), vec![(0x0800, 80), (0x0A80, 80)]);
        // AOS script addresses: 0xD80, 0xDAA, 0xDD4.
        assert_eq!((aos_addr(1), aos_addr(2), aos_addr(3)), (0x0D80, 0x0DAA, 0x0DD4));
        // CSN entry: 10 digits, rate code, $01.25/03, $00.10/01.
        let b = csn_bytes("555-010-12**", "Local", "$1.25", "3", "$0.10", "1").unwrap();
        assert_eq!(b, vec![5, 5, 5, 0, 1, 0, 1, 2, 0x0A, 0x0A, 0x0A, 0x01, 0x25, 0x03, 0x00, 0x10, 0x01]);
        let (p, r, a1, p1, a2, p2) = csn_from_bytes(&b).unwrap();
        assert_eq!((p.as_str(), r.as_str(), a1.as_str(), p1.as_str(), a2.as_str(), p2.as_str()), ("555-010-12**", "Local", "$1.25", "03", "$0.10", "01"));
        // AOS instruction codes and back.
        for (text, code) in [
            ("1 - Dial AOS Phone #", 0x10),
            ("3 - Dial ANI", 0x02),
            ("4 - Dial DTMF Digit: 5", 0x53),
            ("4 - Dial DTMF Digit: 0", 0x03),
            ("4 - Dial DTMF Digit: *", 0xA3),
            ("4 - Dial DTMF Digit: #", 0xC3),
            ("8 - Pause: 0.3 Seconds", 0x37),
            ("9 - Wait for DTMF from AOS: 9", 0x9C),
        ] {
            assert_eq!(aos_code(text), Some(code), "{text}");
            assert_eq!(aos_code(&aos_text(code)), Some(code), "{text}");
        }
        let a = aos_bytes("1-800-555-0100", "1234", &["3 - Dial ANI".into()]).unwrap();
        assert_eq!(a.len(), 42);
        // No 13-character limit on the scrolling message any more.
        assert!(scroll_bytes("A message longer than thirteen characters").is_ok());
    }

    /// A pretend phone that answers like the recorded one.
    struct FakePhone {
        mem: Vec<u8>,
        out: Vec<u8>,
        inbuf: Vec<u8>,
        writing: bool,
        /// Send every block twice, as the real phone did when the ACK came late.
        repeat: bool,
        /// Answer the next read with a block for the wrong address.
        garble_next: bool,
        /// Stop answering after this many records in one !1 session.
        session_limit: Option<usize>,
        session_count: usize,
        /// Each write as (session command, address, length).
        writes: Vec<(String, u16, usize)>,
        mode: String,
    }
    impl Line for FakePhone {
        fn send(&mut self, data: &[u8]) -> Result<(), String> {
            self.inbuf.extend(data);
            loop {
                let s = String::from_utf8_lossy(&self.inbuf).to_string();
                if s.ends_with('\r') && s.starts_with("AT") {
                    self.out.extend(if s.starts_with("ATDT") { b"\r\nCONNECT 1200\r\n".to_vec() } else { b"\r\nOK\r\n".to_vec() });
                    self.inbuf.clear();
                } else if s.starts_with('~') && s.ends_with('\r') {
                    self.out.push(if s == "~654321\r" { ACK } else { NAK });
                    self.inbuf.clear();
                } else if let Some(r) = s.trim_start_matches(char::from(NAK)).strip_prefix("!2").filter(|r| r.len() == 6) {
                    let n = usize::from_str_radix(&r[..2], 16).unwrap();
                    let a = usize::from_str_radix(&r[2..], 16).unwrap();
                    let rec = hex_record(a as u16, &self.mem[a..a + n]);
                    if self.garble_next {
                        self.garble_next = false;
                        let wrong = hex_record(a as u16 + 0x29, &self.mem[a..a + n]);
                        self.out.extend(format!("{wrong}\r\n").as_bytes());
                    } else {
                        self.out.extend(format!("{rec}\r\n").as_bytes());
                        if self.repeat {
                            self.out.extend(format!("{rec}\r\n").as_bytes());
                        }
                    }
                    self.inbuf.clear();
                } else if s == "\u{6}" {
                    self.out.extend(b"\x1a:000D3701B8");
                    self.inbuf.clear();
                } else if s.starts_with("!1") || s.starts_with("!8") || s.starts_with("!9") {
                    self.mode = s[..2].to_string();
                    self.writing = true;
                    self.session_count = 0;
                    self.inbuf.drain(..2);
                    continue;
                } else if self.writing && s == ":00" {
                    self.writing = false;
                    self.inbuf.clear();
                } else if self.writing && s.starts_with(':') && s.len() >= 11 {
                    if let Some((a, _, d)) = parse_record(&s) {
                        self.inbuf.clear();
                        if self.session_limit.is_some_and(|l| self.session_count >= l) {
                            break; // stalls like the real phone did at 0x0896
                        }
                        self.session_count += 1;
                        self.writes.push((self.mode.clone(), a, d.len()));
                        self.mem[a as usize..a as usize + d.len()].copy_from_slice(&d);
                        self.out.push(ACK);
                    }
                } else if !self.writing && s == ":00" {
                    self.inbuf.clear();
                } else if s == "!0" {
                    self.inbuf.clear();
                }
                break;
            }
            Ok(())
        }
        fn take(&mut self) -> Vec<u8> {
            std::mem::take(&mut self.out)
        }
        fn hang_up(&mut self) {}
    }

    #[test]
    fn recovers_from_repeats_damaged_replies_and_write_stalls() {
        let mut mem = vec![0u8; 0x10000];
        for (i, b) in mem[0x0800..0x0B00].iter_mut().enumerate() {
            *b = (i % 10) as u8;
        }
        let mut phone = FakePhone { mem, out: vec![], inbuf: vec![], writing: false, repeat: true, garble_next: true, session_limit: Some(2), session_count: 0, writes: vec![], mode: String::new() };
        let mut log = |_: &str| {};
        let never = || false;
        let started = Instant::now();
        let mut s = Session::new(&mut phone, &mut log, &never);
        // A damaged first reply, then every block sent twice.
        let a = s.read(0x0800, 0x50).unwrap();
        let b = s.read(0x0850, 0x50).unwrap();
        assert_eq!(a[3], 3);
        assert_eq!(b[0], 0);
        // Five records, but the phone answers only two per !1 session.
        let recs: Vec<(u16, Vec<u8>)> = (0..5).map(|i| (0x0900 + i * 0x37, vec![9u8; 0x37])).collect();
        s.write_records(&recs).unwrap();
        assert_eq!(s.read_area(0x0900, 5 * 0x37).unwrap(), vec![9u8; 5 * 0x37]);
        assert!(started.elapsed() < Duration::from_secs(60), "took {:?}", started.elapsed());
    }

    #[test]
    fn reads_and_writes_speed_dial_against_a_pretend_phone() {
        let mut mem = vec![0u8; 0x10000];
        mem[0xFFF0..0xFFF8].copy_from_slice(b"E40.10\x03\x06");
        let mut phone = FakePhone { mem, out: vec![], inbuf: vec![], writing: false, repeat: false, garble_next: false, session_limit: None, session_count: 0, writes: vec![], mode: String::new() };
        let mut log = |_: &str| {};
        let never = || false;
        let mut s = Session::new(&mut phone, &mut log, &never);
        s.dial(&DialSetup { number: "555-010-1234".into(), init_addons: "L3".into(), type_addons: "S37=5L3".into(), dial_timeout: 75 })
            .unwrap();
        // The pretend phone's ID is 123456; it goes down the line as ~654321.
        assert!(s.log_in("654321").is_err());
        s.log_in("123456").unwrap();
        assert_eq!(firmware_text(&s.read(FIRMWARE_ADDR, FIRMWARE_LEN).unwrap()), "E40.10");
        let nums: Vec<String> = (0..10).map(|i| if i == 0 { "15856045476".into() } else { format!("{i}") }).collect();
        let bytes = speed_dial_to_bytes(&nums).unwrap();
        s.write_area(Map { d3: true }.speed_dial(), &bytes).unwrap();
        let back = s.read_area(Map { d3: true }.speed_dial(), bytes.len()).unwrap();
        assert_eq!(speed_dial_from_bytes(&back), nums);
        s.finish();
    }

    /// A made-up 64 KB secondary firmware file: 0xA0 bytes of first-half code (end 80A0 written
    /// at FFE0) and a second half naming E41.00 at FFF0.
    fn fake_firmware_file() -> Vec<u8> {
        let mut f = vec![0xFFu8; 0x10000];
        for (i, b) in f[..0xA0].iter_mut().enumerate() {
            *b = i as u8;
        }
        f[0x7FE0..0x7FE4].copy_from_slice(b"80A0");
        for (i, b) in f[0x8000..0xFFE0].iter_mut().enumerate() {
            *b = (i % 251) as u8;
        }
        f[0xFFF0..0xFFF6].copy_from_slice(b"E41.00");
        f
    }

    #[test]
    fn firmware_file_is_split_like_the_original() {
        let plan = firmware_plan(&fake_firmware_file()).unwrap();
        assert_eq!(plan.version, "E41.00");
        // !8: two 80-byte records of code at 8000, then the 32 bytes from FFE0.
        assert_eq!(plan.first.iter().map(|(a, d)| (*a, d.len())).collect::<Vec<_>>(), vec![(0x8000, 80), (0x8050, 80), (0xFFE0, 32)]);
        assert_eq!(&plan.first[2].1[..4], b"80A0");
        // !9: all of 8000-FFFF in 80-byte records, the last one 48 bytes.
        assert_eq!(plan.second.len(), 410);
        assert_eq!(plan.second.last().map(|(a, d)| (*a, d.len())), Some((0xFFD0, 48)));
        assert_eq!(plan.first.iter().chain(&plan.second).map(|(_, d)| d.len()).sum::<usize>(), 0xA0 + 32 + 0x8000);
        // A 32 KB chip image, or a file whose second half isn't E firmware, is refused.
        assert!(firmware_plan(&vec![0u8; 0x8000]).unwrap_err().starts_with("Wrong Firmware Version"));
        let mut d = fake_firmware_file();
        d[0xFFF0] = b'D';
        assert!(firmware_plan(&d).unwrap_err().starts_with("Wrong Firmware Version"));
    }

    #[test]
    fn firmware_goes_down_in_8_and_9_sessions() {
        let plan = firmware_plan(&fake_firmware_file()).unwrap();
        let mut phone = FakePhone {
            mem: vec![0u8; 0x10000],
            out: vec![],
            inbuf: vec![],
            writing: false,
            repeat: false,
            garble_next: false,
            session_limit: None,
            session_count: 0,
            writes: vec![],
            mode: String::new(),
        };
        let mut log = |_: &str| {};
        let never = || false;
        {
            let mut s = Session::new(&mut phone, &mut log, &never);
            s.write_records_in("!8", &plan.first, Some(("Downloading Firmware", 0, plan.records()))).unwrap();
            s.write_records_in("!9", &plan.second, Some(("Downloading Firmware", plan.first.len(), plan.records()))).unwrap();
            s.write_records(&[(RUN_SELECT_ADDR, vec![0x55])]).unwrap();
            assert_eq!(s.read(RUN_SELECT_ADDR, 1).unwrap(), vec![0x55]);
        }
        let w = &phone.writes;
        assert_eq!(w.len(), plan.records() + 1);
        assert!(w[..3].iter().all(|(m, ..)| m == "!8"));
        assert_eq!(w[2], ("!8".to_string(), 0xFFE0, 32));
        assert!(w[3..413].iter().all(|(m, ..)| m == "!9"));
        assert_eq!(w[413], ("!1".to_string(), 0x0A25, 1));
        assert_eq!(&phone.mem[0xFFF0..0xFFF6], b"E41.00");
    }
}

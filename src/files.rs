//! The program's data files. Record lengths come from the `Open ... Len =` statements in the
//! source; field layouts the decompiled code doesn't show are marked ESTIMATED.

use std::path::Path;

fn latin1(b: &[u8]) -> String {
    b.iter().map(|&c| c as char).collect()
}
fn bytes(s: &str) -> Vec<u8> {
    s.chars().map(|c| if (c as u32) < 256 { c as u8 } else { b'?' }).collect()
}
/// A fixed-length string field (`String * n`): cut or padded with spaces.
pub fn fixed(s: &str, n: usize) -> String {
    let mut t: String = s.chars().take(n).collect();
    while t.chars().count() < n {
        t.push(' ');
    }
    t
}

pub fn read_records(p: &Path, len: usize) -> Result<Vec<String>, String> {
    let b = std::fs::read(p).map_err(|e| e.to_string())?;
    Ok(b.chunks(len).map(latin1).collect())
}
pub fn write_records(p: &Path, recs: &[String], len: usize) -> Result<(), String> {
    let mut out = Vec::new();
    for r in recs {
        out.extend(bytes(&fixed(r, len)));
    }
    std::fs::write(p, out).map_err(|e| e.to_string())
}

// ---- Speed dial (.SDN): frmSpeedDial, `Open ... For Random Len = 14`, ten masked numbers.
pub fn read_sdn(p: &Path) -> Result<Vec<String>, String> {
    let mut r = read_records(p, 14)?;
    r.resize(10, String::new());
    Ok(r.into_iter().take(10).collect())
}
pub fn write_sdn(p: &Path, nums: &[String]) -> Result<(), String> {
    write_records(p, nums, 14)
}

// ---- CSN / UPT tables (.CSN): frmUPTTable `Len = 34`, the 17 bytes of an entry as hex.
pub fn read_csn(p: &Path) -> Result<Vec<Vec<u8>>, String> {
    let recs = read_records(p, 34)?;
    let mut out = Vec::new();
    for r in recs {
        let r = r.trim();
        if r.len() < 34 {
            continue;
        }
        let b: Option<Vec<u8>> = (0..17).map(|i| u8::from_str_radix(&r[i * 2..i * 2 + 2], 16).ok()).collect();
        match b {
            Some(b) => out.push(b),
            None => return Err("This is not a CSN file.".into()),
        }
    }
    Ok(out)
}
pub fn write_csn(p: &Path, entries: &[Vec<u8>]) -> Result<(), String> {
    let recs: Vec<String> = entries.iter().map(|e| e.iter().map(|b| format!("{b:02X}")).collect()).collect();
    write_records(p, &recs, 34)
}

// ---- AOS scripts (.AOS): frmAOSScript `Len = 2`: instruction codes as two hex characters up to
// and including "FF", then the phone number (7 records = 14 characters, as masked) and the
// authorization code (8 records = 16 characters).
pub struct AosFile {
    pub codes: Vec<String>,
    pub phone: String,
    pub auth: String,
}
pub fn read_aos(p: &Path) -> Result<AosFile, String> {
    let recs = read_records(p, 2)?;
    let mut it = recs.into_iter();
    let mut codes = Vec::new();
    for r in it.by_ref() {
        let done = r == "FF";
        codes.push(r);
        if done {
            break;
        }
    }
    let phone: String = it.by_ref().take(7).collect();
    let auth: String = it.take(8).collect();
    Ok(AosFile { codes, phone, auth })
}
pub fn write_aos(p: &Path, f: &AosFile) -> Result<(), String> {
    let mut recs: Vec<String> = f.codes.clone();
    if recs.last().map(|s| s.as_str()) != Some("FF") {
        recs.push("FF".into());
    }
    let ph = fixed(&f.phone, 14);
    let au = fixed(&f.auth, 16);
    for k in (0..14).step_by(2) {
        recs.push(ph.chars().skip(k).take(2).collect());
    }
    for k in (0..16).step_by(2) {
        recs.push(au.chars().skip(k).take(2).collect());
    }
    write_records(p, &recs, 2)
}

// ---- Scrolling message (.SCR): ScrollIt reads it `For Input` (one line).
pub fn read_scr(p: &Path) -> Result<String, String> {
    let b = std::fs::read(p).map_err(|e| e.to_string())?;
    let t = latin1(&b);
    Ok(t.lines().next().unwrap_or("").trim_matches('"').to_string())
}
pub fn write_scr(p: &Path, s: &str) -> Result<(), String> {
    std::fs::write(p, bytes(&format!("\"{s}\"\r\n"))).map_err(|e| e.to_string())
}

// ---- Phone list (.PLW): frmPhoneList `Len = Len(record)`. Fields from the Grid columns: phone,
// type, ID, firmware (6, from frmUpdatePhoneListEntry), address (25), city (15), state (2),
// zip (5), status (1 byte, a code), PIC. The other field widths are ESTIMATED.
pub const PLW_FIELDS: [usize; 10] = [14, 2, 6, 6, 25, 15, 2, 5, 1, 8];
pub const PLW_LEN: usize = 84;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Phone {
    pub phone: String,
    pub ptype: String,
    pub id: String,
    pub firmware: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub status: u8,
    pub pic: String,
}

impl Phone {
    fn from_rec(r: &str) -> Phone {
        let mut parts = Vec::new();
        let mut at = 0;
        let ch: Vec<char> = r.chars().collect();
        for n in PLW_FIELDS {
            let s: String = ch.iter().skip(at).take(n).collect();
            parts.push(s);
            at += n;
        }
        Phone {
            phone: parts[0].trim_end().into(),
            ptype: parts[1].trim_end().into(),
            id: parts[2].trim_end().into(),
            firmware: parts[3].trim_end().into(),
            address: parts[4].trim_end().into(),
            city: parts[5].trim_end().into(),
            state: parts[6].trim_end().into(),
            zip: parts[7].trim_end().into(),
            status: parts[8].chars().next().map(|c| c as u32 as u8).unwrap_or(0),
            pic: parts[9].trim_end().into(),
        }
    }
    fn to_rec(&self) -> String {
        let f = [&self.phone, &self.ptype, &self.id, &self.firmware, &self.address, &self.city, &self.state, &self.zip];
        let mut s = String::new();
        for (v, n) in f.iter().zip(PLW_FIELDS) {
            s.push_str(&fixed(v, n));
        }
        s.push(self.status as char);
        s.push_str(&fixed(&self.pic, 8));
        s
    }
    pub fn digits(&self) -> String {
        self.phone.chars().filter(|c| c.is_ascii_digit()).collect()
    }
}

pub fn read_plw(p: &Path) -> Result<Vec<Phone>, String> {
    Ok(read_records(p, PLW_LEN)?.iter().filter(|r| r.trim().len() > 0).map(|r| Phone::from_rec(r)).collect())
}
pub fn write_plw(p: &Path, list: &[Phone]) -> Result<(), String> {
    let recs: Vec<String> = list.iter().map(|x| x.to_rec()).collect();
    write_records(p, &recs, PLW_LEN)
}

/// fn024F: the text of a transfer status code (USA.INI [PHONE LIST] plST..plFlagged).
pub fn status_key(code: u8) -> &'static str {
    match code {
        1 => "plST",
        2 => "plTOE",
        3 => "plCE",
        4 => "plAE",
        5 => "plLCE",
        6 => "plDNC",
        7 => "plCBU",
        8 => "plSBU",
        9 => "plPWB",
        10 => "plPE",
        11 => "plWFV",
        12 => "plFlagged",
        _ => "plDNA",
    }
}
pub fn status_code_of(err: &str) -> u8 {
    let e = err.to_ascii_lowercase();
    if e.contains("successful") {
        1
    } else if e.contains("timed out") || e.contains("timer") {
        2
    } else if e.contains("checksum") {
        3
    } else if e.contains("acknowledge") || e.contains("incorrect id") {
        4
    } else if e.contains("carrier") {
        5
    } else if e.contains("busy") {
        9
    } else if e.contains("cancel") {
        7
    } else if e.contains("skipped") {
        8
    } else if e.contains("firmware") {
        11
    } else if e.contains("not connect") || e.contains("in use") {
        6
    } else {
        10
    }
}

// ---- Batch file (.BXW): frmBatchSetup, 13 records of a flag byte + a file name (String * 23):
// 1-10 the operations (Check1(0..9), Check2 in bit 2 for 0 and 1), 11 the phone list,
// 12 the scrolling message, 13 the firmware (bit 2 = Run After Download).
pub const BXW_LEN: usize = 24;
#[derive(Clone, Debug, Default)]
pub struct Batch {
    pub flags: Vec<u8>,
    pub files: Vec<String>,
}
impl Batch {
    pub fn new() -> Batch {
        Batch { flags: vec![0; 13], files: vec![String::new(); 13] }
    }
}
pub fn read_bxw(p: &Path) -> Result<Batch, String> {
    let recs = read_records(p, BXW_LEN)?;
    let mut b = Batch::new();
    for (i, r) in recs.iter().take(13).enumerate() {
        let mut ch = r.chars();
        b.flags[i] = ch.next().map(|c| c as u32 as u8).unwrap_or(0);
        let name: String = ch.collect();
        b.files[i] = if name.starts_with("AAAA") { String::new() } else { name.trim().to_string() };
    }
    Ok(b)
}
pub fn write_bxw(p: &Path, b: &Batch) -> Result<(), String> {
    let recs: Vec<String> = (0..13)
        .map(|i| {
            let name = if i == 9 { "A".repeat(23) } else { fixed(&b.files[i], 23) };
            format!("{}{}", b.flags[i] as char, name)
        })
        .collect();
    write_records(p, &recs, BXW_LEN)
}

// ---- Collection log (C:\TFW\CFW\mm-dd-yy.cfw), one record per phone polled. The decompiled
// code reads a phone field and two amounts; this layout is ESTIMATED: phone (14), ID (6),
// time (8), coin box cents (10), cumulative cents (10), refund calls (6), refund cents (10),
// then the phone's raw collection block as hex (70).
pub const CFW_LEN: usize = 134;
#[derive(Clone, Debug, Default)]
pub struct CollectionRec {
    pub phone: String,
    pub id: String,
    pub time: String,
    pub coin_box: u64,
    pub cumulative: u64,
    pub refund_calls: u64,
    pub refund_cents: u64,
    pub raw: Vec<u8>,
}
pub fn append_cfw(p: &Path, r: &CollectionRec) -> Result<(), String> {
    let hex: String = r.raw.iter().map(|b| format!("{b:02X}")).collect();
    let rec = format!(
        "{}{}{}{:010}{:010}{:06}{:010}{}",
        fixed(&r.phone, 14),
        fixed(&r.id, 6),
        fixed(&r.time, 8),
        r.coin_box,
        r.cumulative,
        r.refund_calls,
        r.refund_cents,
        fixed(&hex, 70)
    );
    let mut b = std::fs::read(p).unwrap_or_default();
    b.extend(bytes(&rec));
    std::fs::write(p, b).map_err(|e| e.to_string())
}
pub fn read_cfw(p: &Path) -> Result<Vec<CollectionRec>, String> {
    let recs = read_records(p, CFW_LEN)?;
    Ok(recs
        .into_iter()
        .filter(|r| r.len() >= 64)
        .map(|r| {
            let s = |a: usize, n: usize| -> String { r.chars().skip(a).take(n).collect() };
            let hex = s(64, 70);
            CollectionRec {
                phone: s(0, 14).trim().into(),
                id: s(14, 6).trim().into(),
                time: s(20, 8).trim().into(),
                coin_box: s(28, 10).trim().parse().unwrap_or(0),
                cumulative: s(38, 10).trim().parse().unwrap_or(0),
                refund_calls: s(48, 6).trim().parse().unwrap_or(0),
                refund_cents: s(54, 10).trim().parse().unwrap_or(0),
                raw: (0..hex.trim().len() / 2).filter_map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok()).collect(),
            }
        })
        .collect())
}

/// SMDR log (C:\TFW\SMD\mm-dd-yy.SMD), Module1 sub0479: 50-character records appended, a header
/// with the phone, then each call as 50 hex characters.
pub fn append_smd(p: &Path, phone: &str, records: &[String]) -> Result<(), String> {
    let mut b = std::fs::read(p).unwrap_or_default();
    b.extend(bytes(&crate::proto::smdr_header(phone)));
    for r in records {
        b.extend(bytes(&fixed(r, 50)));
    }
    std::fs::write(p, b).map_err(|e| e.to_string())
}

/// Flag files (.FLG and the new-flags file PLW\_BITFLGS.FLX): lines of
/// `phone,type-id,flag,location,date time` (ESTIMATED layout).
pub fn read_flags(p: &Path) -> Vec<Vec<String>> {
    std::fs::read(p)
        .map(|b| latin1(&b))
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.split(',').map(|s| s.trim().trim_matches('"').to_string()).collect())
        .collect()
}

//! Programmable features files (.FEA): fixed 170-byte records, read and written byte for byte.
//! Layout and value encodings are documented in MECHANICS.md.

pub const RECORD: usize = 170;
const NUM: usize = 3;
const NAME: usize = 20;
const VALUE_AT: usize = 29;
const VALUE_MAX: usize = 16;
const HELP_AT: usize = 45;
const HELP: usize = 125;
pub const FIRMWARE_ADDR: u16 = 0xFFF0;

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    /// One digit per byte (phone numbers, IDs, access codes).
    Digits,
    /// Packed BCD money with two decimals.
    Money,
    /// Packed BCD number.
    Number,
    /// Two-way choice stored as 00 / 01.
    Choice(&'static str, &'static str),
    /// ASCII text (firmware version), read only.
    Text,
}

#[derive(Clone, Debug)]
pub struct Feature {
    raw: [u8; RECORD],
    pub tagged: bool,
}

impl Feature {
    pub fn number(&self) -> String {
        latin1(&self.raw[0..NUM]).trim_end().to_string()
    }
    pub fn name(&self) -> String {
        latin1(&self.raw[NUM..NUM + NAME]).trim_end_matches(['_', ' ']).to_string()
    }
    pub fn address(&self) -> u16 {
        u16::from_le_bytes([self.raw[23], self.raw[24]])
    }
    pub fn len(&self) -> usize {
        (u16::from_le_bytes([self.raw[27], self.raw[28]]) as usize).min(VALUE_MAX)
    }
    pub fn help(&self) -> String {
        latin1(&self.raw[HELP_AT..HELP_AT + HELP])
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
    pub fn value_bytes(&self) -> &[u8] {
        &self.raw[VALUE_AT..VALUE_AT + self.len()]
    }
    pub fn read_only(&self) -> bool {
        let n = self.name().to_ascii_lowercase();
        self.address() == FIRMWARE_ADDR || n.contains("serial number")
    }

    pub fn kind(&self) -> Kind {
        let len = self.len();
        let name = self.name().to_ascii_lowercase();
        let help = self.help().to_ascii_lowercase();
        if self.address() == FIRMWARE_ADDR {
            return Kind::Text;
        }
        if len >= 6 {
            return Kind::Digits;
        }
        if len == 1 {
            if name.contains("callback method") {
                return Kind::Choice("Voice", "Modem");
            }
            if name.contains("dtmf or pulse") {
                return Kind::Choice("DTMF", "Pulse");
            }
            if name.contains("dtmf speed") {
                return Kind::Choice("Fast", "Slow");
            }
            let v = self.value_bytes()[0];
            let yes_no = name.ends_with('?')
                || help.starts_with("answer yes")
                || help.starts_with("enable")
                || help.starts_with("yes ")
                || help.starts_with("used to allow");
            if yes_no && v <= 1 {
                return Kind::Choice("No", "Yes");
            }
        }
        let money_word = ["amount", "amt", "charge", "chrg", "total", "sur "];
        if money_word.iter().any(|w| name.contains(w))
            && !name.contains("calls")
            && !name.contains("time")
            && !name.contains("per")
        {
            return Kind::Money;
        }
        Kind::Number
    }

    /// The value as the person reads it.
    pub fn display(&self) -> String {
        let b = self.value_bytes();
        match self.kind() {
            Kind::Text => latin1(b).trim().to_string(),
            Kind::Digits => b.iter().map(|&d| digit_char(d)).collect(),
            Kind::Choice(a, c) => match b[0] {
                0 => a.to_string(),
                1 => c.to_string(),
                v => format!("{v:02X}"),
            },
            Kind::Money => {
                let s = bcd_string(b);
                if s.chars().all(|c| c == '*') {
                    return s;
                }
                if s.len() <= 2 {
                    format!("$.{s}")
                } else {
                    let (whole, cents) = s.split_at(s.len() - 2);
                    format!("${whole}.{cents}")
                }
            }
            Kind::Number => bcd_string(b),
        }
    }

    /// The editable form: the digits/characters without $ and dots.
    pub fn edit_text(&self) -> String {
        let b = self.value_bytes();
        match self.kind() {
            Kind::Digits => b.iter().map(|&d| digit_char(d)).collect(),
            Kind::Text => latin1(b).trim().to_string(),
            Kind::Choice(..) => format!("{:02X}", b[0]),
            Kind::Money | Kind::Number => bcd_string(b),
        }
    }

    /// How many characters the editable form must have.
    pub fn edit_width(&self) -> usize {
        match self.kind() {
            Kind::Digits => self.len(),
            Kind::Money | Kind::Number => self.len() * 2,
            Kind::Choice(..) => 2,
            Kind::Text => self.len(),
        }
    }

    /// Set the value from its edit form. Returns a plain-words problem when it doesn't fit.
    pub fn set_from_text(&mut self, text: &str) -> Result<(), String> {
        if self.read_only() {
            return Err("This option can't be changed.".into());
        }
        let len = self.len();
        let text: String = text.chars().filter(|c| !matches!(c, '-' | '$' | '.' | ' ' | ':')).collect();
        let bytes: Vec<u8> = match self.kind() {
            Kind::Digits => {
                let mut t: Vec<char> = text.chars().collect();
                if t.len() > len {
                    return Err(format!("Enter at most {len} digits."));
                }
                // Fill from the left with * like the original ("filling in from the left with astericks").
                while t.len() < len {
                    t.insert(0, '*');
                }
                let mut out = Vec::with_capacity(len);
                for c in t {
                    out.push(char_digit(c).ok_or("Only 0-9, * and # are allowed.")?);
                }
                out
            }
            Kind::Money | Kind::Number => {
                let mut t: Vec<char> = text.chars().collect();
                if t.len() > len * 2 {
                    return Err(format!("Enter at most {} digits.", len * 2));
                }
                while t.len() < len * 2 {
                    t.insert(0, if t.iter().all(|&c| c == '*') && !t.is_empty() { '*' } else { '0' });
                }
                let mut out = Vec::with_capacity(len);
                for pair in t.chunks(2) {
                    let hi = nibble(pair[0]).ok_or("Only digits or * are allowed.")?;
                    let lo = nibble(pair[1]).ok_or("Only digits or * are allowed.")?;
                    out.push(hi << 4 | lo);
                }
                out
            }
            Kind::Choice(..) => {
                let v = u8::from_str_radix(&text, 16).map_err(|_| "Choose one of the two settings.")?;
                vec![v]
            }
            Kind::Text => return Err("This option can't be changed.".into()),
        };
        self.raw[VALUE_AT..VALUE_AT + len].copy_from_slice(&bytes);
        Ok(())
    }

    /// Put bytes read from the phone into the value (exactly `len()` bytes).
    pub fn set_value_bytes(&mut self, b: &[u8]) {
        let n = self.len().min(b.len());
        self.raw[VALUE_AT..VALUE_AT + n].copy_from_slice(&b[..n]);
    }

    pub fn set_choice(&mut self, second: bool) {
        self.raw[VALUE_AT] = u8::from(second);
    }
}

#[derive(Clone, Debug, Default)]
pub struct FeatureFile {
    pub features: Vec<Feature>,
}

impl FeatureFile {
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.is_empty() {
            return Err("The file you have chosen is empty.".into());
        }
        if bytes.len() % RECORD != 0 || bytes[0..3].iter().any(|b| !b.is_ascii_graphic() && *b != b' ') {
            return Err("This is not a .FEA file.".into());
        }
        let features = bytes
            .chunks(RECORD)
            .map(|c| {
                let mut raw = [0u8; RECORD];
                raw.copy_from_slice(c);
                Feature { raw, tagged: false }
            })
            .collect();
        Ok(Self { features })
    }

    pub fn to_bytes(&self, only_tagged: bool) -> Vec<u8> {
        self.features
            .iter()
            .filter(|f| !only_tagged || f.tagged || f.address() == FIRMWARE_ADDR)
            .flat_map(|f| f.raw.iter().copied())
            .collect()
    }

    pub fn firmware(&self) -> String {
        self.features
            .iter()
            .find(|f| f.address() == FIRMWARE_ADDR)
            .map(|f| f.display())
            .unwrap_or_default()
    }
}

fn latin1(b: &[u8]) -> String {
    b.iter().map(|&c| if c.is_ascii_control() { ' ' } else { c as char }).collect()
}

fn digit_char(d: u8) -> char {
    match d {
        0..=9 => (b'0' + d) as char,
        0x0A => '*',
        0x0C => '#',
        0x0B => 'B',
        0x0D => 'D',
        0x0E => 'E',
        0x0F => 'F',
        _ => '?',
    }
}

fn char_digit(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        '*' => Some(0x0A),
        '#' => Some(0x0C),
        'B' | 'b' => Some(0x0B),
        'D' | 'd' => Some(0x0D),
        'E' | 'e' => Some(0x0E),
        'F' | 'f' => Some(0x0F),
        _ => None,
    }
}

fn nibble(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        '*' | 'A' | 'a' => Some(0x0A),
        'B'..='F' => Some(c as u8 - b'A' + 10),
        'b'..='f' => Some(c as u8 - b'a' + 10),
        _ => None,
    }
}

fn bcd_string(b: &[u8]) -> String {
    let mut s = String::new();
    for &v in b {
        for n in [v >> 4, v & 0x0F] {
            s.push(match n {
                0..=9 => (b'0' + n) as char,
                0x0A => '*',
                _ => (b'A' + n - 10) as char,
            });
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(num: &str, name: &str, addr: u16, len: u16, value: &[u8], help: &str) -> Vec<u8> {
        let mut r = vec![b' '; RECORD];
        r[0..3].copy_from_slice(format!("{num:<3}").as_bytes());
        let n = format!("{name:_<20}");
        r[3..23].copy_from_slice(&n.as_bytes()[..20]);
        r[23..25].copy_from_slice(&addr.to_le_bytes());
        r[25..27].copy_from_slice(&0u16.to_le_bytes());
        r[27..29].copy_from_slice(&len.to_le_bytes());
        r[29..29 + value.len()].copy_from_slice(value);
        r[45..45 + help.len()].copy_from_slice(help.as_bytes());
        r
    }

    /// With FEA_DIR set to a folder of real .FEA files: every file reads and writes back unchanged.
    #[test]
    fn real_files_round_trip() {
        let Ok(dir) = std::env::var("FEA_DIR") else { return };
        let mut n = 0;
        for e in std::fs::read_dir(dir).unwrap().flatten() {
            let p = e.path();
            if !p.extension().is_some_and(|x| x.eq_ignore_ascii_case("fea")) {
                continue;
            }
            let bytes = std::fs::read(&p).unwrap();
            let f = FeatureFile::parse(&bytes).unwrap();
            assert_eq!(f.to_bytes(false), bytes, "{}", p.display());
            assert!(!f.firmware().is_empty(), "{}", p.display());
            for feat in &f.features {
                let _ = (feat.display(), feat.kind(), feat.help());
                if !feat.read_only() && !matches!(feat.kind(), Kind::Choice(..)) {
                    let mut c = feat.clone();
                    c.set_from_text(&feat.edit_text()).unwrap_or_else(|e| panic!("{} {}: {e}", p.display(), feat.name()));
                    assert_eq!(c.value_bytes(), feat.value_bytes(), "{} {}", p.display(), feat.name());
                }
            }
            n += 1;
        }
        eprintln!("checked {n} FEA files");
        assert!(n > 0);
    }

    #[test]
    fn round_trip_and_values() {
        let mut bytes = rec("14g", "*123 Alternate Nmber", 0x0A45, 11, &[1, 8, 0, 0, 8, 4, 3, 5, 6, 7, 8], "Number");
        bytes.extend(rec("05", "Maximum Box Amount", 0x083D, 3, &[0x01, 0x50, 0x00], "When"));
        bytes.extend(rec("06", "Variable Rates?", 0x0843, 1, &[1], "Used to allow"));
        bytes.extend(rec("48a", "Local Initial Amount", 0x0906, 2, &[0xAA, 0xAA], "Override"));
        let mut f = FeatureFile::parse(&bytes).unwrap();
        assert_eq!(f.features[0].display(), "18008435678");
        assert_eq!(f.features[1].display(), "$0150.00");
        assert_eq!(f.features[2].display(), "Yes");
        assert_eq!(f.features[3].display(), "****");
        assert_eq!(f.to_bytes(false), bytes);
        f.features[1].set_from_text("$0200.25").unwrap();
        assert_eq!(f.features[1].value_bytes(), &[0x02, 0x00, 0x25]);
        f.features[0].set_from_text("411").unwrap();
        assert_eq!(f.features[0].display(), "********411");
    }
}

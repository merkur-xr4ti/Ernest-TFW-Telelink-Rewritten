//! A small INI reader/writer that keeps the file's own lines, so TFW.INI stays readable by
//! the original program too.

#[derive(Clone, Debug, Default)]
pub struct Ini {
    lines: Vec<String>,
}

impl Ini {
    pub fn parse(text: &str) -> Self {
        Self { lines: text.lines().map(|l| l.to_string()).collect() }
    }

    pub fn to_text(&self) -> String {
        let mut s = self.lines.join("\r\n");
        s.push_str("\r\n");
        s
    }

    fn find(&self, section: &str, key: &str) -> (Option<usize>, Option<usize>) {
        let mut in_section = false;
        let mut section_at = None;
        let mut last_in_section = None;
        for (i, line) in self.lines.iter().enumerate() {
            let t = line.trim();
            if t.starts_with('[') && t.ends_with(']') {
                in_section = t[1..t.len() - 1].trim().eq_ignore_ascii_case(section);
                if in_section {
                    section_at = Some(i);
                    last_in_section = Some(i);
                }
                continue;
            }
            if in_section {
                if !t.is_empty() {
                    last_in_section = Some(i);
                }
                if let Some((k, _)) = t.split_once('=') {
                    if k.trim().eq_ignore_ascii_case(key) {
                        return (section_at, Some(i));
                    }
                }
            }
        }
        (last_in_section, None)
    }

    pub fn get(&self, section: &str, key: &str) -> Option<String> {
        let (_, at) = self.find(section, key);
        at.and_then(|i| self.lines[i].split_once('=').map(|(_, v)| v.trim().to_string()))
    }


    pub fn set(&mut self, section: &str, key: &str, value: &str) {
        match self.find(section, key) {
            (_, Some(i)) => self.lines[i] = format!("{key}={value}"),
            (Some(last), None) => self.lines.insert(last + 1, format!("{key}={value}")),
            (None, None) => {
                if self.lines.last().is_some_and(|l| !l.trim().is_empty()) {
                    self.lines.push(String::new());
                }
                self.lines.push(format!("[{section}]"));
                self.lines.push(format!("{key}={value}"));
            }
        }
    }
}

/// The settings the original shipped with (TFW.INI, 1998-07-17).
#[cfg(test)]
const DEFAULT_TFW_INI: &str = "[BatchAutoPrint]\r\nUnpolled=1\r\nMinimum=0\r\nMonthlyTotals=0\r\nFlags=0\r\nMDA=2\r\n\r\n[Modem]\r\nSerialPort=2\r\nDelay=550\r\nModemInitAddons=L3\r\nModemD3InitAddons=S37=5L3\r\nModemD1InitAddons=S37=3L3\r\n\r\n[System]\r\nUserSelectedFontSize= 12\r\nCountry=USA\r\nSound=NO\r\n\r\n[Dialtime]\r\nLocal=70\r\nOnePlus=121\r\n\r\n[DialingDefaults]\r\nPBX=\r\nDCW=\r\n\r\n[Language]\r\nInternational=OFF\r\n";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keeps_values_with_equals() {
        let mut ini = Ini::parse(DEFAULT_TFW_INI);
        assert_eq!(ini.get("Modem", "ModemD3InitAddons").unwrap(), "S37=5L3");
        assert_eq!(ini.get("System", "UserSelectedFontSize").unwrap(), "12");
        ini.set("Modem", "SerialPort", "3");
        ini.set("AutoCall", "AutoStart", "YES");
        let again = Ini::parse(&ini.to_text());
        assert_eq!(again.get("Modem", "SerialPort").unwrap(), "3");
        assert_eq!(again.get("AutoCall", "AutoStart").unwrap(), "YES");
    }
}

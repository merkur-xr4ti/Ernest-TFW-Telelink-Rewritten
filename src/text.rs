//! GetPrivateProfileString / WritePrivateProfileString as Module4 uses them:
//! `fn059F`/`fn056E` read and write the program's own TFW.INI, `fn0589` reads the caption table
//! (C:\TFW\TEXT\USA.INI, or CHINA/TAIWAN.INI when [Language] International=ON).

use crate::ini::Ini;
use std::path::PathBuf;

/// Where the original kept everything (`C:\TFW`). TFW_ROOT moves it (used for testing).
pub fn root() -> PathBuf {
    std::env::var("TFW_ROOT").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("C:\\TFW"))
}

pub fn path(rel: &str) -> PathBuf {
    root().join(rel)
}

fn read_latin1(p: &PathBuf) -> Option<String> {
    std::fs::read(p).ok().map(|b| b.iter().map(|&c| c as char).collect())
}

/// Windows strips one pair of surrounding quotes from a profile value.
fn profile_value(v: &str) -> String {
    let v = v.trim();
    if v.len() >= 2 && v.starts_with('"') && v.ends_with('"') {
        v[1..v.len() - 1].to_string()
    } else {
        v.to_string()
    }
}

/// The program's TFW.INI (App.Path & "\" & App.EXEName & ".INI"), read fresh on each call like
/// the Windows profile functions.
#[derive(Clone)]
pub struct Profile {
    pub file: PathBuf,
}

impl Profile {
    pub fn new() -> Self {
        Self { file: path("TFW.INI") }
    }
    fn load(&self) -> Ini {
        Ini::parse(&read_latin1(&self.file).unwrap_or_default())
    }
    /// fn059F: the value, or "" when it isn't there.
    pub fn get(&self, section: &str, key: &str) -> String {
        self.load().get(section, key).map(|v| profile_value(&v)).unwrap_or_default()
    }
    /// fn056E: write a value; false when the file can't be written.
    pub fn set(&self, section: &str, key: &str, value: &str) -> bool {
        let mut ini = self.load();
        ini.set(section, key, value);
        let bytes: Vec<u8> = ini.to_text().chars().map(|c| if (c as u32) < 256 { c as u8 } else { b'?' }).collect();
        std::fs::write(&self.file, bytes).is_ok()
    }
}

/// The caption table.
pub struct TextTable {
    ini: Option<Ini>,
}

impl TextTable {
    pub fn load(international: bool, country: &str) -> Self {
        let file = if international {
            match country.to_ascii_uppercase().as_str() {
                "CHINA" => "TEXT\\CHINA.INI",
                "TAIWAN" => "TEXT\\TAIWAN.INI",
                _ => "TEXT\\USA.INI",
            }
        } else {
            "TEXT\\USA.INI"
        };
        Self { ini: read_latin1(&path(file)).map(|t| Ini::parse(&t)) }
    }
    /// fn0589: the text for a key; "NO STRING!" when the table has no such key, like the
    /// original; `fallback` when there's no table at all.
    pub fn t(&self, section: &str, key: &str, fallback: &str) -> String {
        match &self.ini {
            Some(ini) => ini.get(section, key).map(|v| profile_value(&v)).unwrap_or_else(|| "NO STRING!".into()),
            None => fallback.to_string(),
        }
    }
    /// Like `t`, but keeps the form's design-time caption when the key is missing.
    pub fn or(&self, section: &str, key: &str, design: &str) -> String {
        match &self.ini {
            Some(ini) => ini.get(section, key).map(|v| profile_value(&v)).unwrap_or_else(|| design.to_string()),
            None => design.to_string(),
        }
    }
}

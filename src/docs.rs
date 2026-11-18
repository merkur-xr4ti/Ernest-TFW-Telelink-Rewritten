//! Fixed lists the original program shows (frmAOSScript menus, frmRateTable classes).

pub const AOS_INSTRUCTIONS: [&str; 13] = [
    "1 - Dial AOS Phone #",
    "2 - Dial AOS Auth. Code",
    "3 - Dial ANI",
    "4 - Dial DTMF Digit",
    "5 - Dial Destination + Home NPA",
    "6 - Dial Destination",
    "7 - Wait for Dialtone/400Hz",
    "8 - Pause",
    "9 - Wait for DTMF from AOS",
    "10 - Enable Earpiece",
    "11 - Send Credit Card",
    "12 - Perform Answer Detection",
    "13 - Speak \"Stand By\" MSG",
];
pub const DTMF_KEYS: [&str; 12] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "*", "0", "#"];
pub const PAUSES: [&str; 9] = [
    "0.1 Seconds",
    "0.2 Seconds",
    "0.3 Seconds",
    "0.4 Seconds",
    "0.5 s/1 sec",
    "0.6 s/2 sec",
    "0.7 s/4 sec",
    "0.8 s/8 sec",
    "0.9 s/16 sec",
];
pub const CARD_PARTS: [&str; 5] = ["Number", "Expiry Date MM/YR", "Expiry Date Month", "Expiry Date Year", "PIN"];

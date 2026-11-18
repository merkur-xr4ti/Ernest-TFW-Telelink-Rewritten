//! TELELINK.FRM (the main window) and the small message forms around it: frmAbout (the splash
//! shown while loading), frmVersionInfo, frmModemSearch, frmNoModems and frmMessage.

use crate::app::App;
use crate::modem::LineChoice;
use crate::vb::{Answer, Ev, MenuItem};
use chrono::{Datelike, Local, Timelike};
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct MainState {
    pub last_tick: Option<Instant>,
    /// tmrAutoCall statics: the day whose schedule is loaded, its run time, the batch files.
    pub ac_day: String,
    pub ac_at: String,
    pub ac_loaded_at: String,
    pub ac_files: Vec<String>,
    pub ac_retry: String,
    pub aa_day: String,
    pub aa_d3: (String, String),
    pub aa_d1: (String, String),
    /// Start-up modem check: when the port was opened, and the modem search's next port.
    pub modem_check: Option<Instant>,
    pub search_port: i32,
    pub search_at: Option<Instant>,
    pub blink: bool,
    pub splash_until: Option<Instant>,
    pub was_connected: bool,
    /// frmMessage is up showing a transfer's progress.
    pub transfer_msg: bool,
}

const SEC: &str = "TELELINK";

pub fn load(app: &mut App, name: &str) {
    match name {
        "Telelink" => telelink_load(app),
        "frmAbout" | "frmSplash" => {
            let w = app.tor(SEC, "vWelcomeToTelelink", "Welcome to Telelink");
            let t = app.tor(SEC, "vTelelinkForWindows", "Telelink for Windows");
            let v = format!("{} {}", app.tor(SEC, "vVersion", "Version"), app.tor(SEC, "version", "2.61"));
            let f = app.form(name);
            f.back = eframe::egui::Color32::from_rgb(0, 0, 128);
            f.c("Label1").caption = w;
            f.c("Label2").caption = t;
            f.c("Label3").caption = v;
            for l in ["Label1", "Label2", "Label3"] {
                f.c(l).fore = Some(eframe::egui::Color32::WHITE);
            }
            // Added for this rebuild: what it is and how it was made, below the version line.
            f.client_h += 480.0;
            let l3 = f.c("Label3");
            l3.height = 860.0;
            l3.caption = format!("{}\n\nThis is a Rust rewrite of decompiled code, completed with the use of AI.", l3.caption);
        }
        "frmVersionInfo" => {
            let t = app.tor(SEC, "vTelelinkForWindows", "Telelink for Windows");
            let v = format!("{} {}", app.tor(SEC, "vVersion", "Version"), app.tor(SEC, "version", "2.61"));
            let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
            let ini = app.prof.file.display().to_string();
            let f = app.form(name);
            f.c("Label2").caption = String::new();
            f.c("Label1").caption = t;
            f.c("Label3").caption = v;
            f.c("Label4").caption = exe;
            f.c("Label5").caption = ini;
        }
        "frmModemSearch" => {
            let s = app.t(SEC, "msSearching", "Searching for an available modem......");
            let f = app.form(name);
            let _ = s; // the table's own text for this label is a placeholder; the form keeps its caption
            f.c("lblPort").width = 3000.0;
            f.c("lblPort").left = 2160.0;
        }
        "frmNoModems" => {
            let t = app.t(SEC, "fcNoModemFoundTitle", "No modem Available");
            let f = app.form(name);
            f.caption = t;
        }
        "frmMessage" => {}
        _ => {}
    }
}

fn menus(app: &App) -> Vec<MenuItem> {
    let t = |k: &str, d: &str| app.tor(SEC, k, d);
    let mut util = MenuItem::sub("control67", t("vUtilities", "Utilities"), vec![]);
    // [Utilities] UtilCount, 1=caption,command ... (control68 array).
    let n: i32 = app.prof.get("Utilities", "UtilCount").trim().parse().unwrap_or(0);
    for k in 1..=n {
        let v = app.prof.get("Utilities", &k.to_string());
        if let Some((cap, cmd)) = v.split_once(',') {
            util.children.push(MenuItem::new(&format!("control68:{}", cmd.trim()), cap.trim()));
        }
    }
    if util.children.is_empty() {
        let mut none = MenuItem::new("control68", t("vNoneInstalled", "None Installed"));
        none.enabled = false;
        util.children.push(none);
        util.enabled = false;
    }
    let mut rte = MenuItem::new("control66", t("vRateTableEditor", "Rate Table Editor"));
    rte.visible = std::path::Path::new("C:\\TFW\\RTE2.EXE").exists();
    vec![
        MenuItem::sub("control25", t("vFile", "File"), vec![MenuItem::new("control26", t("vAboutTelelink", "About Telelink")), MenuItem::sep(), MenuItem::new("control28", t("vExit", "Exit"))]),
        MenuItem::sub(
            "control29",
            t("vDirect", "Direct"),
            vec![
                MenuItem::new("control30", t("vHangUp", "Hang Up!")),
                MenuItem::sep(),
                MenuItem::new("control32", t("vAOSScripts", "AOS Scripts")),
                MenuItem::new("control33", t("vCollectionStatus", "Collection Status")),
                MenuItem::new("control34", t("vCSNTable", "CSN Table")),
                MenuItem::new("control35", t("vProgrammableFeatures", "Programmable Features")),
                MenuItem::new("control36", t("vRateTable", "Rate Table")),
                MenuItem::new("control37", t("vSMDR", "SMDR")),
                MenuItem::new("control38", t("vSpeedDial", "Speed Dial")),
                MenuItem::new("control39", t("vTimeAndDate", "Time and Date")),
                MenuItem::new("control40", "Unit Price Table (UPT)"),
                MenuItem::new("control41", t("vScrollingMessage", "Scrolling Message")),
                MenuItem::new("control42", t("vFirmware", "Firmware")),
            ],
        ),
        MenuItem::sub(
            "control43",
            t("vBatch", "Batch"),
            vec![
                MenuItem::new("control44", t("vPhoneList", "Phone List")),
                MenuItem::sep(),
                MenuItem::new("control46", t("vSetup", "Setup")),
                MenuItem::new("control47", t("vExecute", "Execute")),
                MenuItem::new("control48", t("vViewBatchLog", "View Batch Log")),
            ],
        ),
        MenuItem::sub("control49", t("vIncomming", "Incoming"), vec![MenuItem::new("control50", t("vViewIncoming", "View Incoming"))]),
        MenuItem::sub(
            "control51",
            t("vSettings", "Settings"),
            vec![
                MenuItem::new("control52", t("vAutoCallSchedule", "AutoCall Schedule")),
                MenuItem::new("control54", t("vAutoAnswerSchedule", "Auto Answer Schedule")),
                MenuItem::sep(),
                MenuItem::new("control56", t("vSystemConfiguration", "System Configuration")),
            ],
        ),
        MenuItem::sub(
            "control60",
            t("vReports", "Reports"),
            vec![MenuItem::new("control61", "Call Detail Records"), MenuItem::new("control62", t("vReport", "Report"))],
        ),
        MenuItem::sub("control63", t("vFlags", "Flags"), vec![MenuItem::new("control64", t("vViewNewFlags", "View New Flags")), MenuItem::new("control65", t("vReviewFlags", "Review Flags"))]),
        rte,
        util,
        MenuItem::new("control69", t("vQuickExit", "Quick Exit")),
    ]
}

/// Telelink Form_Load.
fn telelink_load(app: &mut App) {
    let p = app.prof.clone();
    // BatchAutoPrint defaults (written back when missing, as the original did).
    let ap = |key: &str, ok: &dyn Fn(&str) -> bool, def: &str| -> String {
        let v = p.get("BatchAutoPrint", key);
        if ok(&v) {
            v
        } else {
            p.set("BatchAutoPrint", key, def);
            def.to_string()
        }
    };
    app.g.ap_unpolled = ap("Unpolled", &|v| v == "1", "0");
    app.g.ap_minimum = ap("Minimum", &|v| !v.is_empty(), "0");
    app.g.ap_monthly = ap("MonthlyTotals", &|v| v == "1", "0");
    app.g.ap_flags = ap("Flags", &|v| v == "1", "0");
    app.g.ap_mda = ap("MDA", &|v| !v.is_empty(), "2.00");
    app.g.font_size = p.get("System", "UserSelectedFontSize").trim().parse().unwrap_or(10);
    let mut country = p.get("System", "Country");
    if country.is_empty() {
        country = "USA".into();
    }
    app.g.country = country.to_ascii_uppercase();
    app.g.usa = app.g.country == "USA";
    app.g.plw = "plw".into();
    app.g.international = p.get("Language", "International");
    app.g.dial_ld = p.get("DialTime", "OnePlus").trim().parse().unwrap_or(121);
    app.g.dial_local = p.get("DialTime", "Local").trim().parse().unwrap_or(121);
    let snd = p.get("System", "Sound").to_ascii_uppercase();
    app.g.sound = matches!(snd.as_str(), "ON" | "YES" | "TRUE");
    app.g.serial_port = p.get("Modem", "SerialPort").trim().parse().unwrap_or(0);
    app.g.delay = p.get("Modem", "Delay").trim().parse().unwrap_or(600);
    app.g.pbx = p.get("DialingDefaults", "PBX");
    app.g.dcw = p.get("DialingDefaults", "DCW");
    init_strings(app);

    // sub0941: captions from the text table.
    app.g.title = app.tor(SEC, "msTitle", "Telelink");
    app.g.on = app.tor(SEC, "gvON", "ON");
    app.g.off = app.tor(SEC, "gvOFF", "OFF");
    app.g.yes = app.tor(SEC, "gvYES", "YES");
    app.g.no = app.tor(SEC, "gvNO", "NO");
    app.g.d1 = app.tor(SEC, "gvD1", "D1");
    app.g.d3 = app.tor(SEC, "gvD3", "D3");
    app.g.not_connected = app.tor(SEC, "gvNotConnectedToAPhone", "Not Connected To A Phone");
    let m = menus(app);
    let caps: Vec<(String, i32, String)> = vec![
        ("lblAlert".into(), 0, app.tor(SEC, "vCheckFlags", "Check Flags!!")),
        ("lblAlert".into(), 1, app.tor(SEC, "vCheckFlags", "Check Flags!!")),
        ("Frame3D1".into(), 3, app.tor(SEC, "vNextAutoCallRun", "Next AutoCall Run")),
        ("Frame3D1".into(), 8, app.tor(SEC, "vAutoAnswerSchedule", "Auto Answer Schedule")),
        ("lblAutoCall".into(), -1, app.tor(SEC, "vAutoCall", "AutoCall")),
        ("lblAutoAnswer".into(), 0, app.tor(SEC, "vAutoAnswer", "Auto Answer")),
        ("lblIncoming".into(), -1, app.tor(SEC, "vAcceptIncoming", "Accept Incoming")),
        ("lblPhoneType".into(), -1, app.tor(SEC, "vModemIsSetFor", "Modem is set for")),
        ("lblConnection".into(), -1, app.g.not_connected.clone()),
    ];
    let hang = app.tor(SEC, "vHangUp", "Hang Up!");
    let (title, off, no, d3) = (app.g.title.clone(), app.g.off.clone(), app.g.no.clone(), app.g.d3.clone());
    let f = app.form("Telelink");
    f.menus = m;
    f.caption = title;
    for (n, i, c) in caps {
        if let Some(k) = f.find(&n, i) {
            if f.ctls[k].kind == crate::vb::Kind::SSFrame {
                f.ctls[k].text = c;
            } else {
                f.ctls[k].caption = c;
            }
        }
    }
    f.c("cmdHangUp").text = hang;
    f.c("cmdHangUp").visible = false;
    f.c("cmdAutoCall").text = off.clone();
    f.c("cmdAutoAnswer").text = off;
    f.c("cmdAcceptIncoming").text = no;
    f.c("cmdPhoneType").text = d3;
    f.ci("Frame3D1", 3).visible = false;
    f.ci("Frame3D1", 8).visible = false;
    f.ci("Frame3D1", 6).visible = false;
    for k in 0..=2 {
        f.ci("lblToday", k).caption = String::new();
    }
    f.visible = true;
    let israel = app.g.country == "ISRAEL";
    let usa = app.g.usa;
    let f = app.form("Telelink");
    // Country rules of Form_Load: Unit Price Table only in Israel; Call Detail Records and
    // Scrolling Message only in the USA; AOS Scripts and Speed Dial in the USA or Mexico.
    f.menu_enable("control40", israel);
    f.menu_enable("control61", usa);
    if !usa {
        f.menu_enable("control38", false);
        f.menu_enable("control41", false);
        if app.g.country != "MEXICO" {
            app.form("Telelink").menu_enable("control32", false);
        }
    }

    // AutoCall / AutoAnswer AutoStart.
    if p.get("AutoCall", "AutoStart").trim() == "1" {
        set_auto_call(app, true);
    }
    if p.get("AutoAnswer", "AutoStart").trim() == "1" {
        set_auto_answer(app, true);
    }

    if !app.screenshot_mode() {
        // The splash (frmAbout) while loading; the modem check runs behind it.
        app.show("frmAbout", false);
        app.s.main.splash_until = Some(Instant::now() + Duration::from_millis(2500));
        let line = LineChoice::from_profile(&app.prof);
        if line == LineChoice::Com(0) {
            start_modem_search(app);
        } else {
            app.modem.open(line);
            app.s.main.modem_check = Some(Instant::now());
            app.after(300, "Telelink", "ate1");
        }
    }
    tick_clock(app);
}

/// Module8 sub072C: the init string and the D3/D1 addons ("AT" taken off them).
fn init_strings(app: &mut App) {
    let p = app.prof.clone();
    app.g.init_cmd = "AT".into();
    app.g.init_addons = p.get("Modem", "ModemInitAddons");
    let mut d3 = p.get("Modem", "ModemD3InitAddons");
    if let Some(r) = d3.strip_prefix("AT") {
        d3 = r.to_string();
        p.set("Modem", "ModemD3InitAddons", &d3);
    }
    let mut d1 = p.get("Modem", "ModemD1InitAddons");
    if let Some(r) = d1.strip_prefix("AT") {
        d1 = r.to_string();
        p.set("Modem", "ModemD1InitAddons", &d1);
    }
    app.g.d3_addons = d3;
    app.g.d1_addons = d1;
}

/// Module8 sub03BF: (re)open Comm1 and initialise the modem.
pub fn init_modem(app: &mut App) {
    app.modem.open(match LineChoice::from_profile(&app.prof) {
        LineChoice::Com(n) => LineChoice::Com(n.max(1)),
        sip => sip,
    });
    app.modem.baud(1200);
    app.modem.send("ATZ\r");
    let s = format!("{}{}\r", app.g.init_cmd, app.g.init_addons);
    app.modem.send(&s);
    accept_incoming_modem(app);
    let d3 = app.form("Telelink").text("cmdPhoneType") == app.g.d3;
    phone_type_modem(app, d3);
}

/// sub0707: ATS0=1 answers incoming calls, ATS0=0 doesn't.
fn accept_incoming_modem(app: &mut App) {
    let yes = app.form("Telelink").text("cmdAcceptIncoming") == app.g.yes;
    app.modem.send(if yes { "ATS0=1\r" } else { "ATS0=0\r" });
}

/// sub0746: D3 phones talk at 1200 baud with the D3 addons, D1 phones at 300 with theirs.
fn phone_type_modem(app: &mut App, d3: bool) {
    app.modem.baud(if d3 { 1200 } else { 300 });
    let s = if d3 { format!("AT{}\r", app.g.d3_addons) } else { format!("AT{}\r", app.g.d1_addons) };
    app.modem.send(s.trim_start_matches("AT\r"));
    let t = if d3 { app.g.d3.clone() } else { app.g.d1.clone() };
    app.form("Telelink").c("cmdPhoneType").text = t;
}

fn start_modem_search(app: &mut App) {
    app.show("frmModemSearch", false);
    app.s.main.search_port = 0;
    app.s.main.search_at = Some(Instant::now());
    next_search_port(app);
}

/// fn06FA: try COM1..COM8, sending ATE1/ATQ0 and waiting 5 seconds for OK on each.
fn next_search_port(app: &mut App) {
    app.s.main.search_port += 1;
    let n = app.s.main.search_port;
    if n > 8 {
        app.unload("frmModemSearch");
        app.show("frmNoModems", false);
        app.s.main.search_at = None;
        return;
    }
    let cap = format!("{} {n}", app.tor(SEC, "msPort", "Trying Port"));
    app.form("frmModemSearch").c("lblPort").caption = cap;
    app.modem.open(LineChoice::Com(n as u8));
    app.modem.take_rx();
    app.after(300, "Telelink", "search_at");
    app.s.main.search_at = Some(Instant::now());
}

pub fn later(app: &mut App, tag: &str) {
    match tag {
        "ate1" | "search_at" => {
            app.modem.take_rx();
            app.modem.send("ATE1\r");
            app.modem.send("ATQ0\r");
        }
        _ => {}
    }
}

pub fn tick(app: &mut App) {
    // Splash.
    if let Some(t) = app.s.main.splash_until {
        if Instant::now() >= t {
            app.s.main.splash_until = None;
            app.unload("frmAbout");
        }
    }
    // Start-up modem check: OK within 2 s of ATE1/ATQ0.
    if let Some(t) = app.s.main.modem_check {
        if t.elapsed() > Duration::from_millis(2300) {
            app.s.main.modem_check = None;
            let rx = app.modem.take_rx();
            if rx.to_ascii_uppercase().contains("OK") {
                app.g.modem_ok = true;
                init_modem(app);
            } else {
                start_modem_search(app);
            }
        }
    }
    if let Some(t) = app.s.main.search_at {
        let open_err = app.modem.sh.lock().unwrap().open_error.clone();
        if open_err.is_some() || t.elapsed() > Duration::from_millis(5300) {
            let rx = app.modem.take_rx();
            if open_err.is_none() && rx.to_ascii_uppercase().contains("OK") {
                app.s.main.search_at = None;
                let n = app.s.main.search_port;
                app.g.serial_port = n;
                app.g.modem_ok = true;
                app.prof.set("Modem", "SerialPort", &format!(" {n}"));
                app.unload("frmModemSearch");
                init_modem(app);
            } else {
                next_search_port(app);
            }
        }
    }
    // tmrAutoCall: once a second.
    let due = app.s.main.last_tick.is_none_or(|t| t.elapsed() >= Duration::from_millis(1000));
    if due {
        app.s.main.last_tick = Some(Instant::now());
        tick_clock(app);
    }
}

/// tmrAutoCall_Timer.
fn tick_clock(app: &mut App) {
    let now = Local::now().naive_local();
    let days = super::day_names(app);
    let wd = now.weekday().num_days_from_sunday() as usize;
    let today = days[wd].clone();
    // Connection label.
    let conn = app.modem.connected();
    let connected_to = app.tor(SEC, "gvConnectedTo", "Connected to ");
    let rid = app.tor("VIEW INCOMING", "vRID", "Receiving Incoming Data");
    let cd = app.modem.sh.lock().unwrap().cd;
    app.g.connected = conn.is_some() || cd;
    let not_conn = app.g.not_connected.clone();
    let flags_file = app.dir("PLW").join("_BITFLGS.FLX").exists();
    app.s.main.blink = !app.s.main.blink;
    let blink = app.s.main.blink;
    {
        let f = app.form("Telelink");
        f.ci("lblToday", 0).caption = now.format("%-m/%d/%Y").to_string();
        f.ci("lblToday", 1).caption = today.clone();
        f.ci("lblToday", 2).caption = now.format("%H:%M:%S").to_string();
        match &conn {
            Some((t, _)) => {
                f.c("lblConnection").caption = format!("{connected_to}{}", t.shown);
                f.c("cmdHangUp").visible = true;
            }
            None if cd => {
                f.c("lblConnection").caption = rid;
                f.c("cmdHangUp").visible = true;
            }
            None => {
                f.c("lblConnection").caption = not_conn;
                f.c("cmdHangUp").visible = false;
            }
        }
        // New flags: the alert frame blinks.
        f.ci("Frame3D1", 6).visible = flags_file;
        f.ci("lblAlert", 0).visible = flags_file;
        f.ci("lblAlert", 1).visible = flags_file;
        let red = eframe::egui::Color32::from_rgb(255, 0, 0);
        let gray = eframe::egui::Color32::from_rgb(192, 192, 192);
        f.ci("lblAlert", 0).fore = Some(if blink { red } else { gray });
        f.ci("lblAlert", 1).fore = Some(if blink { gray } else { red });
        f.menu_enable("control64", flags_file);
        f.menu_enable("control30", conn.is_some() || cd);
    }
    let stamp = now.format("%Y%m%d%H%M%S").to_string();
    let on = app.g.on.clone();
    // AutoAnswer schedule.
    if app.form("Telelink").text("cmdAutoAnswer") != on {
        let f = app.form("Telelink");
        f.ci("lblAnswerSched", 0).caption = ".".into();
        f.ci("lblAnswerSched", 1).caption = ".".into();
        app.s.main.aa_day.clear();
    } else {
        if app.s.main.aa_day != stamp[..8] {
            app.s.main.aa_day = stamp[..8].to_string();
            let read = |app: &App, key: &str| -> Option<(String, String)> {
                let v = app.prof.get("AutoAnswer", key);
                if v.is_empty() || v == "," {
                    return None;
                }
                let (a, b) = v.split_once(',').unwrap_or((&v, ""));
                let six = |x: &str| format!("{:0>4}00", x.trim()).chars().rev().take(6).collect::<Vec<_>>().into_iter().rev().collect::<String>();
                Some((format!("{}{}", &stamp[..8], six(a)), format!("{}{}", &stamp[..8], six(b))))
            };
            let none = (("9".repeat(14)), ("0".repeat(14)));
            let d3 = read(app, &app.g.d3.clone());
            let d1 = read(app, &app.g.d1.clone());
            let nothing = app.tor(SEC, "vNothingScheduled", "Nothing Scheduled");
            let fmt = |l: &str, w: &Option<(String, String)>| match w {
                Some((a, b)) => format!("({l})  {}:{} to {}:{}", &a[8..10], &a[10..12], &b[8..10], &b[10..12]),
                None => format!("({l})  {nothing}"),
            };
            let (c0, c1) = (fmt("D3", &d3), fmt("D1", &d1));
            app.s.main.aa_d3 = d3.unwrap_or(none.clone());
            app.s.main.aa_d1 = d1.unwrap_or(none);
            let f = app.form("Telelink");
            f.ci("lblAnswerSched", 0).caption = c0;
            f.ci("lblAnswerSched", 1).caption = c1;
        }
        if !app.g.batch_running && !app.g.connected {
            let (a3, b3) = app.s.main.aa_d3.clone();
            let (a1, b1) = app.s.main.aa_d1.clone();
            let yes = app.g.yes.clone();
            let d3 = app.g.d3.clone();
            if stamp >= a3 && stamp < b3 {
                if app.form("Telelink").text("cmdAcceptIncoming") != yes {
                    app.form("Telelink").c("cmdAcceptIncoming").text = yes.clone();
                    accept_incoming_modem(app);
                }
                if app.form("Telelink").text("cmdPhoneType") != d3 {
                    phone_type_modem(app, true);
                }
            } else if stamp >= a1 && stamp < b1 {
                if app.form("Telelink").text("cmdAcceptIncoming") != yes {
                    app.form("Telelink").c("cmdAcceptIncoming").text = yes.clone();
                    accept_incoming_modem(app);
                }
                if app.form("Telelink").text("cmdPhoneType") == d3 {
                    phone_type_modem(app, false);
                }
            } else if app.form("Telelink").text("cmdAcceptIncoming") == yes {
                let no = app.g.no.clone();
                app.form("Telelink").c("cmdAcceptIncoming").text = no;
                accept_incoming_modem(app);
            }
        }
    }
    // Incoming call: carrier while nobody dialed (Comm1_OnComm, CD change).
    if cd && conn.is_none() && !app.s.main.was_connected {
        app.g.income_mode = "Receive".into();
        let yes = app.g.yes.clone();
        if !app.g.batch_running && app.form("Telelink").text("cmdAcceptIncoming") == yes {
            app.show("frmIncome", false);
        }
    }
    app.s.main.was_connected = cd || conn.is_some();

    // AutoCall.
    if app.form("Telelink").text("cmdAutoCall") != on {
        app.s.main.ac_day.clear();
        return;
    }
    if app.s.main.ac_day != today {
        app.s.main.ac_day = today.clone();
        app.s.main.ac_at = "9".repeat(14);
        app.s.main.ac_loaded_at = stamp.clone();
        let r = app.prof.get("AutoCall", "RetryCount");
        app.s.main.ac_retry = if r.is_empty() { "1".into() } else { r };
        let entry = app.prof.get("AutoCall", &today);
        let today_at = app.tor(SEC, "xTodayAt", "Today at");
        let (mut l0, mut l1) = (String::new(), String::new());
        let mut list = String::new();
        if !entry.is_empty() {
            let (time, files) = entry.split_once(',').unwrap_or((&entry, ""));
            let six = format!("{:0>4}00", time.trim());
            app.s.main.ac_at = format!("{}{}", &stamp[..8], &six[six.len() - 6..]);
            l0 = format!("{today_at} {}:{}", &app.s.main.ac_at[8..10], &app.s.main.ac_at[10..12]);
            l1 = paren(files);
            list = files.to_string();
        }
        if app.s.main.ac_at == "9".repeat(14) || app.s.main.ac_at < app.s.main.ac_loaded_at {
            // fn0960: the next day with a schedule.
            match next_scheduled(app, wd) {
                None => {
                    l0 = app.tor(SEC, "xNothing", "Nothing");
                    l1 = app.tor(SEC, "xScheduled", "Scheduled");
                }
                Some((day, time, files)) => {
                    let d = if day == today { format!("{} {day}", app.tor(SEC, "xNext", "Next")) } else { day };
                    let six = format!("{:0>4}00", time.trim());
                    let six = &six[six.len() - 6..];
                    l0 = format!("{d} at {}:{}", &six[..2], &six[2..4]);
                    l1 = paren(&files);
                }
            }
            app.s.main.ac_at = "9".repeat(14);
            list.clear();
        }
        app.s.main.ac_files = list.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| format!("{}", app.dir("BXW").join(s).display())).collect();
        let f = app.form("Telelink");
        f.ci("lblNextRun", 0).caption = l0;
        f.ci("lblNextRun", 1).caption = l1;
    }
    if app.s.main.ac_at > app.s.main.ac_loaded_at && stamp > app.s.main.ac_at && !app.g.autocall_running {
        // Time to run today's batch files (sub021A for each).
        app.s.main.ac_day.clear();
        app.g.autocall_running = true;
        let files = std::mem::take(&mut app.s.main.ac_files);
        let retry: i32 = app.s.main.ac_retry.trim().parse().unwrap_or(1);
        super::batch::auto_run(app, files, retry);
    }
}

fn paren(files: &str) -> String {
    let s: String = files.chars().take(20).collect();
    if files.chars().count() > 20 { format!("({s}...)") } else { format!("({s})") }
}

fn next_scheduled(app: &App, from: usize) -> Option<(String, String, String)> {
    let days = super::day_names(app);
    for k in 1..=7 {
        let d = &days[(from + k) % 7];
        let v = app.prof.get("AutoCall", d);
        if !v.is_empty() {
            let (t, f) = v.split_once(',').unwrap_or((&v, ""));
            return Some((d.clone(), t.to_string(), f.to_string()));
        }
    }
    None
}

/// sub0896: AutoCall ON shows the Next AutoCall Run frame.
pub fn set_auto_call(app: &mut App, on: bool) {
    let (t, f) = (app.g.on.clone(), app.g.off.clone());
    let fm = app.form("Telelink");
    fm.c("cmdAutoCall").text = if on { t } else { f };
    fm.ci("Frame3D1", 3).visible = on;
    app.s.main.ac_day.clear();
    if on && app.loaded("frmBatchExec") {
        let m = app.t(SEC, "xBELoaded", "The Batch Execute form is loaded.");
        let title = app.g.title.clone();
        app.msg("", "", &m, 48, &title);
    }
}

/// sub087F: AutoAnswer ON shows its schedule and takes over Accept Incoming and Phone Type.
pub fn set_auto_answer(app: &mut App, on: bool) {
    let (t, f) = (app.g.on.clone(), app.g.off.clone());
    let yes = app.g.yes.clone();
    let accepting = app.form("Telelink").text("cmdAcceptIncoming") == yes;
    let fm = app.form("Telelink");
    fm.c("cmdAutoAnswer").text = if on { t } else { f };
    fm.ci("Frame3D1", 8).visible = on;
    fm.c("cmdAcceptIncoming").enabled = !on;
    fm.c("cmdPhoneType").enabled = !on;
    app.s.main.aa_day.clear();
    if on == accepting {
        toggle_accept(app);
    }
}

/// sub0864: Accept Incoming YES/NO.
fn toggle_accept(app: &mut App) {
    if app.g.connected {
        let m = app.t(SEC, "vThereIsAPhoneConnected", "There is a phone connected.");
        let t = app.tor(SEC, "vAcceptIncoming", "Accept Incoming");
        app.msg("", "", &m, 48, &t);
        return;
    }
    let (yes, no) = (app.g.yes.clone(), app.g.no.clone());
    let f = app.form("Telelink");
    let now_yes = f.text("cmdAcceptIncoming") == no;
    f.c("cmdAcceptIncoming").text = if now_yes { yes } else { no };
    accept_incoming_modem(app);
}

/// sub08BC: Exit / Quick Exit.
pub fn quick_exit(app: &mut App) {
    let on = app.g.on.clone();
    let f = app.form("Telelink");
    if f.text("cmdAutoCall") == on || f.text("cmdAutoAnswer") == on {
        let m = app.t(SEC, "AutoExit", "AutoCall and/or AutoAnswer is ON, do you still wish to exit?");
        let title = app.g.title.clone();
        app.msg("Telelink", "exit", &m, 36, &title);
    } else {
        exit_now(app);
    }
}

fn exit_now(app: &mut App) {
    app.modem.send("AT\r");
    if app.modem.connected().is_some() {
        app.modem.hang_up();
    }
    app.modem.close();
    app.quit = true;
}

pub fn answer(app: &mut App, tag: &str, a: Answer) {
    match tag {
        "exit" if a == Answer::Yes => exit_now(app),
        _ => {}
    }
}

pub fn event(app: &mut App, name: &str, ev: Ev) {
    if name == "frmVersionInfo" {
        if matches!(ev, Ev::Click(ref c, _) if c == "cmdOK") || ev == Ev::Close {
            app.unload("frmVersionInfo");
        }
        return;
    }
    if name != "Telelink" {
        if ev == Ev::Close {
            app.unload(name);
        }
        return;
    }
    let on = app.g.on.clone();
    match ev {
        Ev::Close => quick_exit(app),
        Ev::Click(c, _) => match c.as_str() {
            "cmdAutoCall" => {
                let now_on = app.form("Telelink").text("cmdAutoCall") == on;
                if !now_on && next_scheduled(app, 6).is_none() && app.prof.get("AutoCall", &super::day_names(app)[0]).is_empty() {
                    let m = app.t(SEC, "xScheduleProblem", "There is a problem with the schedule, please re-configure.");
                    let t = app.g.title.clone();
                    app.msg("", "", &m, 48, &t);
                }
                set_auto_call(app, !now_on);
            }
            "cmdAutoAnswer" => {
                let now_on = app.form("Telelink").text("cmdAutoAnswer") == on;
                set_auto_answer(app, !now_on);
            }
            "cmdAcceptIncoming" => toggle_accept(app),
            "cmdPhoneType" => {
                let d3 = app.form("Telelink").text("cmdPhoneType") == app.g.d3;
                phone_type_modem(app, !d3);
            }
            "cmdHangUp" => hang_up(app),
            "Image1" => app.show("frmBatchExec", false),
            "lblAlert" => {
                if !app.g.connected {
                    app.g.flags_mode = "New".into();
                    app.show("frmCallFlags", false);
                }
            }
            _ => {}
        },
        Ev::DblClick(c, _) => {
            if c == "lblToday" {
                app.g.debug = !app.g.debug;
                crate::app::beep();
            } else {
                app.show("frmVersionInfo", true);
            }
        }
        Ev::Menu(m) => menu(app, &m),
        _ => {}
    }
}

pub fn hang_up(app: &mut App) {
    app.modem.hang_up();
    app.g.connected = false;
    app.g.outgoing = true;
}

fn menu(app: &mut App, m: &str) {
    let on = app.g.on.clone();
    match m {
        "control26" => {
            app.show("frmAbout", true);
            app.s.main.splash_until = Some(Instant::now() + Duration::from_millis(3000));
        }
        "control28" | "control69" => quick_exit(app),
        "control30" => hang_up(app),
        "control32" => app.show("frmAOSScript", true),
        "control33" => {
            app.show("frmCollectionStatus", true);
        }
        "control34" => {
            if app.g.country == "ISRAEL" {
                app.show("frmCSNiTable", true)
            } else {
                app.show("frmCSNTable", true)
            }
        }
        "control35" => app.show("frmProgrammableFeatures", true),
        "control36" => app.show("frmRateTable", true),
        "control37" => app.show("frmSMDR", true),
        "control38" => app.show("frmSpeedDial", true),
        "control39" => app.show("frmTimeDate", true),
        "control40" => app.show("frmUPTTable", false),
        "control41" => app.show("ScrollIt", false),
        "control42" => app.show("firmware", true),
        "control44" => {
            // sub09D0: the phone list editor.
            if app.g.phone_list_as_log {
                app.unload("frmPhoneList");
            }
            app.g.phone_list_as_log = false;
            app.g.lookup = false;
            app.show("frmPhoneList", true);
        }
        "control46" => {
            if app.g.usa || app.g.country == "MEXICO" {
                app.show("frmBatchSetup", true)
            } else {
                app.show("frmBatchSetupChina", true)
            }
        }
        "control47" => {
            if app.form("Telelink").text("cmdAutoCall") == on {
                let m = app.t("BATCH EXECUTE", "beAutoCallISOn", "AutoCall is ON.  Please note that AutoCall will be temporarily turned OFF while this function is being used.");
                let t = app.t("BATCH EXECUTE", "beTitle", "Batch Exec");
                app.msg("", "", &m, 48, &t);
            }
            app.show("frmBatchExec", false);
        }
        "control48" => {
            // sub09BB: the phone list as the batch log.
            if !app.g.phone_list_as_log {
                app.unload("frmPhoneList");
            }
            app.g.phone_list_as_log = true;
            app.g.lookup = false;
            app.show("frmPhoneList", true);
        }
        "control50" => {
            app.g.income_mode = "Review".into();
            if !app.g.batch_running {
                app.show("frmIncome", true);
            }
        }
        "control52" => {
            if app.form("Telelink").text("cmdAutoCall") == on {
                let m = app.t(SEC, "TurnOffAC", "You must turn off AutoCall to configure it.");
                let t = app.g.title.clone();
                app.msg("", "", &m, 48, &t);
            } else {
                app.show("frmBatchSchedule", true);
            }
        }
        "control54" => {
            if app.form("Telelink").text("cmdAutoAnswer") == on {
                let m = app.t(SEC, "TurnOffAA", "You must turn off AutoAnswer to configure it.");
                let t = app.g.title.clone();
                app.msg("", "", &m, 48, &t);
            } else {
                app.show("frmIncomingSchedule", true);
            }
        }
        "control56" => {
            if app.g.connected {
                app.msg("", "", "You must hang up before changing system settings.", 48, "System Config");
            } else {
                app.show("frmSysConfig", true);
            }
        }
        "control61" => app.show("frmCDRGenerator", false),
        "control62" => {
            app.g.lookup = false;
            app.show("frmReport", true);
        }
        "control64" => {
            app.g.flags_mode = "New".into();
            app.show("frmCallFlags", false);
        }
        "control65" => {
            app.g.flags_mode = "Review".into();
            app.show("frmCallFlags", false);
        }
        "control66" => {
            let _ = std::process::Command::new("C:\\TFW\\RTE2.EXE").spawn();
        }
        m if m.starts_with("control68:") => {
            let cmd = &m["control68:".len()..];
            if std::process::Command::new("cmd").args(["/C", "start", "", cmd]).spawn().is_err() {
                app.msg("", "", "Could not execute the specified utility.", 0, "");
            }
        }
        _ => {}
    }
}

#[allow(dead_code)]
fn hhmm(t: chrono::NaiveDateTime) -> String {
    format!("{:02}:{:02}", t.hour(), t.minute())
}

//! SYSCONF.FRM: System Configuration Settings, saved to TFW.INI.

use crate::app::App;
use crate::vb::{Answer, Ev, MenuItem};

const S: &str = "SYSTEM CONFIGURATION SETTINGS";
const COUNTRIES: [&str; 8] = ["USA", "CHINA", "ENGLAND", "ISRAEL", "MEXICO", "SPAIN", "TAIWAN", "OTHER"];

pub fn load(app: &mut App) {
    let t = |k: &str, d: &str| app.tor(S, k, d);
    let mut caps: Vec<(&str, i32, String)> = vec![
        ("fraModem", 0, format!(" {}", t("framodem(0)", "Modem Port"))),
        ("fraModem", 1, format!(" {}", t("framodem(1)", "Modem Initialization"))),
        ("fraModem", 2, format!(" {}", t("framodem(2)", "D3 Setup"))),
        ("fraModem", 3, format!(" {}", t("framodem(3)", "D1 Setup"))),
        ("fraDialing", 0, format!(" {}", t("fraDialing(0)", "Dialing Timeouts (seconds)"))),
        ("fraDialing", 1, format!(" {}", t("fradialing(1)", "Behind a PBX ?"))),
        ("Frame3", -1, t("frame3", "Modem Volume")),
        ("fraSystem", 0, format!(" {} ", t("frasystem(0)", "Enable Sounds"))),
        ("fraSystem", 1, format!(" {} ", t("frasystem(1)", "Country"))),
        ("DCW", -1, t("dcw", "Disable Call Waiting")),
        ("Frame1", -1, t("frame1", "Country Font")),
        ("Frame2", -1, t("frame2", "Printer Font Size")),
        ("cmdOk", -1, t("cmdok", "&Ok")),
        ("cmdCancel", -1, t("cmdcancel", "&Cancel")),
        ("lblDelay", -1, t("lbldelay", "Inter-command delay (ms)")),
        ("lblDialTime", 0, t("lbldialtime(0)", "Local:")),
        ("lblDialTime", 2, t("lbldialtime(2)", "Long Distance:")),
        ("optPBX", 0, t("optpbx(0)", "NO")),
        ("optPBX", 1, t("optpbx(1)", "YES")),
        ("lblPBX", -1, t("lblpbx", "PBX Prefix:")),
        ("chkFlags", -1, t("chkflags", "Flags")),
        ("chkMinimum", -1, t("chkminimum", "Daily Min.")),
        ("chkUnpolled", -1, t("chkunpolled", "Not Polled")),
        ("chkMonthlyTotals", -1, t("chkmonthlytotals", "Monthly Totals")),
        ("optSound", 0, t("optsound(0)", "NO")),
        ("optSound", 1, t("optsound(1)", "YES")),
        ("DCWYes", -1, t("dcwyes", "YES")),
        ("DCWNo", -1, t("dcwno", "NO")),
        ("DCWInfo", -1, t("dcwinfo", "Enter your 2 digit code.")),
        ("IntOFF", -1, t("intoff", "OFF")),
        ("IntON", -1, t("inton", "ON")),
    ];
    for i in 0..4 {
        caps.push(("optModemVolume", i, t(&format!("optmodemvolume({i})"), "")));
    }
    for i in 0..6 {
        caps.push(("lblModemInit", i, t(&format!("lblmodeminit({i})"), "")));
    }
    let title = t("frmsysconfig", "System Configuration Settings");
    let p = app.prof.clone();
    let g = (app.g.serial_port, app.g.delay, app.g.dial_local, app.g.dial_ld, app.g.sound);
    let f = app.form("frmSysConfig");
    f.caption = title;
    f.menus = vec![MenuItem::new("mnuqe", "&Quick Exit")];
    for (n, i, c) in caps {
        if c.is_empty() || c == "NO STRING!" {
            continue;
        }
        if let Some(k) = f.find(n, i) {
            if f.ctls[k].kind == crate::vb::Kind::SSFrame {
                f.ctls[k].text = c;
            } else {
                f.ctls[k].caption = c;
            }
        }
    }
    for i in 1..=8 {
        f.ci("optSerialPort", i).value = (i == g.0) as i32;
    }
    f.c("lblDelayVal").caption = g.1.to_string();
    f.ci("lblDialTime", 1).caption = g.2.to_string();
    f.ci("lblDialTime", 3).caption = g.3.to_string();
    f.ci("optSound", 1).value = g.4 as i32;
    f.ci("optSound", 0).value = (!g.4) as i32;
    let pbx = p.get("DialingDefaults", "PBX");
    f.ci("optPBX", 1).value = (!pbx.is_empty()) as i32;
    f.ci("optPBX", 0).value = pbx.is_empty() as i32;
    f.c("edtPBX").set_masked(&pbx);
    let dcw = p.get("DialingDefaults", "DCW");
    f.c("DCWYes").value = (!dcw.is_empty()) as i32;
    f.c("DCWNo").value = dcw.is_empty() as i32;
    f.c("DCWME").set_masked(&dcw);
    let combo = f.c("cboCountry");
    combo.list = COUNTRIES.iter().map(|s| s.to_string()).collect();
    let c = p.get("System", "Country").to_ascii_uppercase();
    combo.list_index = COUNTRIES.iter().position(|x| *x == c).map(|x| x as i32).unwrap_or(0);
    combo.text = combo.list[combo.list_index as usize].clone();
    let intl = p.get("Language", "International").eq_ignore_ascii_case("ON");
    f.c("IntON").value = intl as i32;
    f.c("IntOFF").value = (!intl) as i32;
    f.c("MaskedEdit1").set_masked(p.get("System", "UserSelectedFontSize").trim());
    for (k, key) in ["ModemInitAddons", "ModemD3InitAddons", "ModemD1InitAddons"].iter().enumerate() {
        f.ci("txtInitString", k as i32).text = "AT".into();
        f.ci("txtInitString", k as i32).enabled = false;
        f.ci("txtInitAddon", k as i32).text = p.get("Modem", key);
    }
    // sub0770: the volume option from the addons.
    for i in 0..4 {
        f.ci("optModemVolume", i).value = 0;
    }
    for k in 0..3 {
        let a = f.texti("txtInitAddon", k).to_ascii_uppercase();
        for (i, code) in ["M0", "L1", "L2", "L3"].iter().enumerate() {
            if a.contains(code) {
                f.ci("optModemVolume", i as i32).value = 1;
            }
        }
    }
    for (n, key) in [("chkUnpolled", "Unpolled"), ("chkMinimum", "Minimum"), ("chkMonthlyTotals", "MonthlyTotals"), ("chkFlags", "Flags")] {
        f.c(n).value = (p.get("BatchAutoPrint", key).trim() == "1") as i32;
    }
    pbx_visibility(app);
    add_phone_line(app);
}

/// Added to the original form: which line the program uses. "Modem on" any COM port (the
/// original only offers COM1-8 above), or the built-in SIP modem with its account.
fn add_phone_line(app: &mut App) {
    let line = crate::modem::LineChoice::from_profile(&app.prof);
    let sip = crate::sip::SipConfig::load(&app.prof);
    let current = app.g.serial_port;
    let mut ports: Vec<String> = serialport::available_ports().map(|v| v.into_iter().map(|p| p.port_name).collect()).unwrap_or_default();
    if current > 0 && !ports.iter().any(|p| *p == format!("COM{current}")) {
        ports.push(format!("COM{current}"));
    }
    ports.sort_by_key(|p| p.trim_start_matches("COM").parse::<u32>().unwrap_or(999));
    let f = app.form("frmSysConfig");
    f.client_h += 1560.0;
    f.c("cmdOk").top += 1560.0;
    f.c("cmdCancel").top += 1560.0;
    f.add("Frame", "fraLine", -1, None, [120.0, 5280.0, 8655.0, 1440.0], &[("Caption", " Phone Line "), ("ForeColor", "&H00FF0000&")]);
    let fr = Some("fraLine");
    let opt = |f: &mut crate::vb::Form, i: i32, top: f32, cap: &str, on: bool| {
        f.add("OptionButton", "optLine", i, fr, [240.0, top, 2055.0, 255.0], &[("Caption", cap), ("Value", if on { "-1" } else { "0" })]);
    };
    let is_sip = matches!(line, crate::modem::LineChoice::Sip(_));
    opt(f, 0, 330.0, "Modem on port", !is_sip);
    opt(f, 1, 720.0, "SIP modem (built in)", is_sip);
    f.add("ComboBox", "cboComPort", -1, fr, [2280.0, 285.0, 1215.0, 315.0], &[("Style", "2")]);
    let combo = f.c("cboComPort");
    combo.list = ports;
    combo.list_index = combo.list.iter().position(|p| *p == format!("COM{current}")).map(|i| i as i32).unwrap_or(-1);
    combo.text = combo.selected().cloned().unwrap_or_default();
    let field = |f: &mut crate::vb::Form, name: &str, label: &str, rect: [f32; 4], value: &str, secret: bool| {
        f.add("Label", &format!("lbl{name}"), -1, fr, [rect[0] - 1095.0, rect[1] + 30.0, 1020.0, 255.0], &[("Caption", label), ("Alignment", "1")]);
        let props: &[(&str, &str)] = if secret { &[("PasswordChar", "*"), ("FontBold", "0")] } else { &[("FontBold", "0")] };
        f.add("TextBox", name, -1, fr, rect, props);
        f.c(name).text = value.to_string();
    };
    field(f, "txtSipServer", "Server:", [4800.0, 285.0, 2295.0, 285.0], &sip.server, false);
    field(f, "txtSipPort", "Port:", [7680.0, 285.0, 735.0, 285.0], &sip.port.to_string(), false);
    field(f, "txtSipUser", "User:", [4800.0, 675.0, 2295.0, 285.0], &sip.user, false);
    field(f, "txtSipPassword", "Password:", [4800.0, 1065.0, 2295.0, 285.0], &sip.password, true);
    f.add("Label", "lblSipNote", -1, fr, [240.0, 1080.0, 3255.0, 255.0], &[("Caption", "Outgoing calls with modem tones"), ("FontBold", "0")]);
    line_visibility(app);
}

fn line_visibility(app: &mut App) {
    let f = app.form("frmSysConfig");
    let sip = f.valuei("optLine", 1) == 1;
    f.c("cboComPort").enabled = !sip;
    for n in ["txtSipServer", "txtSipPort", "txtSipUser", "txtSipPassword"] {
        f.c(n).enabled = sip;
    }
}

/// The COM port list and the original's COM1-8 buttons show the same port.
fn sync_ports(app: &mut App, from_buttons: bool) {
    let f = app.form("frmSysConfig");
    if from_buttons {
        if let Some(n) = (1..=8).find(|&i| f.valuei("optSerialPort", i) == 1) {
            let name = format!("COM{n}");
            let c = f.c("cboComPort");
            if !c.list.contains(&name) {
                c.list.push(name.clone());
            }
            c.list_index = c.list.iter().position(|p| *p == name).unwrap_or(0) as i32;
            c.text = name;
        }
    } else {
        let n: i32 = f.text("cboComPort").trim_start_matches("COM").parse().unwrap_or(0);
        for i in 1..=8 {
            f.ci("optSerialPort", i).value = (i == n) as i32;
        }
    }
}

fn pbx_visibility(app: &mut App) {
    let f = app.form("frmSysConfig");
    let pbx = f.valuei("optPBX", 1) == 1;
    f.c("edtPBX").visible = pbx;
    f.c("lblPBX").visible = pbx;
    let dcw = f.value("DCWYes") == 1;
    f.c("DCWME").visible = dcw;
    f.c("DCWInfo").visible = dcw;
}

fn spin(app: &mut App, ctl: &str, idx: i32, delta: i32, lo: i32, hi: i32) {
    let f = app.form("frmSysConfig");
    let c = f.ci(ctl, idx);
    let v: i32 = c.caption.trim().parse().unwrap_or(0);
    c.caption = (v + delta).clamp(lo, hi).to_string();
}

fn ok(app: &mut App) {
    let f = app.form("frmSysConfig");
    let port: i32 = f.text("cboComPort").trim_start_matches("COM").parse().ok().or((1..=8).find(|&i| f.valuei("optSerialPort", i) == 1)).unwrap_or(1);
    let use_sip = f.valuei("optLine", 1) == 1;
    let sip = crate::sip::SipConfig {
        server: f.text("txtSipServer").trim().to_string(),
        port: f.text("txtSipPort").trim().parse().unwrap_or(5060),
        user: f.text("txtSipUser").trim().to_string(),
        password: f.text("txtSipPassword"),
    };
    if use_sip && (sip.server.is_empty() || sip.user.is_empty()) {
        return app.msg("", "", "Enter the SIP server and user name for the built-in SIP modem.", 48, "Phone Line");
    }
    let delay = f.get("lblDelayVal").caption.clone();
    let (local, ld) = (f.geti("lblDialTime", 1).caption.clone(), f.geti("lblDialTime", 3).caption.clone());
    let pbx = if f.valuei("optPBX", 1) == 1 { Some(f.get("edtPBX").clip_text()) } else { None };
    let dcw = if f.value("DCWYes") == 1 { Some(f.get("DCWME").clip_text()) } else { None };
    let country = f.get("cboCountry").selected().cloned();
    let sound = f.valuei("optSound", 1) == 1;
    let intl = f.value("IntON") == 1;
    let fsize = f.get("MaskedEdit1").clip_text();
    let addons: Vec<String> = (0..3).map(|k| f.texti("txtInitAddon", k)).collect();
    let checks: Vec<(String, i32)> = [("Unpolled", "chkUnpolled"), ("Minimum", "chkMinimum"), ("MonthlyTotals", "chkMonthlyTotals"), ("Flags", "chkFlags")].iter().map(|(k, c)| (k.to_string(), f.value(c))).collect();
    let t = |app: &App, k: &str, d: &str| app.tor(S, k, d);
    if pbx.as_ref().is_some_and(|p| p.is_empty()) {
        let (m, ti) = (t(app, "sysValidPrefix", "You must enter a valid PBX prefix."), t(app, "sysPBX", "PBX"));
        return app.msg("", "", &m, 48, &ti);
    }
    if dcw.as_ref().is_some_and(|d| d.len() != 2) {
        let (m, ti) = (t(app, "sysValidCode", "You must enter a valid code to disable call waiting."), t(app, "sysDCW", "DCW"));
        return app.msg("", "", &m, 48, &ti);
    }
    let Some(country) = country else {
        let (m, ti) = (t(app, "sysSelCountry", "You must select a country"), t(app, "sysCountry", "Country"));
        return app.msg("", "", &m, 48, &ti);
    };
    let p = app.prof.clone();
    let mut ok = true;
    ok &= p.set("Modem", "SerialPort", &format!(" {port}"));
    let old_line = crate::modem::LineChoice::from_profile(&p);
    ok &= p.set("Modem", "Line", if use_sip { "SIP" } else { "COM" });
    ok &= sip.save(&p);
    let line_changed = crate::modem::LineChoice::from_profile(&p) != old_line;
    ok &= p.set("Modem", "Delay", &delay);
    ok &= p.set("Modem", "ModemInitAddons", &addons[0]);
    ok &= p.set("Modem", "ModemD3InitAddons", &addons[1]);
    ok &= p.set("Modem", "ModemD1InitAddons", &addons[2]);
    ok &= p.set("Dialtime", "Local", &local);
    ok &= p.set("Dialtime", "OnePlus", &ld);
    ok &= p.set("DialingDefaults", "PBX", &pbx.clone().unwrap_or_default());
    ok &= p.set("DialingDefaults", "DCW", &dcw.clone().unwrap_or_default());
    ok &= p.set("System", "Country", &country);
    ok &= p.set("System", "Sound", if sound { "YES" } else { "NO" });
    ok &= p.set("System", "UserSelectedFontSize", &format!(" {fsize}"));
    ok &= p.set("Language", "International", if intl { "ON" } else { "OFF" });
    for (k, v) in checks {
        ok &= p.set("BatchAutoPrint", &k, &v.to_string());
    }
    if !ok {
        let m = t(app, "sysSaveFailed", "SaveFailed.  The value is good for this session only.");
        app.msg("", "", &m, 48, "System Configuration");
    }
    let port_changed = port != app.g.serial_port;
    app.g.serial_port = port;
    app.g.delay = delay.parse().unwrap_or(550);
    app.g.dial_local = local.parse().unwrap_or(70);
    app.g.dial_ld = ld.parse().unwrap_or(121);
    app.g.pbx = pbx.unwrap_or_default();
    app.g.dcw = dcw.unwrap_or_default();
    app.g.sound = sound;
    app.g.init_addons = addons[0].clone();
    app.g.d3_addons = addons[1].clone();
    app.g.d1_addons = addons[2].clone();
    app.unload("frmSysConfig");
    if !app.screenshot_mode() && (port_changed || line_changed || app.g.modem_ok) {
        super::main_form::init_modem(app);
    }
}

pub fn event(app: &mut App, ev: Ev) {
    if spin_event(app, &ev) {
        return;
    }
    match ev {
        Ev::Close => app.unload("frmSysConfig"),
        Ev::Click(c, i) => match c.as_str() {
            "cmdOk" => ok(app),
            "cmdCancel" => app.unload("frmSysConfig"),
            "optPBX" | "DCWYes" | "DCWNo" => pbx_visibility(app),
            "optLine" => line_visibility(app),
            "optSerialPort" => sync_ports(app, true),
            "cboComPort" => sync_ports(app, false),
            "optModemVolume" => {
                // sub06E6 removes M0/L1/L2/L3 from the addons, sub075E adds the chosen one.
                let code = ["M0", "L1", "L2", "L3"][i.clamp(0, 3) as usize];
                let f = app.form("frmSysConfig");
                for k in 0..3 {
                    let mut a = f.texti("txtInitAddon", k);
                    for c in ["M0", "L1", "L2", "L3"] {
                        a = a.replace(c, "");
                    }
                    a.push_str(code);
                    f.ci("txtInitAddon", k).text = a;
                }
            }
            _ => {}
        },
        _ => {}
    }
}

/// Spin buttons (handled here so the up/down direction is kept).
pub fn spin_event(app: &mut App, ev: &Ev) -> bool {
    let (c, i, d) = match ev {
        Ev::SpinUp(c, i) => (c.as_str(), *i, 1),
        Ev::SpinDown(c, i) => (c.as_str(), *i, -1),
        _ => return false,
    };
    match c {
        "spinDelay" => spin(app, "lblDelayVal", -1, 50 * d, 0, 5000),
        "spinDialTime" => spin(app, "lblDialTime", if i == 0 { 1 } else { 3 }, d, 10, 255),
        _ => return false,
    }
    true
}

pub fn answer(_app: &mut App, _tag: &str, _a: Answer) {}

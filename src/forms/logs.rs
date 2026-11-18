//! SMDR.FRM, COLLECT.FRM, CALLFLGS.FRM, INCOME.FRM and FRMLOADI.FRM: the logs the batch runs
//! and incoming calls leave in C:\TFW (SMD, CFW, PLW\_BITFLGS.FLX / .FLG, INC).

use crate::app::App;
use crate::files;
use crate::modem::{Op, Res};
use crate::proto;
use crate::vb::{Answer, Ev, MenuItem};
use chrono::{Datelike, Local};
use std::path::PathBuf;

#[derive(Default)]
pub struct LogState {
    /// ScrollIt's Test: the text, the position and when it last moved.
    pub scroll_test: Option<(String, usize, std::time::Instant)>,
    /// An AOS script ready to download: (script number, its 42 bytes).
    pub aos_bytes: Option<(usize, Vec<u8>)>,
    pub fea_upload_after_connect: bool,
    /// SMDR: the open file, its phones with their calls, which phone is shown.
    pub smd_file: Option<PathBuf>,
    pub smd: Vec<(String, Vec<proto::SmdrCall>)>,
    pub smd_at: usize,
    /// Collection Status: the open log and which record is shown.
    pub cfw_file: Option<PathBuf>,
    pub cfw: Vec<files::CollectionRec>,
    pub cfw_at: usize,
    /// Call flags shown.
    pub flags: Vec<Vec<String>>,
    pub flags_file: Option<PathBuf>,
}


pub fn load(app: &mut App, name: &str) {
    match name {
        "frmSMDR" => {
            let s = "SMDR";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let m = vec![
                MenuItem::sub(
                    "menfile",
                    t("menfile", "&File"),
                    vec![
                        MenuItem::new("menfileopen", t("menfileopen", "&Open")),
                        MenuItem::new("menfileprintcurr", t("menfileprintcurr", "&Print Current SMDR")),
                        MenuItem::new("menfileprintfile", t("menfileprintfile", "Print &SMDR File")),
                        MenuItem::new("menfileexportcurr", t("menfileexportcurr", "Export &current to a comma delimitted file.")),
                        MenuItem::new("menfileexportfile", t("menfileexportfile", "&Export SMDR file to a comma delimitted file.")),
                        MenuItem::sep(),
                        MenuItem::new("menfileexit", t("menfileexit", "E&xit")),
                    ],
                ),
                MenuItem::sub("mentransfer", t("mentransfer", "&Transfer"), vec![MenuItem::new("mentransclear", t("mentransclear", "&Clear SMDR")), MenuItem::new("mentransup", t("mentransup", "&Upload"))]),
                MenuItem::new("mnuqe", t("mnuqe", "&Quick Exit")),
            ];
            let caps = [
                ("Label1", format!("{}:", t("label1", "Phone Number"))),
                ("Command3", t("command3", "Search")),
                ("Label13", format!("{}:", t("label13", "ID Number"))),
                ("Command4", t("command4", "Search")),
                ("Label8", t("label8", "Working File:")),
                ("Label2", t("label2", "Date")),
                ("Label3", format!(" {}", t("label3", "Time"))),
                ("Label4", t("label4", "Phone #")),
                ("Label5", t("label5", "Len.")),
                ("Label6", t("label6", "Amt.")),
                ("Label7", t("label7", "Type of     Call")),
                ("Command1", t("command1", "Previous &Day")),
                ("Command2", t("command2", "Next D&ay")),
                ("cmdPreviousPhone", t("cmdpreviousphone", "&Previous Phone")),
                ("cmdNextPhone", t("cmdNextPhone", "&Next Phone")),
            ];
            let title = t("frmsmdr", "SMDR");
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            f.c("lblWorkingFile").caption = String::new();
            f.c("List1").font = "Courier New".into();
            // Today's log, as the original opened it.
            let p = app.dir("SMD").join(format!("{}.SMD", Local::now().format("%m-%d-%y")));
            if p.exists() {
                smd_open(app, p);
            }
        }
        "frmCollectionStatus" => {
            let s = "COLLECTION STATUS";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let m = vec![
                MenuItem::sub(
                    "menfile",
                    t("menfilecaption", "&File"),
                    vec![
                        MenuItem::new("menfileopen", t("menfileopencaption", "&Open Collection Log")),
                        MenuItem::new("menfileprintcurr", t("menfileprintcurrcaption", "&Print Current")),
                        MenuItem::new("menfileprint", t("menfileprintcaption", "Print Collection &Log")),
                        MenuItem::new("menfileexportcurrent", t("menfileexportcurrentcaption", "Export &current to comma delimitted file")),
                        MenuItem::new("menfileexportlog", t("menfileexportlogcaption", "&Export log to comma delimitted file")),
                        MenuItem::sep(),
                        MenuItem::new("menfileexit", t("menfileexitcaption", "E&xit")),
                    ],
                ),
                MenuItem::sub("mentransfer", t("mentransfercaption", "&Transfer"), vec![MenuItem::new("mentransdown", t("mentransdowncaption", "&Download")), MenuItem::new("mentransup", t("mentransupcaption", "&Upload"))]),
                MenuItem::new("mnuqe", t("mnuqecaption", "Quick Exit")),
            ];
            let mut caps: Vec<(String, String)> = vec![
                ("Label1".into(), t("label1caption", "Phone #")),
                ("cmdPhoneSearch".into(), t("cmdphonesearchcaption", "Search")),
                ("Label3".into(), t("label3caption", "Date")),
                ("Label2".into(), t("label2caption", "ID #")),
                ("cmdIDSearch".into(), t("cmdidsearchcaption", "Search")),
                ("Frame1".into(), t("frame1caption", "Last Collected")),
                ("Label16".into(), t("label16caption", "Date Collected")),
                ("Label17".into(), t("label17caption", "Amount Collected")),
                ("Label21".into(), t("label21caption", "Coin Box")),
                ("Label22".into(), t("label22caption", "Cumulative")),
                ("cmdClear".into(), t("cmdclearcaption", "&Clear")),
                ("cmdPrevious".into(), t("cmdprevious", "&Previous")),
                ("cmdNext".into(), t("cmdNext", "&Next")),
                ("lblCurrFile".into(), t("lblcurrfilecaption", "Working File:")),
            ];
            let mut frames = Vec::new();
            for i in 1..=9 {
                frames.push((format!("Frame3D{i}"), t(&format!("frame3d{i}caption"), "")));
            }
            for i in [4, 7, 8, 9, 12, 15, 20, 24] {
                caps.push((format!("Label{i}"), t(&format!("label{i}caption"), "Calls")));
            }
            for i in [5, 6, 10, 11, 13, 14, 19, 23] {
                caps.push((format!("Label{i}"), t(&format!("label{i}caption"), "Amount")));
            }
            let title = t("collectionstatuscaption", "Collection Status");
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(&n, -1, &c);
            }
            for (n, c) in frames {
                if !c.is_empty() && c != "NO STRING!" {
                    f.c(&n).text = c;
                }
            }
            f.c("Label18").caption = String::new();
            f.c("Label25").caption = String::new();
            let p = app.dir("CFW").join(format!("{}.cfw", Local::now().format("%m-%d-%y")));
            if p.exists() {
                cfw_open(app, p);
            }
        }
        "frmCallFlags" => {
            let s = "CALL FLAGS";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let m = vec![
                MenuItem::sub(
                    "mnufile",
                    t("mnufile", "&File"),
                    vec![
                        MenuItem::new("mnufileopen", t("mnufileopen", "&Open")),
                        MenuItem::new("mnufilesave", t("mnufilesave", "&Save && Clear")),
                        MenuItem::new("mnufileprint", t("mnufileprint", "&Print")),
                        MenuItem::new("menfileexport", t("menfileexport", "&Export to a comma delimitted file.")),
                        MenuItem::sep(),
                        MenuItem::new("mnufileexit", t("mnufileexit", "E&xit")),
                    ],
                ),
                MenuItem::new("mnuqe", t("mnuqe", "&Quick Exit")),
            ];
            let caps = [("cmdClear", t("cmdclear", "&Save && Clear")), ("cmdCancel", t("cmdcancel", "&Cancel")), ("Label3", format!("{}:", t("label3", "Phone Number"))), ("Label13", format!("{}:", t("label13", "ID Number")))];
            let title = t("cfTitle", "Call Flags");
            let new = app.g.flags_mode == "New";
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            f.c("lblWorkingFile").caption = String::new();
            f.c("cmdClear").enabled = new;
            if new {
                let p = app.dir("PLW").join("_BITFLGS.FLX");
                flags_open(app, p);
            }
        }
        "frmIncome" => {
            let s = "VIEW INCOMING";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let m = vec![
                MenuItem::sub(
                    "menfile",
                    t("menfile", "&File"),
                    vec![
                        MenuItem::new("menfileopen", t("menfileopen", "&Open")),
                        MenuItem::new("menfileprint", t("menfileprint", "&Print Current")),
                        MenuItem::new("menprintlog", t("menprintlog", "Print &Log")),
                        MenuItem::sep(),
                        MenuItem::new("menfileexit", t("menfileexit", "E&xit")),
                    ],
                ),
                MenuItem::new("mnuqe", t("mnuqe", "&Quick Exit")),
            ];
            let caps = [
                ("Label9", format!("{}:", t("label1", "Phone Number"))),
                ("Label13", format!("{}:", t("label13", "ID Number"))),
                ("Label2", t("label2", "ID Number")),
                ("Label5", t("label5", "Coin Box Status")),
                ("Label1", t("label1", "Phone Number")),
                ("Label3", t("label3", "Clock Data")),
                ("Label4", t("label4", "Coin Box Amount")),
                ("Label6", t("label6", "Active Status Flags")),
                ("Command1", t("command1", "&Previous")),
                ("Command2", t("command2", "&Next")),
                ("Command3", t("vSearch", "Search")),
                ("Command4", t("vSearch", "Search")),
            ];
            let title = if app.g.income_mode == "Receive" { t("vRID", "Receiving Incoming Data") } else { t("vReview", "Review Incoming Log") };
            let rx = app.modem.take_rx();
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            f.c("Label7").caption = String::new();
            f.c("Label8").caption = String::new();
            if !rx.is_empty() {
                f.c("List2").add_item(rx.trim().to_string());
            }
        }
        "frmLoadingCollectionsData" => {}
        _ => {}
    }
}

// ------------------------------------------------------------------ SMDR

fn smd_open(app: &mut App, p: PathBuf) {
    let b = std::fs::read(&p).unwrap_or_default();
    if b.is_empty() {
        let m = app.tor("SMDR", "fileisempty", "The file you have chosen is empty.");
        app.say(&m, "SMDR");
        return;
    }
    let rows = proto::smdr_file_rows(&b, Local::now().year());
    let mut by_phone: Vec<(String, Vec<proto::SmdrCall>)> = Vec::new();
    for (ph, call) in rows {
        match by_phone.iter_mut().find(|(x, _)| *x == ph) {
            Some((_, v)) => v.push(call),
            None => by_phone.push((ph, vec![call])),
        }
    }
    app.s.logs.smd = by_phone;
    app.s.logs.smd_at = 0;
    app.form("frmSMDR").c("lblWorkingFile").caption = p.display().to_string();
    app.s.logs.smd_file = Some(p);
    smd_show(app);
}

fn smd_line(c: &proto::SmdrCall) -> String {
    format!("{:<9} {:<6} {:<18} {:>6} {:>8}  {}", c.date, c.time, c.number, c.length, c.amount, c.call_type)
}

fn smd_show(app: &mut App) {
    let cur = app.s.logs.smd.get(app.s.logs.smd_at).cloned();
    let n = app.s.logs.smd.len();
    let at = app.s.logs.smd_at;
    let f = app.form("frmSMDR");
    let l = f.c("List1");
    l.clear();
    if let Some((ph, calls)) = cur {
        f.c("txtPhoneNumber").text = ph;
        let l = f.c("List1");
        for c in &calls {
            l.add_item(smd_line(c));
        }
    }
    f.c("cmdPreviousPhone").enabled = at > 0;
    f.c("cmdNextPhone").enabled = at + 1 < n;
}

/// The next/previous day's .SMD file in C:\TFW\SMD.
fn smd_day(app: &mut App, step: i64) {
    let cur = app.s.logs.smd_file.as_ref().and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_string()));
    let mut d = cur.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%m-%d-%y").ok()).unwrap_or_else(|| Local::now().date_naive());
    for _ in 0..366 {
        d += chrono::Duration::days(step);
        let p = app.dir("SMD").join(format!("{}.SMD", d.format("%m-%d-%y")));
        if p.exists() {
            smd_open(app, p);
            return;
        }
    }
    let m = app.tor("SMDR", "nomatchfound", "No match found.");
    app.say(&m, "SMDR");
}

fn smd_event(app: &mut App, ev: Ev) {
    match ev {
        Ev::Close => app.unload("frmSMDR"),
        Ev::Click(c, _) => match c.as_str() {
            "cmdPreviousPhone" if app.s.logs.smd_at > 0 => {
                app.s.logs.smd_at -= 1;
                smd_show(app);
            }
            "cmdNextPhone" if app.s.logs.smd_at + 1 < app.s.logs.smd.len() => {
                app.s.logs.smd_at += 1;
                smd_show(app);
            }
            "Command1" => smd_day(app, -1),
            "Command2" => smd_day(app, 1),
            "Command3" => {
                let key: String = app.form("frmSMDR").text("txtPhoneNumber").chars().filter(|c| c.is_ascii_digit()).collect();
                match app.s.logs.smd.iter().position(|(p, _)| p.chars().filter(|c| c.is_ascii_digit()).collect::<String>().ends_with(&key) && !key.is_empty()) {
                    Some(i) => {
                        app.s.logs.smd_at = i;
                        smd_show(app);
                    }
                    None => {
                        let (m, t) = (app.tor("SMDR", "nomatchfound", "No match found."), app.tor("SMDR", "phonenumbersearch", "Phone Number Search"));
                        app.say(&m, &t);
                    }
                }
            }
            "Command4" => {
                let (m, t) = (app.tor("SMDR", "nomatchfound", "No match found."), app.tor("SMDR", "idnumbersearch", "ID Number Search"));
                app.say(&m, &t);
            }
            _ => {}
        },
        Ev::Menu(m) => match m.as_str() {
            "menfileopen" => {
                let t = app.tor("SMDR", "opensmdrdata", "Open SMDR Data");
                if let Some(p) = app.file_dialog(false, &t, "SMDR File (*.SMD)", "smd", "SMD", "") {
                    smd_open(app, p);
                }
            }
            "menfileprintcurr" | "menfileprintfile" | "menfileexportcurr" | "menfileexportfile" => {
                let all = m.ends_with("file");
                let data: Vec<(String, Vec<proto::SmdrCall>)> = if all { app.s.logs.smd.clone() } else { app.s.logs.smd.get(app.s.logs.smd_at).cloned().into_iter().collect() };
                if data.is_empty() {
                    let m2 = app.tor("SMDR", "smdrNothing", "There is nothing to print.");
                    return app.say(&m2, "SMDR");
                }
                if m.contains("print") {
                    let mut lines = vec![app.tor("SMDR", "smdrTitle", "SMDR"), String::new()];
                    for (ph, calls) in &data {
                        lines.push(format!("{} {ph}", app.tor("SMDR", "smdrLogFor", "Log For")));
                        lines.extend(calls.iter().map(smd_line));
                        lines.push(format!("{}: {}", app.tor("SMDR", "smdrCalls", "Calls"), calls.len()));
                        lines.push(String::new());
                    }
                    app.print("SMDR", &lines);
                } else if let Some(p) = app.file_dialog(true, "Export", "Text (*.TXT)", "txt", "SMD", "") {
                    let mut out = Vec::new();
                    for (ph, calls) in &data {
                        for c in calls {
                            out.push(format!("\"{ph}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"", c.date, c.time, c.number, c.length, c.amount, c.code, c.posted, c.call_type));
                        }
                    }
                    let _ = std::fs::write(p, out.join("\r\n"));
                }
            }
            "menfileexit" => app.unload("frmSMDR"),
            "mentransup" => app.transfer("frmSMDR", "upload", Op::ReadSmdr),
            "mentransclear" => {
                let q = app.t("TELELINK", "goverwrite", "This will overwrite the data currently stored in your phone.  Do you want to continue?");
                let t = app.g.title.clone();
                app.msg("frmSMDR", "clear", &q, 36, &t);
            }
            _ => {}
        },
        _ => {}
    }
}

// ------------------------------------------------------------------ Collection Status

fn money(c: u64) -> String {
    format!("{}.{:02}", c / 100, c % 100)
}

fn cfw_open(app: &mut App, p: PathBuf) {
    match files::read_cfw(&p) {
        Ok(v) if !v.is_empty() => {
            app.s.logs.cfw = v;
            app.s.logs.cfw_at = 0;
            app.form("frmCollectionStatus").c("Label25").caption = p.display().to_string();
            app.s.logs.cfw_file = Some(p);
            cfw_show(app);
        }
        _ => {
            let m = app.tor("COLLECTION STATUS", "notcfw", "This is not a .CFW file.");
            app.say(&m, "Collection");
        }
    }
}

fn cfw_show(app: &mut App) {
    let r = app.s.logs.cfw.get(app.s.logs.cfw_at).cloned().unwrap_or_default();
    let date = app.s.logs.cfw_file.as_ref().and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_string())).unwrap_or_default();
    let n = app.s.logs.cfw.len();
    let at = app.s.logs.cfw_at;
    let f = app.form("frmCollectionStatus");
    f.c("txtPhoneBox").text = r.phone.clone();
    f.c("meIDBox").set_masked(&r.id);
    f.c("Label18").caption = date;
    f.c("MaskedEdit1").text = money(r.coin_box);
    f.c("MaskedEdit2").text = money(r.cumulative);
    // Instant refunds have their own counters; the other call classes sit in the phone's
    // collection block, whose layout isn't known: they stay blank (ESTIMATED).
    f.c("MaskedEdit11").text = r.refund_calls.to_string();
    f.c("MaskedEdit12").text = money(r.refund_cents);
    f.c("cmdPrevious").enabled = at > 0;
    f.c("cmdNext").enabled = at + 1 < n;
}

fn cfw_event(app: &mut App, ev: Ev) {
    let s = "COLLECTION STATUS";
    match ev {
        Ev::Close => app.unload("frmCollectionStatus"),
        Ev::Click(c, _) => match c.as_str() {
            "cmdPrevious" if app.s.logs.cfw_at > 0 => {
                app.s.logs.cfw_at -= 1;
                cfw_show(app);
            }
            "cmdNext" if app.s.logs.cfw_at + 1 < app.s.logs.cfw.len() => {
                app.s.logs.cfw_at += 1;
                cfw_show(app);
            }
            "cmdClear" => {
                app.s.logs.cfw.clear();
                app.s.logs.cfw_file = None;
                app.form("frmCollectionStatus").c("Label25").caption = String::new();
                cfw_show(app);
            }
            "cmdCalculate" => {
                // Last Collected: the newest earlier log where the coin box went down.
                let phone = app.form("frmCollectionStatus").text("txtPhoneBox");
                let mut found = None;
                let mut d = Local::now().date_naive();
                let mut prev_box: Option<u64> = None;
                for _ in 0..400 {
                    let p = app.dir("CFW").join(format!("{}.cfw", d.format("%m-%d-%y")));
                    if let Ok(v) = files::read_cfw(&p) {
                        if let Some(r) = v.iter().find(|r| r.phone == phone) {
                            if let Some(pb) = prev_box {
                                if pb < r.coin_box {
                                    found = Some((d, r.coin_box));
                                    break;
                                }
                            }
                            prev_box = Some(r.coin_box);
                        }
                    }
                    d -= chrono::Duration::days(1);
                }
                let f = app.form("frmCollectionStatus");
                match found {
                    Some((d, amt)) => {
                        f.c("txtDateCollected").text = d.format("%m-%d-%Y").to_string();
                        f.c("txtAmountCollected").text = money(amt);
                    }
                    None => {
                        let m = app.tor(s, "mfpNoData", "Never collected according to available records.");
                        app.say(&m, "Collection");
                    }
                }
            }
            "cmdPhoneSearch" | "cmdIDSearch" => {
                let f = app.form("frmCollectionStatus");
                let (ph, id) = (f.text("txtPhoneBox"), f.get("meIDBox").clip_text());
                let pos = if c == "cmdPhoneSearch" { app.s.logs.cfw.iter().position(|r| r.phone.contains(ph.trim()) && !ph.trim().is_empty()) } else { app.s.logs.cfw.iter().position(|r| r.id == id) };
                match pos {
                    Some(i) => {
                        app.s.logs.cfw_at = i;
                        cfw_show(app);
                    }
                    None => {
                        let m = app.tor(s, "nomatchfound", "No match found.");
                        app.say(&m, "Collection");
                    }
                }
            }
            _ => {}
        },
        Ev::Menu(m) => match m.as_str() {
            "menfileopen" => {
                let t = app.tor(s, "opencollectionlog", "Open Collection Log");
                if let Some(p) = app.file_dialog(false, &t, "Collection Log (*.CFW)", "cfw", "CFW", "") {
                    cfw_open(app, p);
                }
            }
            "menfileprintcurr" | "menfileprint" => {
                let recs: Vec<files::CollectionRec> = if m == "menfileprint" { app.s.logs.cfw.clone() } else { app.s.logs.cfw.get(app.s.logs.cfw_at).cloned().into_iter().collect() };
                if recs.is_empty() {
                    let m2 = app.tor(s, "nothingtoprint", "There is nothing to print.");
                    return app.say(&m2, "Collection");
                }
                let mut lines = vec![app.tor(s, "mfpcollectionstatus", "COLLECTION STATUS"), String::new()];
                for r in recs {
                    lines.push(format!("{:<15} ID {:<7} Coin Box {:>10}  Cumulative {:>10}", r.phone, r.id, money(r.coin_box), money(r.cumulative)));
                }
                app.print("Collection Status", &lines);
            }
            "menfileexportcurrent" | "menfileexportlog" => {
                if let Some(p) = app.file_dialog(true, "Export", "Text (*.TXT)", "txt", "CFW", "") {
                    let body: Vec<String> = app.s.logs.cfw.iter().map(|r| format!("\"{}\",\"{}\",{},{}", r.phone, r.id, money(r.coin_box), money(r.cumulative))).collect();
                    let _ = std::fs::write(p, body.join("\r\n"));
                }
            }
            "menfileexit" => app.unload("frmCollectionStatus"),
            "mentransup" => app.transfer("frmCollectionStatus", "upload", Op::ReadCollection),
            "mentransdown" => {
                let q = app.t("TELELINK", "goverwrite", "This will overwrite the data currently stored in your phone.  Do you want to continue?");
                let t = app.g.title.clone();
                app.msg("frmCollectionStatus", "clear", &q, 36, &t);
            }
            _ => {}
        },
        _ => {}
    }
}

// ------------------------------------------------------------------ Call flags

fn flags_open(app: &mut App, p: PathBuf) {
    let rows = files::read_flags(&p);
    app.s.logs.flags = rows;
    app.s.logs.flags_file = Some(p.clone());
    let no = app.tor("CALL FLAGS", "cfNoFlags", "There are no flags to view within this file.");
    let end = app.tor("CALL FLAGS", "cfEndOfFlags", "End Of Flags");
    let rows = app.s.logs.flags.clone();
    let f = app.form("frmCallFlags");
    f.c("lblWorkingFile").caption = p.display().to_string();
    let l = f.c("List4Display");
    l.clear();
    for r in &rows {
        let g = |i: usize| r.get(i).cloned().unwrap_or_default();
        l.add_item(format!("{:<14}{:<10}{:<26}{:<27}{}", g(0), g(1), g(2), g(3), g(4)));
    }
    if rows.is_empty() {
        l.add_item(no);
    } else {
        l.add_item(format!("   <<< {end} >>>"));
    }
}

fn flags_event(app: &mut App, ev: Ev) {
    match ev {
        Ev::Close => app.unload("frmCallFlags"),
        Ev::Click(c, i) => match c.as_str() {
            "cmdCancel" => app.unload("frmCallFlags"),
            "cmdClear" => flags_save_clear(app),
            "gbtnSortCol" => {
                let k = i.max(0) as usize;
                app.s.logs.flags.sort_by(|a, b| a.get(k).cmp(&b.get(k)));
                let f = app.form("frmCallFlags");
                for j in 0..4 {
                    f.ci("gbtnSortCol", j).value = (j == i) as i32;
                }
                if let Some(p) = app.s.logs.flags_file.clone() {
                    let rows = app.s.logs.flags.clone();
                    flags_open(app, p);
                    app.s.logs.flags = rows;
                }
            }
            _ => {}
        },
        Ev::Menu(m) => match m.as_str() {
            "mnufileopen" => {
                let t = app.tor("CALL FLAGS", "openflagfile", "Open Flag File");
                if let Some(p) = app.file_dialog(false, &t, "Flag File (*.FLG)", "flg", "PLW", "") {
                    flags_open(app, p);
                }
            }
            "mnufilesave" => flags_save_clear(app),
            "mnufileprint" => {
                let lines = app.form("frmCallFlags").get("List4Display").list.clone();
                app.print("Call Flags", &lines);
            }
            "menfileexport" => {
                if let Some(p) = app.file_dialog(true, "Export", "Text (*.TXT)", "txt", "PLW", "") {
                    let body: Vec<String> = app.s.logs.flags.iter().map(|r| r.iter().map(|x| format!("\"{x}\"")).collect::<Vec<_>>().join(",")).collect();
                    let _ = std::fs::write(p, body.join("\r\n"));
                }
            }
            "mnufileexit" => app.unload("frmCallFlags"),
            _ => {}
        },
        _ => {}
    }
}

/// Save & Clear: the new flags go to a dated .FLG file and the new-flags file is removed.
fn flags_save_clear(app: &mut App) {
    let src = app.dir("PLW").join("_BITFLGS.FLX");
    let dst = app.dir("PLW").join(format!("{}.FLG", Local::now().format("%m-%d-%y")));
    let mut b = std::fs::read(&dst).unwrap_or_default();
    b.extend(std::fs::read(&src).unwrap_or_default());
    match std::fs::write(&dst, b) {
        Ok(()) => {
            let _ = std::fs::remove_file(&src);
            app.unload("frmCallFlags");
        }
        Err(e) => app.say(&format!("Error {e}"), "Call Flags"),
    }
}

// ------------------------------------------------------------------ dispatch

pub fn event(app: &mut App, name: &str, ev: Ev) {
    match name {
        "frmSMDR" => smd_event(app, ev),
        "frmCollectionStatus" => cfw_event(app, ev),
        "frmCallFlags" => flags_event(app, ev),
        "frmIncome" => match ev {
            Ev::Close => app.unload(name),
            Ev::Menu(m) if m == "menfileexit" => app.unload(name),
            _ => {}
        },
        _ => {
            if ev == Ev::Close {
                app.unload(name);
            }
        }
    }
}

pub fn answer(app: &mut App, name: &str, tag: &str, a: Answer) {
    if tag == "clear" && a == Answer::Yes {
        match name {
            "frmSMDR" => app.transfer(name, "clear", Op::ClearSmdr),
            "frmCollectionStatus" => app.transfer(name, "clear", Op::ClearCounters),
            _ => {}
        }
    }
}

pub fn done(app: &mut App, name: &str, _what: &str, r: Result<Res, String>) {
    let title = app.form(name).caption.clone();
    let phone = app.modem.connected().map(|(t, _)| (t.shown, t.id)).unwrap_or_default();
    match &r {
        Ok(Res::Smdr(recs)) => {
            let _ = std::fs::create_dir_all(app.dir("SMD"));
            let p = app.dir("SMD").join(format!("{}.SMD", Local::now().format("%m-%d-%y")));
            let _ = files::append_smd(&p, &phone.0, recs);
            smd_open(app, p);
        }
        Ok(Res::Collection(c)) => {
            let rec = files::CollectionRec {
                phone: phone.0.clone(),
                id: phone.1.clone(),
                time: Local::now().format("%H:%M:%S").to_string(),
                coin_box: c.coin_box_cents.unwrap_or(0),
                cumulative: c.cumulative_cents.unwrap_or(0),
                refund_calls: c.refund_calls.unwrap_or(0),
                refund_cents: c.refund_cents.unwrap_or(0),
                raw: c.raw.clone(),
            };
            let _ = std::fs::create_dir_all(app.dir("CFW"));
            let p = app.dir("CFW").join(format!("{}.cfw", Local::now().format("%m-%d-%y")));
            let _ = files::append_cfw(&p, &rec);
            cfw_open(app, p);
            let n = app.s.logs.cfw.len();
            app.s.logs.cfw_at = n.saturating_sub(1);
            cfw_show(app);
        }
        _ => {}
    }
    super::report_outcome(app, &title, &r);
}

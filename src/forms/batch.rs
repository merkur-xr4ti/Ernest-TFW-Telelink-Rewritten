//! BATSETUP.FRM / BATSETCH.FRM (Batch File Setup), BATCHEXE.FRM (Execute Batch),
//! BATSCHED.FRM (AutoCall schedule) and INCSCHED.FRM (AutoAnswer schedule).

use crate::app::App;
use crate::files::{self, Batch, Phone};
use crate::modem::{DialConf, Job, Op, Res, Target};
use crate::proto::{self, Clock};
use crate::vb::{Answer, Ev, MenuItem};
use chrono::{Datelike, Local, Timelike};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Default)]
pub struct BatchState {
    pub setup: Batch,
    pub setup_file: Option<PathBuf>,
    /// The phone list of the last batch run (for View Batch Log).
    pub last_list: Option<PathBuf>,
    /// Phones flagged for re-run from the batch log.
    pub rerun: Option<Vec<Phone>>,
    pub run: Option<Run>,
    /// AutoCall: the batch files still to run today and the attempts per phone.
    pub auto_queue: Vec<String>,
    pub auto_attempts: i32,
    /// Batch Schedule: the day whose file list is being chosen.
    pub sched_day: Option<i32>,
    pub sched_files: Vec<String>,
    pub start_at: Option<chrono::NaiveDateTime>,
}

pub struct Run {
    pub list_path: PathBuf,
    pub phones: Vec<Phone>,
    pub at: usize,
    pub attempt: i32,
    pub attempts: i32,
    pub steps: Vec<(String, Op)>,
    pub step: usize,
    pub job: Option<Job>,
    pub dialing: bool,
    pub firmware: String,
    pub skip: bool,
    pub log: PathBuf,
    pub auto: bool,
}

const SEC: &str = "BATCH FILE SETUP";
const EX: &str = "BATCH EXECUTE";

/// The control-file kinds of Text1(i): dialog title, filter, extension, folder.
fn control_file(i: i32) -> Option<(&'static str, &'static str, &'static str, &'static str)> {
    Some(match i {
        2..=4 => ("Select Script", "AOS Scripts (*.AOS)", "aos", "AOS"),
        5 => ("Select CSN Table", "CSN Table (*.CSN)", "csn", "CSN"),
        6 => ("Select Programmable Features", "Features (*.FEA)", "fea", "FEA"),
        7 => ("Select Rate Table", "Rate Table (*.RTF)", "rtf", "RTF"),
        8 => ("Select Speed Dial Numbers", "Speed Dial (*.SDN)", "sdn", "SDN"),
        10 => ("Select Phone List", "Phone List (*.PLW)", "plw", "PLW"),
        12 => ("Select Scrolling Message", "Scrolling Message (*.SCR)", "scr", "MSG"),
        14 => ("Select Firmware", "Firmware (*.BIN)", "bin", "FW"),
        _ => return None,
    })
}

pub fn load(app: &mut App, name: &str) {
    match name {
        "frmBatchSetup" | "frmBatchSetupChina" => {
            let t = |k: &str, d: &str| app.tor(SEC, k, d);
            let m = vec![
                MenuItem::sub(
                    "menfile",
                    t("menfile", "&File"),
                    vec![
                        MenuItem::new("menfileopen", t("menfileopen", "&Open")),
                        MenuItem::new("menfilesave", t("menfilesave", "&Save")),
                        MenuItem::new("menfilesaveas", t("menfilesaveas", "Save &As")),
                        MenuItem::new("menfileprint", t("menfileprint", "&Print")),
                        MenuItem::sep(),
                        MenuItem::new("menfileexit", t("menfileexit", "E&xit")),
                    ],
                ),
                MenuItem::new("mnuqe", t("mnuqe", "&Quick Exit")),
            ];
            let mut caps = vec![("Label2".to_string(), -1, t("label2", "Batch Operation")), ("Label3".into(), -1, t("label3", "Control Files")), ("Option1".into(), -1, t("option1", "Active Phone List")), ("Label4".into(), -1, t("label4", "Working File:"))];
            for i in [0, 1, 5, 6, 7, 8, 9] {
                caps.push(("Check1".into(), i, app.tor(SEC, &format!("check1({i})"), "")));
            }
            caps.push(("Check2".into(), 0, t("check2(0)", "Clear Counters")));
            caps.push(("Check2".into(), 1, t("check2(1)", "Clear SMDR")));
            let title = t("batchfilesetup", "Batch File Setup");
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, i, c) in caps {
                if !c.is_empty() {
                    if let Some(k) = f.find(&n, i) {
                        f.ctls[k].caption = c;
                    }
                }
            }
            f.c("Label1").caption = String::new();
            app.s.batch.setup = Batch::new();
            app.s.batch.setup_file = None;
        }
        "frmBatchExec" => {
            let t = |k: &str, d: &str| app.tor(EX, k, d);
            let m = vec![MenuItem::new("mnuqe", t("mnuqe", "&Quick Exit"))];
            let caps = [
                ("Label1", t("label1", "Current File Name")),
                ("Label2", t("label2", "Call Attempts")),
                ("Label3", t("label3", "Batch File List")),
                ("optRunNow", t("optrunnow", "Run Now")),
                ("optRunLater", t("optrunlater", "Run Later")),
                ("chkUnpolled", t("chkunpolled", "Call unpolled phones only")),
                ("Frame1", t("frame1", "Start Time -----> (Click Ok to Start)")),
                ("cmdOk", t("cmdok", "&OK")),
                ("cmdCancel", t("cmdcancel", "&Cancel")),
                ("cmdSkip", t("cmdskip", "&Skip")),
                ("cmdViewUnpolled", t("cmdviewunpolled", "List Phones")),
            ];
            let title = t("frmbatchexec", "Execute Batch");
            let bxw = app.dir("BXW").display().to_string();
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            f.c("meCallAttempts").set_masked("1");
            let l = f.c("lstBatchFileList");
            l.list = crate::vb::list_files(&bxw, "*.bxw");
            f.c("cmdSkip").enabled = false;
            f.c("cmdOk").default = true;
            if let Some(list) = app.s.batch.rerun.take() {
                // Re-Run Batch from the batch log: the flagged phones only.
                app.s.batch.rerun = Some(list);
                app.form(name).c("txtBatchFileName").text = "(Re-Run flagged phones)".into();
            }
        }
        "frmBatchSchedule" => {
            let s = "BATCH SCHEDULE SETUP";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let caps = [
                ("chkStartup", format!(" {}", t("chkstartup", "Turn AutoCall on when Telelink starts"))),
                ("Label1", t("label1", "Day of Week")),
                ("Label2", t("label2", "Start Time")),
                ("Label3", t("label3", "Batch Files to Run")),
                ("lblCallAttempts", format!("{}:", t("lblcallattempts", "Call Attempts per Phone"))),
                ("cmdSaveList", t("cmdsavelist", "&Save")),
                ("cmdQuitList", t("cmdquitlist", "&Quit")),
                ("cmdOk", t("cmdok", "&Ok")),
                ("cmdCancel", t("cmdcancel", "&Cancel")),
                ("Label4", t("xMilitaryTime", "Military Time (HHMM)")),
            ];
            let title = t("frmbatchschedule", "Batch Schedule Setup");
            let days = super::day_names(app);
            let p = app.prof.clone();
            let bxw = app.dir("BXW").display().to_string();
            let f = app.form(name);
            f.caption = title;
            f.menus = vec![MenuItem::new("mnuqe", app_qe())];
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            for (i, d) in days.iter().enumerate() {
                let v = p.get("AutoCall", d);
                let (time, files) = v.split_once(',').map(|(a, b)| (a.to_string(), b.to_string())).unwrap_or((v.clone(), String::new()));
                let f2 = &mut *f;
                f2.ci("chkDayOfWeek", i as i32).caption = d.clone();
                f2.ci("chkDayOfWeek", i as i32).value = (!v.is_empty()) as i32;
                f2.ci("edtStartTime", i as i32).set_masked(&time);
                f2.ci("lblFileList", i as i32).caption = files;
            }
            let r = p.get("AutoCall", "RetryCount");
            f.c("txtCallAttempts").text = if r.is_empty() { "1".into() } else { r };
            f.c("chkStartup").value = (p.get("AutoCall", "AutoStart").trim() == "1") as i32;
            let fl = f.c("fileBatchFiles");
            fl.path = bxw;
            fl.pattern = "*.bxw".into();
            for n in ["fileBatchFiles", "cmdSaveList", "cmdQuitList"] {
                f.c(n).visible = false;
            }
            app.s.batch.sched_day = None;
        }
        "frmIncomingSchedule" => {
            let s = "INCOMING CALL SCHEDULE SETUP";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let mut caps: Vec<(&str, i32, String)> = vec![
                ("chkStartup", -1, format!(" {}", t("chkstartup", "Turn AutoAnswer on when Telelink starts"))),
                ("chkAutoAnswer", 0, t("chkautoanswer(0)", "Accept D3 Incoming Calls")),
                ("chkAutoAnswer", 1, t("chkautoanswer(1)", "Accept D1 Incoming Calls")),
                ("cmdOk", -1, t("cmdok", "&Ok")),
                ("cmdCancel", -1, t("cmdcancel", "&Cancel")),
            ];
            for i in 0..5 {
                caps.push(("lblAutoAnswer", i, t(&format!("lblautoanswer({i})"), "")));
            }
            let title = t("frmincomingschedule", "Incoming Call Schedule Setup");
            let p = app.prof.clone();
            let (d3, d1) = (app.g.d3.clone(), app.g.d1.clone());
            let f = app.form(name);
            f.caption = title;
            f.menus = vec![MenuItem::new("mnuqe", app_qe())];
            for (n, i, c) in caps {
                if !c.is_empty() && c != "NO STRING!" {
                    f.set_caption(n, i, &c);
                }
            }
            for (k, key) in [(0, d3), (1, d1)] {
                let v = p.get("AutoAnswer", &key);
                let on = !v.is_empty() && v != ",";
                let (a, b) = v.split_once(',').unwrap_or((&v, ""));
                f.ci("chkAutoAnswer", k).value = on as i32;
                f.ci("edtSchedTime", k * 2).set_masked(a);
                f.ci("edtSchedTime", k * 2 + 1).set_masked(b);
            }
            f.c("chkStartup").value = (p.get("AutoAnswer", "AutoStart").trim() == "1") as i32;
            sched_visibility(app);
        }
        _ => {}
    }
}

fn app_qe() -> String {
    "&Quick Exit".into()
}

fn sched_visibility(app: &mut App) {
    let f = app.form("frmIncomingSchedule");
    for k in 0..2 {
        let on = f.valuei("chkAutoAnswer", k) == 1;
        f.ci("edtSchedTime", k * 2).visible = on;
        f.ci("edtSchedTime", k * 2 + 1).visible = on;
        f.ci("lblAutoAnswer", k * 2).visible = on;
        f.ci("lblAutoAnswer", k * 2 + 1).visible = on;
    }
    let any = f.valuei("chkAutoAnswer", 0) == 1 || f.valuei("chkAutoAnswer", 1) == 1;
    f.ci("lblAutoAnswer", 4).visible = any;
}

// ------------------------------------------------------------------ Batch File Setup

fn setup_show(app: &mut App, name: &str) {
    let b = app.s.batch.setup.clone();
    let f = app.form(name);
    for i in 0..10 {
        if f.has("Check1", i) {
            f.ci("Check1", i).value = (b.flags[i as usize] & 1) as i32;
        }
        if f.has("Text1", i) {
            f.ci("Text1", i).text = b.files[i as usize].clone();
        }
    }
    for i in [0, 1] {
        if f.has("Check2", i) {
            f.ci("Check2", i).value = ((b.flags[i as usize] & 2) / 2) as i32;
        }
    }
    f.c("Option1").value = (b.flags[10] & 1) as i32;
    f.ci("Text1", 10).text = b.files[10].clone();
    if f.has("Check1", 12) {
        f.ci("Check1", 12).value = (b.flags[11] & 1) as i32;
        f.ci("Text1", 12).text = b.files[11].clone();
    }
    if f.has("Check1", 14) {
        f.ci("Check1", 14).value = (b.flags[12] & 1) as i32;
        f.ci("Check2", 14).value = ((b.flags[12] & 2) / 2) as i32;
        f.ci("Text1", 14).text = b.files[12].clone();
    }
    let file = app.s.batch.setup_file.as_ref().map(|p| p.display().to_string()).unwrap_or_default();
    app.form(name).c("Label1").caption = file;
}

fn setup_collect(app: &mut App, name: &str) {
    let f = app.form(name);
    let mut b = Batch::new();
    for i in 0..10usize {
        let c1 = if f.has("Check1", i as i32) { f.valuei("Check1", i as i32) as u8 } else { 0 };
        let c2 = if i < 2 && f.has("Check2", i as i32) { f.valuei("Check2", i as i32) as u8 } else { 0 };
        b.flags[i] = c1 + 2 * c2;
        if (2..=8).contains(&i) && f.has("Text1", i as i32) {
            b.files[i] = f.texti("Text1", i as i32);
        }
    }
    b.flags[10] = f.value("Option1") as u8;
    b.files[10] = f.texti("Text1", 10);
    if f.has("Check1", 12) {
        b.flags[11] = f.valuei("Check1", 12) as u8;
        b.files[11] = if b.flags[11] == 1 { f.texti("Text1", 12) } else { String::new() };
    }
    if f.has("Check1", 14) {
        b.flags[12] = (f.valuei("Check1", 14) + 2 * f.valuei("Check2", 14)) as u8;
        b.files[12] = if b.flags[12] & 1 == 1 { f.texti("Text1", 14) } else { String::new() };
    }
    app.s.batch.setup = b;
}

fn setup_save(app: &mut App, name: &str, ask: bool) {
    setup_collect(app, name);
    let b = app.s.batch.setup.clone();
    if b.flags[10] == 0 || b.files[10].trim().is_empty() {
        app.msg("", "", "You must specify a Phone List.", 0, "Batch Setup");
        return;
    }
    let p = match (ask, app.s.batch.setup_file.clone()) {
        (false, Some(p)) => p,
        _ => match app.file_dialog(true, "Save Batch File", "Batch File (*.BXW)", "bxw", "BXW", "") {
            Some(p) => p,
            None => return,
        },
    };
    if !p.extension().is_some_and(|e| e.eq_ignore_ascii_case("bxw")) {
        app.msg("", "", "You must use the .BXW extension to save this file.", 0, "Batch Setup");
        return;
    }
    match files::write_bxw(&p, &b) {
        Ok(()) => {
            app.s.batch.setup_file = Some(p);
            app.changed(name, false);
            setup_show(app, name);
        }
        Err(e) => app.msg("", "", &format!("Unknown Error: {e}"), 16, "BatchSetup"),
    }
}

fn setup_event(app: &mut App, name: &str, ev: Ev) {
    match ev {
        Ev::Close => {
            // BATSETUP.FRM's Form_QueryUnload asks; the China version (BATSETCH.FRM) doesn't.
            if app.doc(name).dirty && name == "frmBatchSetup" {
                app.msg(name, "save_exit", "Do you wish to save your changes before exiting?", 35, "Batch File Setup");
            } else {
                app.unload(name);
            }
        }
        Ev::Change(..) => app.changed(name, true),
        Ev::Click(c, i) => {
            app.changed(name, true);
            let idx = if c == "Option1" { 10 } else { i };
            let checked = match c.as_str() {
                "Check1" => app.form(name).valuei("Check1", i) == 1,
                "Option1" => true,
                _ => false,
            };
            if checked {
                if let Some((title, filter, ext, dir)) = control_file(idx) {
                    let title = if (2..=4).contains(&idx) { format!("{title} # {}", idx - 1) } else { title.to_string() };
                    match app.file_dialog(false, &title, filter, ext, dir, "") {
                        Some(p) => {
                            if !p.extension().is_some_and(|e| e.eq_ignore_ascii_case(ext)) {
                                app.msg("", "", "You have specified an incorrect extension.", 0, "Batch Setup");
                            } else {
                                app.form(name).ci("Text1", idx).text = p.display().to_string();
                            }
                        }
                        None => {
                            if c == "Check1" && app.form(name).texti("Text1", idx).is_empty() {
                                app.form(name).ci("Check1", i).value = 0;
                            }
                        }
                    }
                }
            }
        }
        Ev::Menu(m) => match m.as_str() {
            "menfileopen" => {
                let Some(p) = app.file_dialog(false, "Open Batch File", "Batch File (*.BXW)", "bxw", "BXW", "") else { return };
                if !p.extension().is_some_and(|e| e.eq_ignore_ascii_case("bxw")) {
                    app.msg("", "", "That is not a BXW File", 0, "Batch Setup");
                    return;
                }
                match files::read_bxw(&p) {
                    Ok(b) => {
                        app.s.batch.setup = b;
                        app.s.batch.setup_file = Some(p);
                        setup_show(app, name);
                        app.changed(name, false);
                    }
                    Err(e) => app.msg("", "", &format!("Unknown Error: {e}"), 16, "BatchSetup"),
                }
            }
            "menfilesave" => setup_save(app, name, false),
            "menfilesaveas" => setup_save(app, name, true),
            "menfileprint" => {
                setup_collect(app, name);
                let b = app.s.batch.setup.clone();
                let mut lines = vec!["Batch Setup".to_string(), format!("For the file: {}", app.s.batch.setup_file.as_ref().map(|p| p.display().to_string()).unwrap_or_default()), "      Batch Operation                        File Spec".into(), String::new()];
                let f = app.form(name);
                for i in 0..10 {
                    if f.has("Check1", i) {
                        let x = if f.valuei("Check1", i) == 1 { " X " } else { "   " };
                        lines.push(format!("{x}{:<40}{}", crate::vb::plain(&f.geti("Check1", i).caption), b.files[i as usize]));
                    }
                }
                lines.push(format!(" X {:<40}{}", "Active Phone List", b.files[10]));
                app.print("Batch Setup", &lines);
            }
            "menfileexit" => setup_event(app, name, Ev::Close),
            _ => {}
        },
        _ => {}
    }
}

// ------------------------------------------------------------------ Execute Batch

/// The operations of a batch file, in frmBatchExec's order.
fn batch_steps(app: &App, b: &Batch) -> Result<Vec<(String, Op)>, String> {
    let t = |k: &str, d: &str| app.tor(EX, k, d);
    let mut s: Vec<(String, Op)> = Vec::new();
    s.push((t("updatingfirmwareversion", "Updating Firmware Version..."), Op::ReadFirmware));
    if b.flags[0] & 1 == 1 {
        s.push((t("uploadingcollectiondata", "Uploading Collection Data..."), Op::ReadCollection));
    }
    if b.flags[0] & 2 == 2 {
        s.push((t("resettingcollectiondata", "Resetting Collection Data"), Op::ClearCounters));
    }
    if b.flags[1] & 1 == 1 {
        s.push((t("uploadingsmdrinfo", "Uploading SMDR Info..."), Op::ReadSmdr));
    }
    if b.flags[1] & 2 == 2 {
        s.push((t("clearingsmdrbuffer", "Clearing SMDR Buffer..."), Op::ClearSmdr));
    }
    if b.flags[12] & 1 == 1 && !b.files[12].is_empty() {
        let plan = std::fs::read(&b.files[12]).map_err(|e| e.to_string()).and_then(|f| proto::firmware_plan(&f))?;
        s.push((t("downloadingfirmware", "Downloading Firmware..."), Op::DownloadFirmware(plan, b.flags[12] & 2 == 2)));
    }
    for n in 1..=3usize {
        if b.flags[n + 1] & 1 == 1 && !b.files[n + 1].is_empty() {
            let a = files::read_aos(std::path::Path::new(&b.files[n + 1]))?;
            let steps: Vec<String> = a.codes.iter().filter(|c| c.as_str() != "FF").filter_map(|c| u8::from_str_radix(c, 16).ok()).map(proto::aos_text).collect();
            let bytes = proto::aos_bytes(&a.phone, &a.auth.replace('_', ""), &steps)?;
            s.push((t(&format!("downloadingaos{n}"), &format!("Downloading AOS #{n}...")), Op::WriteAos(n, bytes)));
        }
    }
    if b.flags[5] & 1 == 1 && !b.files[5].is_empty() {
        s.push((t("downloadingcsntable", "Downloading CSN Table..."), Op::WriteCsn(files::read_csn(std::path::Path::new(&b.files[5]))?)));
    }
    if b.flags[6] & 1 == 1 && !b.files[6].is_empty() {
        let ff = std::fs::read(&b.files[6]).map_err(|e| e.to_string()).and_then(|x| crate::fea::FeatureFile::parse(&x))?;
        let recs = ff.features.iter().filter(|x| !x.read_only()).map(|x| (x.address(), x.value_bytes().to_vec())).collect();
        s.push((t("downloadingfeatures", "Downloading Features..."), Op::WriteRecords(recs)));
    }
    if b.flags[7] & 1 == 1 && !b.files[7].is_empty() {
        s.push((t("downloadingratetable", "Downloading Rate Table..."), Op::WriteRate(std::fs::read(&b.files[7]).map_err(|e| e.to_string())?)));
    }
    if b.flags[8] & 1 == 1 && !b.files[8].is_empty() {
        let n = files::read_sdn(std::path::Path::new(&b.files[8]))?;
        let digits: Vec<String> = n.iter().map(|x| x.chars().filter(|c| c.is_ascii_digit() || *c == '*').collect()).collect();
        s.push((t("downloadingspeeddialnumbers", "Downloading Speed Dial Numbers..."), Op::WriteSpeedDial(digits)));
    }
    if b.flags[9] & 1 == 1 {
        let n = Local::now().naive_local();
        let day = proto::day_code(n.weekday().num_days_from_sunday());
        let c = Clock { yy: (n.year() % 100) as u8, mm: n.month() as u8, dd: n.day() as u8, day, hh: n.hour() as u8, mi: n.minute() as u8, ss: n.second() as u8 };
        s.push((t("downloadingtimedate", "Downloading Time & Date..."), Op::WriteClock(c)));
    }
    if b.flags[11] & 1 == 1 && !b.files[11].is_empty() {
        s.push((t("downloadingscrollingmessage", "Downloading Scrolling Message..."), Op::WriteScroll(files::read_scr(std::path::Path::new(&b.files[11]))?)));
    }
    Ok(s)
}

fn log_line(app: &mut App, s: &str) {
    if app.loaded("frmBatchExec") {
        let l = app.form("frmBatchExec").c("List1");
        l.list.push(s.to_string());
        l.list_index = l.list.len() as i32 - 1;
    }
    if let Some(r) = &app.s.batch.run {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&r.log) {
            let _ = writeln!(f, "{}  {s}", Local::now().format("%H:%M:%S"));
        }
    }
}

/// Start a batch file (frmBatchExec cmdOk / Telelink sub021A for AutoCall).
fn start_run(app: &mut App, bxw: &str, attempts: i32, unpolled_only: bool, auto: bool) -> Result<(), String> {
    let (batch, phones, list_path) = if let Some(list) = app.s.batch.rerun.take() {
        let p = app.doc("frmPhoneList").file.clone().unwrap_or_default();
        let b = if bxw.ends_with(".bxw") || bxw.ends_with(".BXW") { files::read_bxw(std::path::Path::new(bxw))? } else { app.s.batch.setup.clone() };
        (b, list, p)
    } else {
        let b = files::read_bxw(std::path::Path::new(bxw)).map_err(|_| app.tor(EX, "beNotBXW", "This is not a BXW file."))?;
        let lp = PathBuf::from(b.files[10].trim());
        let mut l = files::read_plw(&lp)?;
        if unpolled_only {
            l.retain(|p| p.status != 1);
        }
        (b, l, lp)
    };
    let steps = batch_steps(app, &batch)?;
    let log = app.dir("BXW").join(format!("{}.log", Local::now().format("%m-%d-%y")));
    app.s.batch.last_list = Some(list_path.clone());
    app.g.batch_running = true;
    app.s.batch.run = Some(Run { list_path, phones, at: 0, attempt: 0, attempts: attempts.max(1), steps, step: 0, job: None, dialing: false, firmware: String::new(), skip: false, log, auto });
    let m = app.tor(EX, "executingbatchfile", "Executing Batch File...");
    log_line(app, &format!("{m} {bxw}"));
    next_call(app);
    Ok(())
}

fn next_call(app: &mut App) {
    let Some(r) = app.s.batch.run.as_mut() else { return };
    if r.at >= r.phones.len() {
        finish_run(app);
        return;
    }
    let p = r.phones[r.at].clone();
    r.attempt += 1;
    r.step = 0;
    r.dialing = true;
    r.skip = false;
    let digits = p.digits();
    let d3 = !p.ptype.eq_ignore_ascii_case("D1");
    let local = digits.len() <= 7;
    let target = Target { dial: digits.clone(), shown: p.phone.clone(), id: p.id.clone(), d3 };
    let conf = DialConf {
        init_addons: app.g.init_addons.clone(),
        d3_addons: app.g.d3_addons.clone(),
        d1_addons: app.g.d1_addons.clone(),
        pbx: app.g.pbx.clone(),
        dcw: app.g.dcw.clone(),
        timeout: (if local { app.g.dial_local } else { app.g.dial_ld }).max(30) as u32,
    };
    let n = app.s.batch.run.as_ref().map(|r| (r.at, r.phones.len())).unwrap_or((0, 0));
    if app.loaded("frmBatchExec") {
        let f = app.form("frmBatchExec");
        f.c("Gauge1").visible = true;
        f.c("Gauge1").value = (n.0 * 100 / n.1.max(1)) as i32;
        f.c("cmdSkip").enabled = true;
    }
    log_line(app, &format!("Calling {} (ID {})...", p.phone, p.id));
    let job = app.modem.dial(target, conf);
    if let Some(r) = app.s.batch.run.as_mut() {
        r.job = Some(job);
    }
}

fn phone_done(app: &mut App, status: u8, text: &str) {
    let Some(r) = app.s.batch.run.as_mut() else { return };
    let retry = status != 1 && status != 8 && status != 7 && r.attempt < r.attempts;
    if retry {
        let at = r.at;
        log_line(app, &format!("{text} - trying again"));
        let _ = at;
        next_call(app);
        return;
    }
    let fw = r.firmware.clone();
    let i = r.at;
    r.phones[i].status = status;
    if !fw.is_empty() {
        r.phones[i].firmware = fw.chars().take(6).collect();
    }
    // Write the status into the phone list (the batch log).
    let path = r.list_path.clone();
    let updated = r.phones[i].clone();
    if let Ok(mut all) = files::read_plw(&path) {
        if let Some(x) = all.iter_mut().find(|x| x.phone == updated.phone) {
            *x = updated.clone();
            let _ = files::write_plw(&path, &all);
        }
    }
    log_line(app, &format!("{}: {text}", updated.phone));
    if let Some(r) = app.s.batch.run.as_mut() {
        r.at += 1;
        r.attempt = 0;
        r.firmware.clear();
    }
    next_call(app);
}

fn finish_run(app: &mut App) {
    let auto = app.s.batch.run.as_ref().map(|r| r.auto).unwrap_or(false);
    app.s.batch.run = None;
    app.g.batch_running = false;
    log_line(app, &app.tor(EX, "endoflist", "<<< End Of List >>>"));
    if app.loaded("frmBatchExec") {
        let f = app.form("frmBatchExec");
        f.c("Gauge1").value = 100;
        f.c("cmdSkip").enabled = false;
        f.c("cmdOk").enabled = true;
    }
    if auto {
        if let Some(next) = (!app.s.batch.auto_queue.is_empty()).then(|| app.s.batch.auto_queue.remove(0)) {
            let n = app.s.batch.auto_attempts;
            if let Err(e) = start_run(app, &next, n, false, true) {
                log_line(app, &e);
                finish_run(app);
            }
        } else {
            app.g.autocall_running = false;
        }
    }
}

/// AutoCall: run today's batch files one after another.
pub fn auto_run(app: &mut App, files: Vec<String>, attempts: i32) {
    if app.loaded("frmBatchExec") && app.s.batch.run.is_none() {
        app.unload("frmBatchExec");
    }
    app.s.batch.auto_queue = files;
    app.s.batch.auto_attempts = attempts;
    if app.s.batch.auto_queue.is_empty() {
        app.g.autocall_running = false;
        return;
    }
    let first = app.s.batch.auto_queue.remove(0);
    if let Err(e) = start_run(app, &first, attempts, false, true) {
        log_line(app, &e);
        app.g.autocall_running = false;
    }
}

pub fn tick(app: &mut App) {
    // The clock line and a Run Later start.
    if app.visible("frmBatchExec") {
        let now = Local::now().naive_local();
        let cur = app.tor(EX, "currenttime", "Current time is now: ");
        app.form("frmBatchExec").c("lblDateTime").caption = format!("{cur} {}", now.format("%m-%d-%Y %I:%M:%S %p"));
        if let Some(at) = app.s.batch.start_at {
            if now >= at {
                app.s.batch.start_at = None;
                exec_ok(app, true);
            }
        }
    }
    let Some(r) = app.s.batch.run.as_ref() else { return };
    let (dialing, skip) = (r.dialing, r.skip);
    let Some(job) = r.job.clone() else { return };
    let st = job.status();
    if app.loaded("frmBatchExec") {
        app.form("frmBatchExec").c("lblStatus").caption = st;
    }
    let Some(out) = job.take() else { return };
    match out {
        Err(e) => {
            if !dialing {
                app.modem.hang_up();
            }
            let code = if skip { 8 } else { files::status_code_of(&e) };
            phone_done(app, code, &e);
        }
        Ok(res) => {
            let r = app.s.batch.run.as_mut().unwrap();
            if dialing {
                r.dialing = false;
                if let Res::Connected { firmware } = &res {
                    r.firmware = firmware.clone();
                }
            } else {
                // Keep what the step brought back.
                let phone = r.phones[r.at].clone();
                match &res {
                    Res::Firmware { running, .. } => r.firmware = running.clone(),
                    Res::Collection(c) => {
                        let rec = files::CollectionRec {
                            phone: phone.phone.clone(),
                            id: phone.id.clone(),
                            time: Local::now().format("%H:%M:%S").to_string(),
                            coin_box: c.coin_box_cents.unwrap_or(0),
                            cumulative: c.cumulative_cents.unwrap_or(0),
                            refund_calls: c.refund_calls.unwrap_or(0),
                            refund_cents: c.refund_cents.unwrap_or(0),
                            raw: c.raw.clone(),
                        };
                        let p = app.dir("CFW").join(format!("{}.cfw", Local::now().format("%m-%d-%y")));
                        let _ = std::fs::create_dir_all(app.dir("CFW"));
                        let _ = files::append_cfw(&p, &rec);
                    }
                    Res::Smdr(recs) => {
                        let p = app.dir("SMD").join(format!("{}.SMD", Local::now().format("%m-%d-%y")));
                        let _ = std::fs::create_dir_all(app.dir("SMD"));
                        let _ = files::append_smd(&p, &phone.phone, recs);
                    }
                    _ => {}
                }
                let r = app.s.batch.run.as_mut().unwrap();
                r.step += 1;
            }
            let r = app.s.batch.run.as_mut().unwrap();
            if r.step < r.steps.len() {
                let (label, op) = r.steps[r.step].clone();
                let job = app.modem.run(op);
                app.s.batch.run.as_mut().unwrap().job = Some(job);
                log_line(app, &label);
            } else {
                app.modem.hang_up();
                let ok = app.tor("PHONE LIST", "plST", "Successful transfer");
                phone_done(app, 1, &ok);
            }
        }
    }
}

fn exec_ok(app: &mut App, now: bool) {
    let f = app.form("frmBatchExec");
    let name = f.text("txtBatchFileName");
    let attempts: i32 = f.get("meCallAttempts").clip_text().parse().unwrap_or(0);
    let unpolled = f.value("chkUnpolled") == 1;
    let later = f.value("optRunLater") == 1;
    if name.trim().is_empty() {
        let m = app.tor(EX, "selectafilefirst", "You must select a file first.");
        let t = app.tor(EX, "batchexec", "Batch Exec");
        app.msg("", "", &m, 48, &t);
        return;
    }
    if attempts < 1 {
        let m = app.tor(EX, "enternumberofattempts", "You must enter the number of call attempts.");
        let t = app.tor(EX, "batchexec", "Batch Exec");
        app.msg("", "", &m, 48, &t);
        return;
    }
    if later && !now {
        let f = app.form("frmBatchExec");
        f.c("Frame1").visible = true;
        let txt = f.get("lblScheduledStartTime").caption.clone();
        if let Ok(t) = chrono::NaiveTime::parse_from_str(txt.trim(), "%H:%M") {
            let d = Local::now().naive_local();
            let mut at = d.date().and_time(t);
            if at < d {
                at += chrono::Duration::days(1);
            }
            app.s.batch.start_at = Some(at);
            let w = app.tor(EX, "beWaiting", "Waiting for start time");
            app.form("frmBatchExec").c("lblStatus").caption = w;
            app.form("frmBatchExec").c("lblStatus").visible = true;
        } else {
            let m = app.tor(EX, "beEnterStart", "Enter the batch start time.");
            app.form("frmBatchExec").c("lblScheduledStartTime").caption = "00:00".into();
            app.msg("", "", &format!("{m} (HH:MM, click the box)"), 48, "Batch Exec");
        }
        return;
    }
    let path = if name.contains('\\') { name.clone() } else { app.dir("BXW").join(&name).display().to_string() };
    app.form("frmBatchExec").c("cmdOk").enabled = false;
    app.form("frmBatchExec").c("lblStatus").visible = true;
    if let Err(e) = start_run(app, &path, attempts, unpolled, false) {
        app.form("frmBatchExec").c("cmdOk").enabled = true;
        app.msg("", "", &e, 48, "Batch Exec");
    }
}

fn exec_event(app: &mut App, ev: Ev) {
    match ev {
        Ev::Close => exec_event(app, Ev::Click("cmdCancel".into(), -1)),
        Ev::Click(c, _) | Ev::DblClick(c, _) => match c.as_str() {
            "lstBatchFileList" => {
                if let Some(s) = app.form("frmBatchExec").get("lstBatchFileList").selected().cloned() {
                    app.form("frmBatchExec").c("txtBatchFileName").text = s;
                }
            }
            "optRunLater" => {
                let f = app.form("frmBatchExec");
                f.c("Frame1").visible = true;
                if f.get("lblScheduledStartTime").caption.is_empty() {
                    f.c("lblScheduledStartTime").caption = Local::now().format("%H:%M").to_string();
                }
            }
            "optRunNow" => app.form("frmBatchExec").c("Frame1").visible = false,
            "lblScheduledStartTime" => {
                // Step the start time by 15 minutes (the time entry box of the original isn't known).
                let f = app.form("frmBatchExec");
                let t = chrono::NaiveTime::parse_from_str(f.get("lblScheduledStartTime").caption.trim(), "%H:%M").unwrap_or_default();
                let t = t + chrono::Duration::minutes(15);
                f.c("lblScheduledStartTime").caption = t.format("%H:%M").to_string();
            }
            "cmdOk" => exec_ok(app, false),
            "cmdSkip" => {
                if let Some(r) = app.s.batch.run.as_mut() {
                    r.skip = true;
                    r.attempt = r.attempts;
                    if let Some(j) = &r.job {
                        j.cancel.store(true, std::sync::atomic::Ordering::SeqCst);
                    }
                }
            }
            "cmdCancel" => {
                if app.s.batch.run.is_some() {
                    let m = app.tor(EX, "surecancel", "All pending batches will be cancelled.  Are you sure you want to cancel?");
                    app.msg("frmBatchExec", "cancel", &m, 36, "Batch Exec");
                } else {
                    app.s.batch.start_at = None;
                    app.unload("frmBatchExec");
                }
            }
            "cmdViewUnpolled" => {
                let name = app.form("frmBatchExec").text("txtBatchFileName");
                let path = app.dir("BXW").join(&name);
                if let Ok(b) = files::read_bxw(&path) {
                    if let Ok(l) = files::read_plw(std::path::Path::new(b.files[10].trim())) {
                        let w = app.tor(EX, "willbecalled", "The following phones will be called:");
                        let mut lines = vec![w];
                        lines.extend(l.iter().filter(|p| p.status != 1).map(|p| p.phone.clone()));
                        app.say(&lines.join("\n"), "Batch Exec");
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}

// ------------------------------------------------------------------ AutoCall schedule

fn sched_ok(app: &mut App) {
    let s = "BATCH SCHEDULE SETUP";
    let days = super::day_names(app);
    let f = app.form("frmBatchSchedule");
    let mut entries = Vec::new();
    for i in 0..7 {
        let on = f.valuei("chkDayOfWeek", i) == 1;
        let t = f.geti("edtStartTime", i).clip_text();
        let files = f.geti("lblFileList", i).caption.clone();
        entries.push((on, t, files));
    }
    let attempts = f.text("txtCallAttempts");
    let auto = f.value("chkStartup") == 1;
    let title = app.tor(s, "bAutoCallSetup", "AutoCall Setup");
    for (i, (on, t, files)) in entries.iter().enumerate() {
        if !on {
            continue;
        }
        let hh: i32 = t.get(0..2).and_then(|x| x.parse().ok()).unwrap_or(-1);
        let mm: i32 = t.get(2..4).and_then(|x| x.parse().ok()).unwrap_or(-1);
        if !(0..=23).contains(&hh) {
            let m = app.tor(s, "bHours", "Hours must be between 00 and 23.");
            return app.msg("", "", &m, 48, &title);
        }
        if !(0..=59).contains(&mm) {
            let m = app.tor(s, "bMin", "Minutes must be between 00 and 59.");
            return app.msg("", "", &m, 48, &title);
        }
        if files.trim().is_empty() {
            let m = app.tor(s, "bSelect", "You must select at least one batch file to enable this day.");
            return app.msg("", "", &format!("{m} ({})", days[i]), 48, &title);
        }
    }
    for (i, (on, t, files)) in entries.iter().enumerate() {
        let v = if *on { format!("{t},{files}") } else { String::new() };
        if !app.prof.set("AutoCall", &days[i], &v) {
            let m = app.tor(s, "bNoSave", "Unable to save schedule for ");
            return app.msg("", "", &format!("{m}{}", days[i]), 48, &title);
        }
    }
    app.prof.set("AutoCall", "RetryCount", attempts.trim());
    app.prof.set("AutoCall", "AutoStart", if auto { "1" } else { "0" });
    app.s.main.ac_day.clear();
    app.unload("frmBatchSchedule");
}

fn sched_event(app: &mut App, ev: Ev) {
    match ev {
        Ev::Close => app.unload("frmBatchSchedule"),
        Ev::Click(c, i) | Ev::DblClick(c, i) => match c.as_str() {
            "lblFileList" => {
                app.s.batch.sched_day = Some(i);
                let cur = app.form("frmBatchSchedule").geti("lblFileList", i).caption.clone();
                app.s.batch.sched_files = cur.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                let f = app.form("frmBatchSchedule");
                for n in ["fileBatchFiles", "cmdSaveList", "cmdQuitList"] {
                    f.c(n).visible = true;
                }
            }
            "fileBatchFiles" => {
                if let (Some(d), Some(s)) = (app.s.batch.sched_day, app.form("frmBatchSchedule").get("fileBatchFiles").selected().cloned()) {
                    if let Some(k) = app.s.batch.sched_files.iter().position(|x| x.eq_ignore_ascii_case(&s)) {
                        app.s.batch.sched_files.remove(k);
                    } else {
                        app.s.batch.sched_files.push(s.to_ascii_lowercase());
                    }
                    let cap = app.s.batch.sched_files.join(",");
                    app.form("frmBatchSchedule").ci("lblFileList", d).caption = cap;
                }
            }
            "cmdSaveList" | "cmdQuitList" => {
                let f = app.form("frmBatchSchedule");
                for n in ["fileBatchFiles", "cmdSaveList", "cmdQuitList"] {
                    f.c(n).visible = false;
                }
                if let (Some(d), true) = (app.s.batch.sched_day, c == "cmdSaveList") {
                    let any = !app.s.batch.sched_files.is_empty();
                    app.form("frmBatchSchedule").ci("chkDayOfWeek", d).value = any as i32;
                }
                app.s.batch.sched_day = None;
            }
            "cmdOk" => sched_ok(app),
            "cmdCancel" => app.unload("frmBatchSchedule"),
            _ => {}
        },
        _ => {}
    }
}

// ------------------------------------------------------------------ AutoAnswer schedule

fn incsched_ok(app: &mut App) {
    let s = "INCOMING CALL SCHEDULE SETUP";
    let f = app.form("frmIncomingSchedule");
    let mut w = Vec::new();
    for k in 0..2 {
        w.push((f.valuei("chkAutoAnswer", k) == 1, f.geti("edtSchedTime", k * 2).clip_text(), f.geti("edtSchedTime", k * 2 + 1).clip_text()));
    }
    let auto = f.value("chkStartup") == 1;
    let title = app.tor(s, "icss", "Incoming Call Schedule Setup");
    for (on, a, b) in &w {
        if !on {
            continue;
        }
        for t in [a, b] {
            let hh: i32 = t.get(0..2).and_then(|x| x.parse().ok()).unwrap_or(-1);
            let mm: i32 = t.get(2..4).and_then(|x| x.parse().ok()).unwrap_or(-1);
            if !(0..=23).contains(&hh) {
                let m = app.t("TELELINK", "HoursWarning", "Hours must be between 00 and 23.");
                return app.msg("", "", &m, 48, &title);
            }
            if !(0..=59).contains(&mm) {
                let m = app.t("TELELINK", "MinutesWarning", "Minutes must be between 00 and 59.");
                return app.msg("", "", &m, 48, &title);
            }
        }
        if b <= a {
            let m = app.tor(s, "endtimegreater", "The end time must be later than the start time.");
            return app.msg("", "", &m, 48, &title);
        }
    }
    if w[0].0 && w[1].0 && w[0].1 < w[1].2 && w[1].1 < w[0].2 {
        let m = app.tor(s, "icConflict", "There is a conflict between the D1 and D3 schedules.");
        return app.msg("", "", &m, 48, &title);
    }
    let (d3, d1) = (app.g.d3.clone(), app.g.d1.clone());
    for (k, key) in [(0usize, d3), (1, d1)] {
        let v = if w[k].0 { format!("{},{}", w[k].1, w[k].2) } else { String::new() };
        app.prof.set("AutoAnswer", &key, &v);
    }
    app.prof.set("AutoAnswer", "AutoStart", if auto { "1" } else { "0" });
    app.s.main.aa_day.clear();
    app.unload("frmIncomingSchedule");
}

pub fn event(app: &mut App, name: &str, ev: Ev) {
    match name {
        "frmBatchSetup" | "frmBatchSetupChina" => setup_event(app, name, ev),
        "frmBatchExec" => exec_event(app, ev),
        "frmBatchSchedule" => sched_event(app, ev),
        "frmIncomingSchedule" => match ev {
            Ev::Click(c, _) if c == "chkAutoAnswer" => sched_visibility(app),
            Ev::Click(c, _) if c == "cmdOk" => incsched_ok(app),
            Ev::Click(c, _) if c == "cmdCancel" => app.unload(name),
            Ev::Close => app.unload(name),
            _ => {}
        },
        _ => {}
    }
}

pub fn answer(app: &mut App, name: &str, tag: &str, a: Answer) {
    match (name, tag) {
        (_, "save_exit") => match a {
            Answer::Yes => {
                setup_save(app, name, false);
                if !app.doc(name).dirty {
                    app.unload(name);
                }
            }
            Answer::No => app.unload(name),
            _ => {}
        },
        ("frmBatchExec", "cancel") if a == Answer::Yes => {
            if let Some(r) = app.s.batch.run.as_ref() {
                if let Some(j) = &r.job {
                    j.cancel.store(true, std::sync::atomic::Ordering::SeqCst);
                }
            }
            app.s.batch.run = None;
            app.s.batch.auto_queue.clear();
            app.g.batch_running = false;
            app.g.autocall_running = false;
            app.modem.hang_up();
            app.unload("frmBatchExec");
        }
        _ => {}
    }
}

#[allow(dead_code)]
fn started() -> Instant {
    Instant::now()
}

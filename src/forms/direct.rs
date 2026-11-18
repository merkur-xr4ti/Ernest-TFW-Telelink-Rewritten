//! The Direct windows that move one thing to or from the connected phone: SPEEDIAL.FRM,
//! TIMEDATE.FRM, SCROLLIT.FRM, T_RATE.FRM and FIRMWARE.FRM.

use crate::app::App;
use crate::modem::{Op, Res};
use crate::proto::{self, Clock};
use crate::vb::{Answer, Ev, MenuItem};
use chrono::{Datelike, Local, Timelike};
use std::path::PathBuf;

fn file_menu(app: &App, sec: &str, with_save: bool, print: bool) -> MenuItem {
    let t = |k: &str, d: &str| app.tor(sec, k, d);
    let mut items = vec![MenuItem::new("menfileopen", t("menfileopen", "&Open"))];
    if with_save {
        items.push(MenuItem::new("menfilesave", t("menfilesave", "&Save")));
        items.push(MenuItem::new("menfilesaveas", t("menfilesaveas", &t("mnufilesaveas", "Save &As"))));
    }
    if print {
        items.push(MenuItem::new("menfileprint", t("menfileprint", "&Print")));
    }
    items.push(MenuItem::sep());
    items.push(MenuItem::new("menfileexit", t("menfileexit", "E&xit")));
    MenuItem::sub("menfile", t("menfile", "&File"), items)
}

pub fn load(app: &mut App, name: &str) {
    match name {
        "frmSpeedDial" => {
            let m = vec![file_menu(app, "SPEED DIAL", true, true), super::transfer_menu(app, "SPEED DIAL", vec![]), MenuItem::new("mnuqe", app.tor("TELELINK", "vQuickExit", "&Quick Exit"))];
            let f = app.form(name);
            f.menus = m;
            f.c("Label11").caption = String::new();
        }
        "frmTimeDate" => {
            let s = "TIME DATE";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let m = vec![
                MenuItem::sub("menfile", t("menfile", "&File"), vec![MenuItem::new("menexit", t("menexit", "E&xit"))]),
                MenuItem::sub("mentimetransfer", t("mentimetransfer", "&Transfer"), vec![MenuItem::new("mentimedown", t("mentimedown", "&Download")), MenuItem::new("mentimeup", t("mentimeup", "&Upload"))]),
                MenuItem::new("mnuqe", t("mnuqe", "&Quick Exit")),
            ];
            let caps = [("Label3", t("label3", "Day")), ("Label2", t("label2", "Date")), ("Label1", t("label1", "Time")), ("cmdNow", t("cmdnow", "Current Date && Time").replace(" & ", " && "))];
            let title = t("frmtimedate", "Time & Date");
            // The phone's day order (Form_Load adds Friday first): ListIndex + 1 is the day code.
            let days = super::day_names(app);
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            let combo = f.c("Combo1");
            for k in [5usize, 6, 0, 1, 2, 3, 4] {
                combo.add_item(days[k].clone());
            }
            now_into(app);
        }
        "ScrollIt" => {
            let m = vec![file_menu(app, "SCROLL", true, false), super::transfer_menu(app, "SCROLL", vec![]), MenuItem::new("mnuqe", app.tor("TELELINK", "vQuickExit", "&Quick Exit"))];
            let f = app.form(name);
            f.menus = m;
            f.c("txtMessage").text = "Replace this message with your own message.".into();
        }
        "frmRateTable" => {
            let s = "RATE TABLE";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let m = vec![
                file_menu(app, s, true, false),
                super::transfer_menu(app, s, vec![MenuItem::new("mentranscenter", t("mentranscenter", "&Rate Center"))]),
                MenuItem::new("mnuqe", t("mnuqe", "&Quick Exit")),
            ];
            let caps = [("Label1", t("label1", "Rate Center NPA")), ("Label2", t("label2", "Rate Center Prefix")), ("Label3", t("label3", "Creation Date")), ("Label4", t("label4", "Uploading"))];
            let title = t("frmratetable", "Rate Table");
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            f.c("Label4").visible = false;
            f.c("Label5").caption = String::new();
        }
        "firmware" => {
            let s = "FIRMWARE";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let m = vec![
                MenuItem::sub("menfile", t("menfile", "&File"), vec![MenuItem::new("menfileopen", t("menfileopen", "Open")), MenuItem::sep(), MenuItem::new("menfileexit", t("menfileexit", "E&xit"))]),
                MenuItem::sub(
                    "menTransfer",
                    t("menTransfer", "&Transfer"),
                    vec![MenuItem::new("menTransferUpload", t("menTransferUpload", "&Upload Firmware Versions")), MenuItem::new("menTransferDownload", t("menTransferDownload", "&Download New Firmware"))],
                ),
                MenuItem::new("mnuqe", t("mnuqe", "&Quick Exit")),
            ];
            let caps = [
                ("control24", t("label1", "Current Firmware Versions")),
                ("control11", t("label4", "Loaded For Download to Secondary")),
                ("control13", t("label6", "Primary")),
                ("control14", t("label7", "Secondary")),
                ("control15", t("option1", "Run Primary After Download")),
                ("control16", t("option2", "Run Secondary After Download")),
                ("control20", t("active", "ACTIVE")),
                ("control21", t("active", "ACTIVE")),
                ("control8", t("activenext", "Active Next Power-up")),
                ("control25", t("activenext", "Active Next Power-up")),
            ];
            let title = t("firmware", "Firmware");
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            for n in ["control9", "control10", "control12", "control19"] {
                f.c(n).caption = String::new();
            }
            for n in ["control20", "control21", "control8", "control25", "control18"] {
                f.c(n).visible = false;
            }
            f.c("control18").max = 100;
        }
        _ => {}
    }
}

fn now_into(app: &mut App) {
    let now = Local::now().naive_local();
    let wd = now.weekday().num_days_from_sunday() as i32;
    let f = app.form("frmTimeDate");
    f.c("MaskedEdit1").set_masked(&now.format("%m/%d/%y").to_string());
    f.c("MaskedEdit2").set_masked(&now.format("%H:%M:%S").to_string());
    f.c("Combo1").list_index = (wd + 2) % 7;
    let i = f.get("Combo1").list_index as usize;
    f.c("Combo1").text = f.get("Combo1").list[i].clone();
}

fn working_label(name: &str) -> &'static str {
    match name {
        "frmSpeedDial" => "Label11",
        "frmRateTable" => "Label5",
        _ => "",
    }
}

fn set_file(app: &mut App, name: &str, p: Option<PathBuf>) {
    let wf = app.tor("RATE TABLE", "workingfile", "Working File: ");
    let lbl = working_label(name);
    if !lbl.is_empty() {
        let cap = p.as_ref().map(|p| format!("{wf}{}", p.display())).unwrap_or_default();
        app.form(name).c(lbl).caption = cap;
    }
    if name == "ScrollIt" {
        let cap = match &p {
            Some(p) => format!("Scrolling Message - {}", p.display()),
            None => "Scrolling Message".into(),
        };
        app.form(name).caption = cap;
    }
    app.doc(name).file = p;
}

fn speed_numbers(app: &mut App) -> Vec<String> {
    let f = app.form("frmSpeedDial");
    (0..10).map(|i| f.geti("MaskedEdit1", i).clip_text()).collect()
}

fn open(app: &mut App, name: &str) {
    let (title, filter, ext, dir) = match name {
        "frmSpeedDial" => ("Open Speed Dial Numbers", "Speed Dial (*.SDN)", "sdn", "SDN"),
        "ScrollIt" => ("Open Scrolling Message", "Scrolling Message (*.SCR)", "scr", "MSG"),
        "frmRateTable" => ("Open Rate Table", "Rate Table (*.RTF)", "rtf", "RTF"),
        _ => ("Open Firmware", "Firmware (*.BIN)", "bin", "FW"),
    };
    let Some(p) = app.file_dialog(false, title, filter, ext, dir, "") else { return };
    match name {
        "frmSpeedDial" => match crate::files::read_sdn(&p) {
            Ok(nums) => {
                let f = app.form(name);
                for (i, n) in nums.iter().enumerate() {
                    f.ci("MaskedEdit1", i as i32).set_masked(n.trim());
                }
                set_file(app, name, Some(p));
                app.changed(name, false);
            }
            Err(e) => app.say(&e, "Speed Dial"),
        },
        "ScrollIt" => match crate::files::read_scr(&p) {
            Ok(s) => {
                app.form(name).c("txtMessage").text = s;
                set_file(app, name, Some(p));
                app.changed(name, false);
            }
            Err(e) => app.say(&e, "Scrolling Message"),
        },
        "frmRateTable" => match std::fs::read(&p) {
            Ok(b) if !b.is_empty() => {
                app.s.report.rate = b;
                set_file(app, name, Some(p));
            }
            _ => {
                let m = app.tor("RATE TABLE", "nortf", "This is not an .RTF file.");
                app.say(&m, "Rate Table");
            }
        },
        _ => match std::fs::read(&p).map_err(|e| e.to_string()).and_then(|b| proto::firmware_plan(&b)) {
            Ok(plan) => {
                let f = app.form("firmware");
                f.c("control12").caption = plan.version.clone();
                app.s.report.firmware = Some(plan);
                app.doc("firmware").file = Some(p);
            }
            Err(e) => {
                let m = app.tor("FIRMWARE", "nofirm", "This is not a firmware file");
                app.say(&format!("{m}\n\n{e}"), "Firmware");
            }
        },
    }
}

fn save(app: &mut App, name: &str, ask: bool) {
    let (title, filter, ext, dir) = match name {
        "frmSpeedDial" => ("Save Speed Dial Numbers", "Speed Dial (*.SDN)", "sdn", "SDN"),
        "ScrollIt" => ("Save Scrolling Message", "Scrolling Message (*.SCR)", "scr", "MSG"),
        _ => ("Save Rate Table", "Rate Table (*.RTF)", "rtf", "RTF"),
    };
    let p = match (ask, app.doc(name).file.clone()) {
        (false, Some(p)) => p,
        _ => match app.file_dialog(true, title, filter, ext, dir, "") {
            Some(p) => p,
            None => return,
        },
    };
    if !p.extension().is_some_and(|e| e.eq_ignore_ascii_case(ext)) {
        app.say(&format!("You must use the .{} extension to save this file.", ext.to_ascii_uppercase()), title);
        return;
    }
    let r = match name {
        "frmSpeedDial" => {
            let f = app.form(name);
            let nums: Vec<String> = (0..10).map(|i| f.geti("MaskedEdit1", i).text.clone()).collect();
            crate::files::write_sdn(&p, &nums)
        }
        "ScrollIt" => {
            let t = app.form(name).text("txtMessage");
            crate::files::write_scr(&p, &t)
        }
        _ => {
            if app.s.report.rate.is_empty() {
                let m = app.tor("RATE TABLE", "rtfnotinmemory", "No rate table in memory.");
                app.say(&m, "Rate Table");
                return;
            }
            std::fs::write(&p, &app.s.report.rate).map_err(|e| e.to_string())
        }
    };
    match r {
        Ok(()) => {
            set_file(app, name, Some(p));
            app.changed(name, false);
        }
        Err(e) => app.say(&format!("Unknown Error: {e}"), title),
    }
}

/// Form_QueryUnload: Speed Dial asks (sub0F7D, titled with the program's name); Scrolling
/// Message asks about its message (sub08BC); Time & Date, Rate Table and Firmware close at once.
fn close(app: &mut App, name: &str) {
    let dirty = app.doc(name).dirty;
    let title = app.g.title.clone();
    if dirty && name == "frmSpeedDial" {
        app.msg(name, "save_exit", "Do you wish to save your changes before exiting?", 35, &title);
    } else if dirty && name == "ScrollIt" {
        app.msg(name, "save_exit", "Do you wish to save this message before exiting?", 35, "Save Message?");
    } else {
        app.unload(name);
    }
}

pub fn event(app: &mut App, name: &str, ev: Ev) {
    let overwrite = app.t("TELELINK", "goverwrite", "This will overwrite the data currently stored in your phone.  Do you want to continue?");
    let title = app.g.title.clone();
    match ev {
        Ev::Close => close(app, name),
        Ev::Change(..) => {
            if name != "frmTimeDate" {
                app.changed(name, true);
            }
        }
        Ev::Menu(m) => match m.as_str() {
            "menfileopen" => open(app, name),
            "menfilesave" => save(app, name, false),
            "menfilesaveas" => save(app, name, true),
            "menfileprint" if name == "frmSpeedDial" => {
                let nums = speed_numbers(app);
                let mut lines = vec!["SPEED DIAL NUMBERS".to_string(), String::new()];
                for (i, n) in nums.iter().enumerate() {
                    lines.push(format!("Speed Dial {i}   {n}"));
                }
                app.print("Speed Dial", &lines);
            }
            "menfileexit" | "menexit" => close(app, name),
            "mentransdown" | "mentimedown" | "menTransferDownload" => {
                if name == "firmware" && app.s.report.firmware.is_none() {
                    let m = app.t("TELELINK", "xNothingToDownload", "There is nothing to download.");
                    app.say(&m, "Firmware");
                } else if name == "frmRateTable" && app.s.report.rate.is_empty() {
                    let m = app.tor("RATE TABLE", "rtNothingToDownload", "There is nothing to download.");
                    app.say(&m, "Rate Table");
                } else {
                    app.msg(name, "download", &overwrite, 36, &title);
                }
            }
            "mentransup" | "mentimeup" | "menTransferUpload" => {
                let op = match name {
                    "frmSpeedDial" => Op::ReadSpeedDial,
                    "frmTimeDate" => Op::ReadClock,
                    "ScrollIt" => Op::ReadScroll,
                    "frmRateTable" => Op::ReadRate,
                    _ => Op::ReadFirmware,
                };
                if name == "frmRateTable" {
                    app.form(name).c("Label4").visible = true;
                }
                app.transfer(name, "upload", op);
            }
            "mentranscenter" => {
                let t = app.form(name);
                let (npa, pre) = (t.text("Text1"), t.text("Text2"));
                app.say(&format!("Rate Center NPA {npa}, Prefix {pre}"), "Rate Center");
            }
            _ => {}
        },
        Ev::Click(c, _) => match (name, c.as_str()) {
            ("frmTimeDate", "cmdNow") => now_into(app),
            ("ScrollIt", "cmdTest") => test_scroll(app),
            _ => {}
        },
        _ => {}
    }
}

/// cmdTest: scroll the message through a 20-character window like the phone's display.
fn test_scroll(app: &mut App) {
    let msg = app.form("ScrollIt").text("txtMessage");
    app.s.logs.scroll_test = Some((format!("{}{}", " ".repeat(20), msg), 0, std::time::Instant::now()));
    let f = app.form("ScrollIt");
    f.c("lblTestMessage").visible = true;
    f.c("shpTestBox").visible = true;
    f.c("txtMessage").visible = false;
}

pub fn scroll_tick(app: &mut App) {
    let Some((text, pos, at)) = app.s.logs.scroll_test.clone() else { return };
    if !app.loaded("ScrollIt") {
        app.s.logs.scroll_test = None;
        return;
    }
    if at.elapsed().as_millis() < 250 {
        return;
    }
    let chars: Vec<char> = text.chars().collect();
    if pos > chars.len() {
        app.s.logs.scroll_test = None;
        let f = app.form("ScrollIt");
        f.c("lblTestMessage").visible = false;
        f.c("shpTestBox").visible = false;
        f.c("txtMessage").visible = true;
        return;
    }
    let window: String = chars.iter().skip(pos).take(20).collect();
    app.form("ScrollIt").c("lblTestMessage").caption = format!("{window:<20}");
    app.s.logs.scroll_test = Some((text, pos + 1, std::time::Instant::now()));
}

pub fn answer(app: &mut App, name: &str, tag: &str, a: Answer) {
    match tag {
        "save_exit" => match a {
            Answer::Yes => {
                save(app, name, false);
                if !app.doc(name).dirty {
                    app.unload(name);
                }
            }
            Answer::No => app.unload(name),
            _ => {}
        },
        "download" if a == Answer::Yes => {
            let op = match name {
                "frmSpeedDial" => Op::WriteSpeedDial(speed_numbers(app)),
                "frmTimeDate" => {
                    let f = app.form(name);
                    let d = f.get("MaskedEdit1").clip_text();
                    let t = f.get("MaskedEdit2").clip_text();
                    let n = |s: &str, a: usize| s.get(a..a + 2).and_then(|x| x.parse::<u8>().ok()).unwrap_or(0);
                    let day = (f.get("Combo1").list_index.max(0) + 1) as u8;
                    Op::WriteClock(Clock { yy: n(&d, 4), mm: n(&d, 0), dd: n(&d, 2), day, hh: n(&t, 0), mi: n(&t, 2), ss: n(&t, 4) })
                }
                "ScrollIt" => Op::WriteScroll(app.form(name).text("txtMessage")),
                "frmRateTable" => Op::WriteRate(app.s.report.rate.clone()),
                _ => {
                    let sec = app.form("firmware").value("control16") == 1;
                    match app.s.report.firmware.clone() {
                        Some(p) => {
                            let cap = app.tor("TELELINK", "bDownloading", "Downloading");
                            let f = app.form("firmware");
                            f.c("control19").caption = cap;
                            f.c("control18").visible = true;
                            Op::DownloadFirmware(p, sec)
                        }
                        None => return,
                    }
                }
            };
            app.transfer(name, "download", op);
        }
        "save_upload" if a == Answer::Yes => save(app, name, true),
        _ => {}
    }
}

pub fn done(app: &mut App, name: &str, what: &str, r: Result<Res, String>) {
    let title = app.form(name).caption.clone();
    match (&r, name) {
        (Ok(Res::SpeedDial(n)), _) => {
            let f = app.form(name);
            for (i, x) in n.iter().enumerate().take(10) {
                f.ci("MaskedEdit1", i as i32).set_masked(x);
            }
            app.changed(name, true);
        }
        (Ok(Res::Clock(c)), _) => {
            let f = app.form(name);
            f.c("MaskedEdit1").set_masked(&format!("{:02}{:02}{:02}", c.mm, c.dd, c.yy));
            f.c("MaskedEdit2").set_masked(&format!("{:02}{:02}{:02}", c.hh, c.mi, c.ss));
            let k = (c.day.max(1) - 1).min(6) as i32;
            f.c("Combo1").list_index = k;
            f.c("Combo1").text = f.get("Combo1").list[k as usize].clone();
        }
        (Ok(Res::Scroll(s)), _) => {
            app.form(name).c("txtMessage").text = s.clone();
            app.changed(name, true);
        }
        (Ok(Res::Rate(b)), _) => {
            app.s.report.rate = b.clone();
            app.form(name).c("Label4").visible = false;
            let q = app.tor("RATE TABLE", "rtSave", "Do you wish to save this file before exiting?");
            let t = app.tor("RATE TABLE", "rtSaveUpload", "Save Upload?");
            app.msg(name, "save_upload", &q, 36, &t);
            return;
        }
        (Ok(Res::Firmware { primary, secondary, running }), _) => {
            let f = app.form("firmware");
            f.c("control9").caption = primary.clone();
            f.c("control10").caption = secondary.clone();
            f.c("control20").visible = running == primary;
            f.c("control21").visible = running == secondary && running != primary;
        }
        _ => {}
    }
    if name == "firmware" && what == "download" {
        let f = app.form("firmware");
        f.c("control18").visible = false;
        f.c("control19").caption = String::new();
    }
    if name == "frmRateTable" {
        app.form(name).c("Label4").visible = false;
    }
    super::report_outcome(app, &title, &r);
}

#[allow(dead_code)]
fn now_hms() -> String {
    let n = Local::now();
    format!("{:02}:{:02}:{:02}", n.hour(), n.minute(), n.second())
}

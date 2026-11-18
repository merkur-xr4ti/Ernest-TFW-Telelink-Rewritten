//! REPORT.FRM (Report Generator), FRMCDRGE.FRM / FRMCDRVI.FRM / CDRWORK.FRM (Call Detail
//! Records) and the small message forms FRMPROCE, FRMMDA and FRMUPDAT.

use crate::app::App;
use crate::files;
use crate::proto;
use crate::vb::{Answer, Ev, MenuItem};
use chrono::{Datelike, Local, NaiveDate};
use std::path::PathBuf;

#[derive(Default)]
pub struct ReportState {
    /// The rate table in memory (frmRateTable).
    pub rate: Vec<u8>,
    /// The firmware file loaded for download (firmware form).
    pub firmware: Option<proto::FirmwarePlan>,
    pub phone_list: Option<PathBuf>,
    /// Per phone: (phone, location, daily amounts, last coin box).
    pub rows: Vec<(String, String, Vec<(NaiveDate, f32)>, f32)>,
}

const S: &str = "REPORT GENERATOR";

pub fn load(app: &mut App, name: &str) {
    match name {
        "frmReport" => {
            let t = |k: &str, d: &str| app.tor(S, k, d);
            let m = vec![
                MenuItem::sub(
                    "menfile",
                    t("menfile", "&File"),
                    vec![
                        MenuItem::new("menfileopen", t("menfileopen", "&Open Phone List")),
                        MenuItem::new("mnufilesaveas", t("mnufilesaveas", "Save &Report As")),
                        MenuItem::new("mnuviewreport", t("mnuviewreport", "&View Report")),
                        MenuItem::new("menfilegraph", t("menfilegraph", "&Generate Graph")),
                        MenuItem::new("menfileprint", t("menfileprint", "&Print")),
                        MenuItem::new("menfileexport", t("menfileexport", "&Export to a comma delimitted file.")),
                        MenuItem::sep(),
                        MenuItem::new("menfileexit", t("menfileexit", "E&xit")),
                    ],
                ),
                MenuItem::sub(
                    "menoptions",
                    t("menoptions", "&Options"),
                    vec![MenuItem::new("menoptdaily", t("menoptdaily", "&Daily Revenue")), MenuItem::new("menoptcumm", t("menoptcumm", "&Cummulative")), MenuItem::new("menoptcoin", t("menoptcoin", "Coin &Box"))],
                ),
                MenuItem::sub(
                    "mengraphtype",
                    "&Graph Types",
                    vec![
                        MenuItem::new("mengraph2dpie", t("mengraph2dpie", "2D-&Pie")),
                        MenuItem::new("mengraph3dpie", t("mengraph3dpie", "3D-P&ie")),
                        MenuItem::new("mengraph2dbar", t("mengraph2dbar", "2D-&Bar")),
                        MenuItem::new("mengraph3dbar", t("mengraph3dbar", "3D-B&ar")),
                        MenuItem::new("mengraphline", t("mengraphline", "&Line")),
                    ],
                ),
                MenuItem::new("mnuqe", t("mnuqe", "&Quick Exit")),
            ];
            let caps = [
                ("Label2", format!("{}:", t("label2", "Phone Number"))),
                ("Label10", t("label10", "Working File:")),
                ("Command6", t("command6", "&Daily")),
                ("Command3", t("command3", "Grap&h")),
                ("Command4", t("command4", "&Cumulative Graph")),
                ("Command5", t("command5", "Coin &Box Graph")),
                ("Command1", t("command1", "Generate &Report")),
                ("Command7", t("command7", "Clear &List Box")),
                ("Command2", t("command2", "Clear Gra&ph")),
            ];
            let title = t("frmreport", "Report Generator");
            let today = Local::now().date_naive();
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            f.c("lblCurrentPhoneNo").caption = String::new();
            f.c("lblCurrentPhoneNo").back = Some(eframe::egui::Color32::from_rgb(0, 0, 128));
            f.c("lblDays").back = Some(eframe::egui::Color32::from_rgb(0, 0, 128));
            f.c("Label9").caption = String::new();
            f.c("lblHead").caption = format!("{:<15} {:<20} {:>7} {:>11} {:>9} {:>10}", "Phone Number", "Location", "Polling", "Total", "Avg/Day", "In Box");
            f.ci("edtDateRange", 0).set_masked(&(today - chrono::Duration::days(30)).format("%m%d%y").to_string());
            f.ci("edtDateRange", 1).set_masked(&today.format("%m%d%y").to_string());
            days_label(app);
        }
        "frmCDRGenerator" => {
            let plw = app.dir("PLW").display().to_string();
            let today = Local::now().date_naive();
            let f = app.form(name);
            f.menus = vec![MenuItem::new("mnuqe", "&Quick Exit")];
            let pl = f.c("PhoneList");
            pl.path = plw;
            pl.pattern = "*.plw".into();
            f.c("meFromDate").set_masked(&today.with_day(1).unwrap_or(today).format("%m%d%y").to_string());
            f.c("meToDate").set_masked(&today.format("%m%d%y").to_string());
            f.c("optAllAmounts").value = 1;
            f.c("meSeconds").set_masked("30");
            f.c("meCents").set_masked("24");
        }
        "frmCDRViewer" | "frmCDRWorking" | "frmProcessingReports" | "frmMDA" | "frmUpdateSMDRCDF" => {}
        _ => {}
    }
}

fn date_of(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s.trim(), "%m/%d/%y").ok()
}

fn days_label(app: &mut App) {
    let f = app.form("frmReport");
    let (a, b) = (date_of(&f.texti("edtDateRange", 0)), date_of(&f.texti("edtDateRange", 1)));
    let n = match (a, b) {
        (Some(a), Some(b)) => (b - a).num_days(),
        _ => 0,
    };
    f.c("lblDays").caption = n.to_string();
}

fn generate(app: &mut App) {
    let Some(pl) = app.s.report.phone_list.clone() else {
        let m = app.tor(S, "selectphonelist", "Please select a phone list before generating any report.");
        return app.msg("", "", &m, 48, "Report");
    };
    let f = app.form("frmReport");
    let (Some(from), Some(to)) = (date_of(&f.texti("edtDateRange", 0)), date_of(&f.texti("edtDateRange", 1))) else {
        return app.msg("", "", "Invalid date.", 48, "Report Date Range");
    };
    if to <= from {
        let m = app.tor(S, "oneday", "Please check the date range.  It should be at least one day.");
        return app.msg("", "", &m, 48, "Report");
    }
    let phones = files::read_plw(&pl).unwrap_or_default();
    if phones.is_empty() {
        let m = app.tor(S, "emptyphonelist", "The phone list you have chosen is empty.");
        return app.msg("", "", &m, 48, "Report");
    }
    let mut logs: Vec<(NaiveDate, Vec<files::CollectionRec>)> = Vec::new();
    let mut d = from;
    let mut missing = Vec::new();
    while d <= to {
        let p = app.dir("CFW").join(format!("{}.cfw", d.format("%m-%d-%y")));
        match files::read_cfw(&p) {
            Ok(v) if !v.is_empty() => logs.push((d, v)),
            _ => missing.push(d),
        }
        d += chrono::Duration::days(1);
    }
    let never = app.tor(S, "neverpolled", "This Phone was Never Polled during the period.");
    let mut rows = Vec::new();
    let mut lines = vec![format!("{} {} {} {}", app.tor(S, "reportfrom", "Report for the period from: "), from.format("%m-%d-%Y"), app.tor(S, "rto", " to "), to.format("%m-%d-%Y")), String::new()];
    let (mut grand, mut grand_box) = (0.0f32, 0.0f32);
    for p in &phones {
        let mut daily = Vec::new();
        let mut prev: Option<f32> = None;
        let mut last_box = 0.0;
        let mut polls = 0;
        for (d, v) in &logs {
            if let Some(r) = v.iter().find(|r| r.phone == p.phone) {
                polls += 1;
                let cum = r.cumulative as f32 / 100.0;
                if let Some(pc) = prev {
                    daily.push((*d, (cum - pc).max(0.0)));
                }
                prev = Some(cum);
                last_box = r.coin_box as f32 / 100.0;
            }
        }
        let loc = format!("{} {}", p.address, p.city);
        if polls == 0 {
            lines.push(format!("{:<15} {never}", p.phone));
        } else {
            let total: f32 = daily.iter().map(|x| x.1).sum();
            let days = (to - from).num_days().max(1) as f32;
            lines.push(format!("{:<15} {:<20.20} {:>7} {:>11.2} {:>9.2} {:>10.2}", p.phone, loc, polls, total, total / days, last_box));
            grand += total;
            grand_box += last_box;
        }
        rows.push((p.phone.clone(), loc, daily, last_box));
    }
    lines.push(String::new());
    lines.push(format!("{:<36} {:>7} {:>11.2} {:>9} {:>10.2}", app.tor(S, "grGrandTotals", "GRAND TOTALS"), "", grand, "", grand_box));
    if !missing.is_empty() {
        lines.push(String::new());
        for k in ["info1", "info2", "info3", "info4"] {
            lines.push(app.tor(S, k, ""));
        }
        lines.extend(missing.iter().map(|d| format!("               {}", d.format("%m-%d-%y"))));
    }
    lines.push(String::new());
    lines.push(app.tor(S, "grEndOfReport", "End Of Report"));
    app.s.report.rows = rows;
    let first = phones.first().map(|p| p.phone.clone()).unwrap_or_default();
    let f = app.form("frmReport");
    f.c("List1").list = lines;
    f.c("List1").list_index = 0;
    f.c("lblCurrentPhoneNo").caption = first;
}

fn graph(app: &mut App, cumulative: bool) {
    let sel = app.form("frmReport").get("List1").list_index.max(0) as usize;
    let phone = app.form("frmReport").get("List1").list.get(sel).map(|l| l.chars().take(15).collect::<String>().trim().to_string()).unwrap_or_default();
    let row = app.s.report.rows.iter().find(|r| r.0 == phone).or(app.s.report.rows.first()).cloned();
    let Some((ph, _, daily, last_box)) = row else {
        let m = app.tor(S, "producereportfirst", "Please produce a report first.");
        return app.msg("", "", &m, 48, "Report");
    };
    let f = app.form("frmReport");
    let g = f.c("Graph1");
    g.visible = true;
    if cumulative {
        let mut acc = 0.0;
        g.data = daily.iter().map(|x| {
            acc += x.1;
            acc
        }).collect();
        g.caption = format!("Cumulative Amount, {ph}");
        g.graph_type = 1;
    } else {
        g.data = vec![last_box];
        g.caption = format!("Coin Box Amount, {ph}");
        g.graph_type = 0;
    }
    g.labels = daily.iter().map(|x| x.0.format("%m/%d").to_string()).collect();
    f.c("lblCurrentPhoneNo").caption = ph;
}

pub fn event(app: &mut App, name: &str, ev: Ev) {
    match name {
        "frmReport" => match ev {
            Ev::Close => app.unload(name),
            Ev::Change(..) | Ev::LostFocus(..) => days_label(app),
            Ev::SpinUp(c, _) | Ev::SpinDown(c, _) => {
                let _ = c;
            }
            Ev::Click(c, _) => match c.as_str() {
                "Command1" => generate(app),
                "Command4" => graph(app, true),
                "Command5" => graph(app, false),
                "Command7" => {
                    app.form(name).c("List1").clear();
                    app.s.report.rows.clear();
                }
                "Command2" => {
                    let g = app.form(name).c("Graph1");
                    g.visible = false;
                    g.data.clear();
                }
                "List1" => {
                    let f = app.form(name);
                    if let Some(l) = f.get("List1").selected().cloned() {
                        let ph: String = l.chars().take(15).collect();
                        if ph.chars().any(|c| c.is_ascii_digit()) {
                            f.c("lblCurrentPhoneNo").caption = ph.trim().to_string();
                        }
                    }
                }
                _ => {}
            },
            Ev::Menu(m) => match m.as_str() {
                "menfileopen" => {
                    let t = app.tor(S, "openphonelist", "Open Phone List");
                    if let Some(p) = app.file_dialog(false, &t, "Phone List (*.PLW)", "plw", "PLW", "") {
                        app.form(name).c("Label9").caption = p.display().to_string();
                        app.s.report.phone_list = Some(p);
                    }
                }
                "mnufilesaveas" => {
                    let lines = app.form(name).get("List1").list.clone();
                    if lines.is_empty() {
                        let m2 = app.tor(S, "producebeforesave", "Please generate a report before saving.");
                        return app.msg("", "", &m2, 48, "Report");
                    }
                    let t = app.tor(S, "savereport", "Save Report");
                    if let Some(p) = app.file_dialog(true, &t, "Report (*.RPT)", "rpt", "RPT", "") {
                        let _ = files::write_records(&p, &lines, 93);
                    }
                }
                "mnuviewreport" => {
                    let t = app.tor(S, "openreport", "Open a Report");
                    if let Some(p) = app.file_dialog(false, &t, "Report (*.RPT)", "rpt", "RPT", "") {
                        match files::read_records(&p, 93) {
                            Ok(l) => {
                                let f = app.form(name);
                                f.c("List1").list = l.into_iter().map(|s| s.trim_end().to_string()).collect();
                                f.c("Label9").caption = p.display().to_string();
                            }
                            Err(_) => {
                                let m2 = app.tor(S, "norpt", "This is not a .RPT file.");
                                app.say(&m2, "Report");
                            }
                        }
                    }
                }
                "menfilegraph" | "menoptcumm" => graph(app, true),
                "menoptcoin" => graph(app, false),
                "menoptdaily" => {
                    graph(app, true);
                    let sel = app.form(name).get("List1").list_index.max(0) as usize;
                    let _ = sel;
                    if let Some((_, _, daily, _)) = app.s.report.rows.first().cloned() {
                        let g = app.form(name).c("Graph1");
                        g.data = daily.iter().map(|x| x.1).collect();
                        g.graph_type = 0;
                        g.caption = "Daily Amount".into();
                    }
                }
                "mengraph2dbar" | "mengraph3dbar" | "mengraph2dpie" | "mengraph3dpie" => app.form(name).c("Graph1").graph_type = 0,
                "mengraphline" => app.form(name).c("Graph1").graph_type = 1,
                "menfileprint" => {
                    let lines = app.form(name).get("List1").list.clone();
                    if lines.is_empty() {
                        let m2 = app.tor(S, "grNoReports", "There are no reports to print.");
                        return app.say(&m2, "Report");
                    }
                    app.print("Report", &lines);
                }
                "menfileexport" => {
                    if app.s.report.rows.is_empty() {
                        let m2 = app.tor(S, "grGen", "Please generate a report before exporting.");
                        return app.say(&m2, "Report");
                    }
                    if let Some(p) = app.file_dialog(true, "Export Report", "Text (*.TXT)", "txt", "RPT", "") {
                        let body: Vec<String> = app.s.report.rows.iter().map(|r| format!("\"{}\",\"{}\",{:.2},{:.2}", r.0, r.1, r.2.iter().map(|x| x.1).sum::<f32>(), r.3)).collect();
                        let _ = std::fs::write(p, body.join("\r\n"));
                    }
                }
                "menfileexit" => app.unload(name),
                _ => {}
            },
            _ => {}
        },
        "frmCDRGenerator" => match ev {
            Ev::Close => app.unload(name),
            Ev::Click(c, _) => match c.as_str() {
                "cmdSelectAll" => (0..19).for_each(|i| {
                    if app.form(name).has("Check1", i) {
                        app.form(name).ci("Check1", i).value = 1;
                    }
                }),
                "Command1" => (0..19).for_each(|i| {
                    if app.form(name).has("Check1", i) {
                        app.form(name).ci("Check1", i).value = 0;
                    }
                }),
                "cmdSelectDialAround" => {
                    app.form(name).c("fraDialAround").enabled = true;
                    for i in [5, 6, 13, 14] {
                        app.form(name).ci("Check1", i).value = 1;
                    }
                }
                "cmdClear" => {
                    let f = app.form(name);
                    for i in 0..19 {
                        if f.has("Check1", i) {
                            f.ci("Check1", i).value = 0;
                        }
                    }
                    f.c("optAllAmounts").value = 1;
                    f.c("optLessThan").value = 0;
                    f.c("optGreaterThan").value = 0;
                }
                "cmdCreateReport" | "cmdViewReport" => cdr_report(app),
                _ => {}
            },
            _ => {}
        },
        "frmCDRViewer" => {
            if ev == Ev::Close {
                app.unload(name);
            }
        }
        "frmMDA" => match ev {
            Ev::Click(c, _) if c == "Command1" => {
                let v = app.form(name).text("Text1");
                app.prof.set("BatchAutoPrint", "MDA", v.trim());
                app.g.ap_mda = v;
                app.unload(name);
            }
            Ev::Close => app.unload(name),
            _ => {}
        },
        _ => {
            if ev == Ev::Close {
                app.unload(name);
            }
        }
    }
}

/// Create Report: the calls of the chosen phone list's phones in the SMD logs of the date range.
fn cdr_report(app: &mut App) {
    let f = app.form("frmCDRGenerator");
    let list = f.get("PhoneList").selected().cloned();
    let (from, to) = (date_of(&f.text("meFromDate")), date_of(&f.text("meToDate")));
    let types: Vec<String> = (0..19).filter(|&i| f.has("Check1", i) && f.valuei("Check1", i) == 1).map(|i| crate::vb::plain(&f.geti("Check1", i).caption).trim().to_ascii_lowercase()).collect();
    let amount: f32 = f.get("meAmount").clip_text().parse::<f32>().unwrap_or(0.0) / 100.0;
    let (less, greater) = (f.value("optLessThan") == 1, f.value("optGreaterThan") == 1);
    let Some(list) = list else {
        return app.say("Please choose a phone list.", "Call Detail Report Generator");
    };
    let (Some(from), Some(to)) = (from, to) else {
        return app.say("Please check the date range.", "Call Detail Report Generator");
    };
    let phones = files::read_plw(&app.dir("PLW").join(&list)).unwrap_or_default();
    let keys: Vec<String> = phones.iter().map(|p| p.digits()).collect();
    let mut lines = Vec::new();
    let mut counts: std::collections::BTreeMap<String, usize> = Default::default();
    let mut d = from;
    while d <= to {
        let p = app.dir("SMD").join(format!("{}.SMD", d.format("%m-%d-%y")));
        if let Ok(b) = std::fs::read(&p) {
            for (ph, c) in proto::smdr_file_rows(&b, d.year()) {
                let digits: String = ph.chars().filter(|c| c.is_ascii_digit()).collect();
                if !keys.iter().any(|k| k.ends_with(&digits) || digits.ends_with(k.as_str())) {
                    continue;
                }
                let kind = c.call_type.to_ascii_lowercase().replace('_', " ");
                if !types.is_empty() && !types.iter().any(|t| kind.contains(t.as_str()) || t.contains(kind.as_str())) {
                    continue;
                }
                let amt: f32 = c.amount.trim_start_matches('$').parse().unwrap_or(0.0);
                if (less && amt >= amount) || (greater && amt <= amount) {
                    continue;
                }
                *counts.entry(c.call_type.clone()).or_default() += 1;
                let mark = if amt == 0.0 { "*" } else { "" };
                lines.push(format!("{:<19} {:<8} {:<5} {:<19} {:>5} {:>8} {mark}", ph, c.date, c.time, c.number, c.length, c.amount));
            }
        }
        d += chrono::Duration::days(1);
    }
    app.show("frmCDRViewer", false);
    let v = app.form("frmCDRViewer");
    v.c("lblFileName").caption = list;
    v.c("lblDateRange").caption = format!("{} - {}", from.format("%m/%d/%Y"), to.format("%m/%d/%Y"));
    v.c("lblCallAmounts").caption = if less { format!("Less than ${amount:.2}") } else if greater { format!("Greater than ${amount:.2}") } else { "All Amounts".into() };
    v.c("lblDialAround").caption = String::new();
    v.c("lstCallDetail").list = lines;
    v.c("lstCallTypes").list = counts.iter().map(|(k, n)| format!("{k:<10.10}{n:>4}")).collect();
}

pub fn answer(_app: &mut App, _owner: &str, _tag: &str, _a: Answer) {}

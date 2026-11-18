//! FEATURES.FRM + GETPF.FRM, AOSSCRPT.FRM, T_CSN.FRM / T_CSNI.FRM / T_UPT.FRM + CSNENTRY.FRM,
//! T_PHONE.FRM + U_PHONE.FRM.

use crate::app::App;
use crate::fea::{FeatureFile, Kind as FeaKind};
use crate::files::{self, Phone};
use crate::modem::{Op, Res};
use crate::proto::{self, Map};
use crate::vb::{Answer, Ev, MenuItem};
use eframe::egui::Color32;

#[derive(Default)]
pub struct FeaState {
    pub file: FeatureFile,
    pub page: usize,
    /// The option being edited in frmGetProgrammableFeatures.
    pub editing: Option<usize>,
}

#[derive(Default)]
pub struct AosState {
    /// Per script 1..3: instruction texts, codes, phone, auth, working file.
    pub scripts: [AosScript; 4],
    pub current: usize,
    /// The instruction chosen to add (Label3) and its code.
    pub pick: Option<(String, u8)>,
}
#[derive(Default, Clone)]
pub struct AosScript {
    pub steps: Vec<(String, u8)>,
    pub phone: String,
    pub auth: String,
    pub file: String,
}

pub type CsnRow = (String, String, String, String, String, String);
#[derive(Default)]
pub struct CsnState {
    pub rows: std::collections::HashMap<String, Vec<CsnRow>>,
    /// Which table frmCSNEntry edits, and which row (None = a new one).
    pub entry_for: String,
    pub entry_row: Option<usize>,
}

#[derive(Default)]
pub struct PhoneState {
    pub list: Vec<Phone>,
    pub edit_row: Option<usize>,
    pub show_unsuccessful: bool,
}

fn qe(app: &App, sec: &str) -> MenuItem {
    MenuItem::new("mnuqe", app.tor(sec, "mnuqe", "&Quick Exit"))
}

fn file_menu(app: &App, sec: &str, extra: &[(&str, &str, &str)]) -> MenuItem {
    let t = |k: &str, d: &str| app.tor(sec, k, d);
    let mut items = vec![
        MenuItem::new("menfileopen", t("menfileopen", "&Open")),
        MenuItem::new("menfilesave", t("menfilesave", "&Save")),
        MenuItem::new("menfilesaveas", t("menfilesaveas", "Save &As")),
    ];
    for (n, k, d) in extra {
        items.push(MenuItem::new(n, t(k, d)));
    }
    items.push(MenuItem::new("menfileprint", t("menfileprint", "&Print")));
    items.push(MenuItem::new("menfileexport", t("menfileexport", "&Export to comma delimitted file")));
    items.push(MenuItem::sep());
    items.push(MenuItem::new("menfileexit", t("menfileexit", "E&xit")));
    MenuItem::sub("menfile", t("menfile", "&File"), items)
}

pub fn load(app: &mut App, name: &str) {
    match name {
        "frmProgrammableFeatures" => {
            let s = "PROGRAMMABLE FEATURES";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let m = vec![
                file_menu(app, s, &[("menfilesavetag", "menfilesavetag", "Save &Tagged as")]),
                MenuItem::sub(
                    "mentransfer",
                    t("mentransfer", "&Transfer"),
                    vec![MenuItem::new("mentransdown", t("mentransdown", "&Download")), MenuItem::new("mentransup", t("mentransup", "&Upload")), MenuItem::new("mentranspdown", t("mentranspdown", "&Tagged Download"))],
                ),
                qe(app, s),
            ];
            let caps = [
                ("cmdToggleTagAll", t("cmdtoggletagall", "Toggle &All Tags")),
                ("cmdTagAll", t("cmdtagall", "Tag All")),
                ("cmdUntagAll", t("cmduntagall", "Untag All")),
                ("cmdClear", t("cmdclear", "C&lear")),
                ("cmdPrevious", t("cmdprevious", "&Previous")),
                ("cmdNext", t("cmdnext", "&Next")),
                ("lblChange", t("lblchange", "Don't forget to save your changes!")),
            ];
            let title = t("frmprogrammablefeatures", "Programmable Features");
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            app.s.fea = FeaState::default();
            // Preview aid (with --screenshot): open a features file and tag its first options.
            if let Ok(p) = std::env::var("TFW_PREVIEW_FEA") {
                if let Ok(ff) = std::fs::read(&p).map_err(|e| e.to_string()).and_then(|b| FeatureFile::parse(&b)) {
                    app.s.fea.file = ff;
                    for x in app.s.fea.file.features.iter_mut().take(3).skip(1) {
                        x.tagged = true;
                    }
                }
            }
            fea_page(app);
        }
        "frmGetProgrammableFeatures" => {}
        "frmAOSScript" => {
            let m = vec![
                MenuItem::sub(
                    "menfile",
                    "&File",
                    vec![
                        MenuItem::new("menfileopen", "&Open"),
                        MenuItem::new("menfilesave", "&Save"),
                        MenuItem::new("menfilesaveas", "Save &As"),
                        MenuItem::new("menfileprint", "&Print"),
                        MenuItem::sep(),
                        MenuItem::new("menfileexit", "E&xit"),
                    ],
                ),
                aos_menu(),
                MenuItem::sub("mentransfer", "&Transfer", vec![MenuItem::new("mentransdown", "&Download")]),
                MenuItem::new("mnuqe", app.tor("TELELINK", "vQuickExit", "&Quick Exit")),
            ];
            let f = app.form(name);
            f.menus = m;
            for k in 1..=3 {
                f.ci("List1", k).visible = k == 1;
            }
            f.c("Label3").caption = String::new();
            app.s.aos = AosState { current: 1, ..Default::default() };
            aos_show(app);
        }
        "frmCSNTable" | "frmCSNiTable" | "frmUPTTable" => {
            let s = "CSN TABLE";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let m = vec![file_menu(app, s, &[]), super::transfer_menu(app, s, vec![]), qe(app, s)];
            let caps = [
                ("cmdAdd", t("cmdadd", "&Add")),
                ("cmdEdit", t("cmdedit", "&Edit")),
                ("cmdDelete", t("cmddelete", "&Delete")),
                ("cmdClear", t("cmdclear", "&Clear")),
                ("Label1", t("label1", "Phone Number")),
                ("Label2", t("label2", "Rate Type")),
                ("Label4", t("label4", "1st $/Time")),
                ("Label7", t("label7", "2nd $/Time")),
                ("lblCurrFile", t("lblcurrfile", "Working File:")),
                ("lblChange", t("lblchange", "Don't forget to save your changes!")),
            ];
            let f = app.form(name);
            f.menus = m;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            f.set_caption("lblWorkingFile", -1, "");
            if f.has("frmExportCSNInfo", -1) {
                f.c("frmExportCSNInfo").visible = false;
            }
            app.s.csn.rows.insert(name.to_string(), Vec::new());
            csn_show(app, name);
        }
        "frmCSNEntry" => {
            let s = "CSN ENTRY";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let caps = [
                ("lblPhoneNumber", t("lblphonenumber", "Phone Number")),
                ("lblRateType", t("lblratetype", "Rate Type")),
                ("lblInitialAmount", t("lblinitialamount", "Initial Amount")),
                ("lblInitialPeriod", t("lblinitialperiod", "Initial Period")),
                ("lblSecondAmount", t("lblsecondamount", "Second Amount")),
                ("lblSecondPeriod", t("lblsecondperiod", "Second Period")),
                ("cmdCancel", t("cmdcancel", "&Cancel")),
                ("cmdOk", t("cmdok", "&OK")),
            ];
            let f = app.form(name);
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            for (r, _) in proto::CSN_RATES {
                f.c("cboRateType").add_item(r);
            }
            f.c("cboRateType").list_index = 0;
            f.c("cboRateType").text = "Default".into();
        }
        "frmPhoneList" => {
            let s = "PHONE LIST";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let mut fm = file_menu(app, s, &[]);
            if let Some(p) = fm.children.iter_mut().find(|c| c.name == "menfileprint") {
                p.children = vec![MenuItem::new("menfileprintsingle", t("menfileprintsingle", "&Single Spaced")), MenuItem::new("menfileprintdouble", t("menfileprintdouble", "&Double Spaced"))];
            }
            let m = vec![fm, qe(app, s)];
            let as_log = app.g.phone_list_as_log;
            let caps = [
                ("lblCurrFile", t("lblcurrfile", "Working File:")),
                ("Label3", format!("{}:", t("label3", "Phone Number"))),
                ("Label13", format!("{}:", t("label13", "ID Number"))),
                ("lblChange", t("lblchange", "<-- Don't forget to save your changes!")),
                ("optShowU", t("optshowu", "Show unsuccessful polls only")),
                ("optShowAll", t("optshowall", "Show all")),
                ("cmdAdd", t("cmdadd", "&Add")),
                ("cmdEdit", t("cmdedit", "&Edit")),
                ("cmdDelete", t("cmddelete", "&Delete")),
                ("cmdClear", t("cmdclear", "&Clear")),
                ("cmdRunBatch", t("cmdrunbatch", "&Re-Run Batch")),
                ("cmdTag", t("cmdtag", "&Tag for Re-Run")),
                ("Command1", t("SortButton", "&Sort")),
                ("Command3", t("Searchbutton", "Search")),
                ("Command4", t("Searchbutton", "Search")),
            ];
            let title = if as_log { t("batchlog", "Batch Log") } else { t("frmPhoneList", "Phone List") };
            let cols: Vec<(String, usize)> = vec![
                (t("phonenumber", "Phone Number"), 14),
                (t("phonetypex", "Phone Type"), 6),
                (t("id", "ID"), 7),
                (t("firmwareversion", "Firmware Version"), 8),
                (t("status", "Status"), 18),
                (t("address", "Address"), 25),
                (t("city", "City"), 15),
                (t("state", "State"), 5),
                (t("zip", "Zip"), 6),
                ("Code".into(), 5),
                (t("plPIC", "PIC"), 9),
            ];
            let f = app.form(name);
            f.menus = m;
            f.caption = title;
            for (n, c) in caps {
                f.set_caption(n, -1, &c);
            }
            f.set_caption("LblWorkingFile", -1, "");
            let g = f.c("Grid1");
            g.cells = vec![cols.iter().map(|(c, _)| c.clone()).collect()];
            g.col_widths = cols.iter().map(|(_, n)| 120.0 * *n as f32).collect();
            g.font = "MS Sans Serif".into();
            g.bold = false;
            f.c("optShowAll").value = 1;
            for b in ["optShowU", "optShowAll", "cmdRunBatch", "cmdTag"] {
                f.c(b).visible = as_log;
            }
            f.c("cmdEdit").enabled = false;
            f.c("cmdDelete").enabled = false;
            app.s.phones = PhoneState::default();
            if as_log {
                // The batch log: the last batch's phone list with each phone's status.
                if let Some(p) = app.s.batch.last_list.clone() {
                    if let Ok(l) = files::read_plw(&p) {
                        app.s.phones.list = l;
                        app.doc("frmPhoneList").file = Some(p);
                    }
                }
            }
            phones_show(app);
        }
        "frmUpdatePhoneListEntry" => {
            let s = "PHONE LIST ENTRY";
            let t = |k: &str, d: &str| app.tor(s, k, d);
            let title = t("plMainTitle", "Phone List Entry");
            let caps = [(0, t("plANI", "Payphone ANI:")), (1, t("plTypeID", "Payphone Type & ID:").replace(" & ", " && ")), (2, t("plPIC", "Payphone PIC:")), (3, t("plAddress", "Location Address:"))];
            let (ok, cancel) = (t("plOK", "OK"), t("plCancel", "Cancel"));
            let f = app.form(name);
            f.caption = title;
            for (i, c) in caps {
                f.set_caption("lblPhoneListEntry", i, &c);
            }
            f.c("Command1").caption = format!("&{ok}");
            f.c("Command2").caption = format!("&{cancel}");
        }
        _ => {}
    }
}

// ------------------------------------------------------------------- Programmable features

fn fea_page(app: &mut App) {
    let opt = app.tor("PROGRAMMABLE FEATURES", "optionx", "Option");
    let none = app.tor("PROGRAMMABLE FEATURES", "noprogrammablefeaturesavailable", "No Programmable Features Available");
    let page = app.s.fea.page;
    let feats = app.s.fea.file.features.clone();
    let n = feats.len();
    let f = app.form("frmProgrammableFeatures");
    for i in 0..12 {
        let k = page * 12 + i;
        let fr = f.ci("Frame3D1", i as i32);
        match feats.get(k) {
            Some(x) => {
                // sub16F8: a tagged option's frame takes the tag colour (the colour value itself
                // was lost in decompiling; a light yellow is used).
                fr.visible = true;
                fr.text = format!("{opt} {}", x.number());
                fr.back = if x.tagged { Some(TAG_COLOR) } else { None };
                f.ci("Label1", i as i32).caption = x.name();
                f.ci("Label2", i as i32).caption = x.display();
            }
            None => {
                fr.visible = n == 0 && i == 0;
                fr.text = String::new();
                fr.back = None;
                f.ci("Label1", i as i32).caption = if n == 0 && i == 0 { none.clone() } else { String::new() };
                f.ci("Label2", i as i32).caption = String::new();
            }
        }
    }
    f.c("cmdPrevious").enabled = page > 0;
    f.c("cmdNext").enabled = (page + 1) * 12 < n;
    fea_menus(app);
}

const TAG_COLOR: Color32 = Color32::from_rgb(255, 255, 160);

/// Form_Load leaves Save, Save As, Save Tagged, Print, Export and the downloads off until there
/// are features; sub1781 (the Transfer menu) offers Download when nothing is tagged and Tagged
/// Download when something is.
fn fea_menus(app: &mut App) {
    let have = !app.s.fea.file.features.is_empty();
    let tagged = app.s.fea.file.features.iter().any(|x| x.tagged);
    let f = app.form("frmProgrammableFeatures");
    for m in ["menfilesave", "menfilesaveas", "menfileprint", "menfileexport"] {
        f.menu_enable(m, have);
    }
    f.menu_enable("menfilesavetag", have && tagged);
    f.menu_enable("mentransdown", have && !tagged);
    f.menu_enable("mentranspdown", have && tagged);
}

/// sub16E6: a click on an option's box tags it (or untags it); option 99, the firmware
/// version, can't be tagged.
fn fea_toggle(app: &mut App, i: usize) {
    if app.s.fea.file.features.is_empty() {
        let m = app.tor("PROGRAMMABLE FEATURES", "pfNoData", "No data is available.");
        app.msg("", "", &m, 48, "Programmable Features");
        return;
    }
    let k = app.s.fea.page * 12 + i;
    if let Some(x) = app.s.fea.file.features.get_mut(k) {
        if x.address() != crate::fea::FIRMWARE_ADDR && !x.number().starts_with("99") {
            x.tagged = !x.tagged;
        }
    }
}

fn fea_open(app: &mut App) {
    let Some(p) = app.file_dialog(false, "Open Programmable Features", "Features (*.FEA)", "fea", "FEA", "") else { return };
    match std::fs::read(&p).map_err(|e| e.to_string()).and_then(|b| FeatureFile::parse(&b)) {
        Ok(ff) => {
            app.s.fea = FeaState { file: ff, page: 0, editing: None };
            let wf = app.tor("PROGRAMMABLE FEATURES", "workingfile", "Working File: ");
            app.form("frmProgrammableFeatures").c("Label3").caption = format!("{wf}{}", p.display());
            app.doc("frmProgrammableFeatures").file = Some(p);
            app.changed("frmProgrammableFeatures", false);
            fea_page(app);
        }
        Err(e) => app.say(&e, "Programmable Features"),
    }
}

fn fea_save(app: &mut App, ask: bool, tagged: bool) {
    let p = match (ask || tagged, app.doc("frmProgrammableFeatures").file.clone()) {
        (false, Some(p)) => p,
        _ => match app.file_dialog(true, "Save Programmable Features", "Features (*.FEA)", "fea", "FEA", "") {
            Some(p) => p,
            None => return,
        },
    };
    if !p.extension().is_some_and(|e| e.eq_ignore_ascii_case("fea")) {
        let m = app.tor("PROGRAMMABLE FEATURES", "usefea", "You must use the .FEA extention to save this file.");
        app.say(&m, "Programmable Features");
        return;
    }
    match std::fs::write(&p, app.s.fea.file.to_bytes(tagged)) {
        Ok(()) => {
            if !tagged {
                let wf = app.tor("PROGRAMMABLE FEATURES", "workingfile", "Working File: ");
                app.form("frmProgrammableFeatures").c("Label3").caption = format!("{wf}{}", p.display());
                app.doc("frmProgrammableFeatures").file = Some(p);
                app.changed("frmProgrammableFeatures", false);
            }
        }
        Err(e) => app.say(&format!("Unknown Error: {e}"), "Programmable Features"),
    }
}

fn fea_edit(app: &mut App, i: usize) {
    let k = app.s.fea.page * 12 + i;
    let Some(x) = app.s.fea.file.features.get(k).cloned() else { return };
    if x.read_only() {
        return;
    }
    app.s.fea.editing = Some(k);
    let (vt, cancel, ok) = (app.tor("TELELINK", "gvValue", "Value"), app.tor("TELELINK", "gvCancel", "Cancel"), app.tor("TELELINK", "gvOK", "OK"));
    let f = app.form("frmGetProgrammableFeatures");
    f.caption = format!("{} {}", x.number(), x.name());
    f.c("Label1").caption = x.help();
    f.c("Label2").caption = vt.clone();
    f.c("Frame1").caption = vt;
    f.c("Command1").caption = format!("&{ok}");
    f.c("Command2").caption = format!("&{cancel}");
    f.c("Label3").visible = false;
    f.c("MaskedEdit2").visible = false;
    match x.kind() {
        FeaKind::Choice(a, b) => {
            f.c("Frame1").visible = true;
            f.c("MaskedEdit1").visible = false;
            f.c("Label2").visible = false;
            for j in 0..4 {
                let o = f.ci("Option1", j);
                o.visible = j < 2;
                o.caption = if j == 0 { a.to_string() } else { b.to_string() };
                o.value = (j == x.value_bytes()[0] as i32) as i32;
            }
        }
        _ => {
            f.c("Frame1").visible = false;
            f.c("MaskedEdit1").visible = true;
            f.c("Label2").visible = true;
            let me = f.c("MaskedEdit1");
            me.mask = String::new();
            me.max_len = x.edit_width();
            me.text = x.edit_text();
        }
    }
    app.show("frmGetProgrammableFeatures", true);
}

fn fea_apply_edit(app: &mut App) {
    let Some(k) = app.s.fea.editing.take() else { return };
    let f = app.form("frmGetProgrammableFeatures");
    let choice = f.get("Frame1").visible;
    let second = f.valuei("Option1", 1) == 1;
    let text = f.text("MaskedEdit1");
    let x = &mut app.s.fea.file.features[k];
    let r = if choice {
        x.set_choice(second);
        Ok(())
    } else {
        x.set_from_text(&text)
    };
    match r {
        Ok(()) => {
            app.unload("frmGetProgrammableFeatures");
            app.changed("frmProgrammableFeatures", true);
            fea_page(app);
        }
        Err(e) => {
            app.s.fea.editing = Some(k);
            app.say(&e, "Programmable Features");
        }
    }
}

pub fn fea_download(app: &mut App, tagged: bool) {
    let recs: Vec<(u16, Vec<u8>)> = app
        .s
        .fea
        .file
        .features
        .iter()
        .filter(|x| !x.read_only() && (!tagged || x.tagged))
        .map(|x| (x.address(), x.value_bytes().to_vec()))
        .collect();
    if recs.is_empty() {
        let m = app.t("TELELINK", "xNothingToDownload", "There is nothing to download.");
        app.say(&m, "Programmable Features");
        return;
    }
    // sub15E7 / sub1796: connect first when there's no call, then compare the phone's firmware
    // with the file's option 99 (5 characters on D3 phones, 4 or 5 on D1) before sending.
    let Some((t, phone_fw)) = app.modem.connected() else {
        app.doc("frmProgrammableFeatures").pending = None;
        app.g.after_connect = Some(("frmProgrammableFeatures".into(), if tagged { "tagged_after".into() } else { "download_after".into() }));
        app.show("frmConnect", true);
        return;
    };
    let file_fw = app.s.fea.file.firmware();
    let n = if t.d3 || phone_fw.get(1..3) == Some("10") { 5 } else { 4 };
    let head = |s: &str| s.trim().chars().take(n).collect::<String>().to_ascii_uppercase();
    if head(&phone_fw) != head(&file_fw) {
        let m = app.tor("PROGRAMMABLE FEATURES", "pfInvalidFirmwareVersion", "Invalid Firmware Version");
        let title = app.tor("PROGRAMMABLE FEATURES", "pfPF", "Programmable Features");
        app.msg("", "", &format!("{m}\n\n{phone_fw} / {file_fw}"), 48, &title);
        return;
    }
    app.transfer("frmProgrammableFeatures", "download", Op::WriteRecords(recs));
}

/// Upload: the .FEA file whose firmware (option 99) matches the phone's, its options read
/// from the phone (sub046D's pages).
pub fn fea_upload(app: &mut App) {
    let fw = app.modem.connected().map(|(_, fw)| fw);
    match fw {
        Some(fw) => {
            let Some((p, ff)) = find_fea(app, &fw) else {
                let m = app.tor("PROGRAMMABLE FEATURES", "pfInvalidFirmwareVersion", "Invalid Firmware Version");
                app.say(&format!("{m}: {fw}"), "Programmable Features");
                return;
            };
            let d3 = app.modem.connected().map(|(t, _)| t.d3).unwrap_or(true);
            let addrs: Vec<(u16, usize)> = ff.features.iter().map(|x| (x.address(), x.len())).collect();
            let pages = Map { d3 }.feature_pages(&addrs);
            app.s.fea = FeaState { file: ff, page: 0, editing: None };
            app.doc("frmProgrammableFeatures").file = None;
            let up = app.tor("PROGRAMMABLE FEATURES", "pfUploadedFrom", "Uploaded from");
            app.form("frmProgrammableFeatures").c("Label3").caption = format!("{up} {}", p.display());
            app.transfer("frmProgrammableFeatures", "upload", Op::ReadAreas(pages));
        }
        None => {
            // Connect first; the upload runs again once the firmware is known.
            app.doc("frmProgrammableFeatures").pending = None;
            app.g.after_connect = Some(("frmProgrammableFeatures".into(), "upload_after".into()));
            app.show("frmConnect", true);
            app.s.logs.fea_upload_after_connect = true;
        }
    }
}

/// sub15FD: the features file for a firmware version, by name: the first character becomes
/// `v`; on D3 phones the 4th becomes `-` and the 6th `0` (D32.13 → v32-10, E40.10 → v40-10);
/// on D1 phones a version with `.` 3rd (D8.54F) gets `-` 3rd and `0` 5th and 6th (v8-500),
/// others `-` 4th and `0` 6th. None for versions shorter than 6 characters.
pub fn fea_file_name(fw: &str, d3: bool) -> Option<String> {
    let mut c: Vec<char> = fw.trim().chars().collect();
    if c.len() < 6 {
        return None;
    }
    c.truncate(6);
    c[0] = 'v';
    if d3 || c[2] != '.' {
        c[3] = '-';
        c[5] = '0';
    } else {
        c[2] = '-';
        c[4] = '0';
        c[5] = '0';
    }
    Some(format!("{}.fea", c.iter().collect::<String>()))
}

pub fn find_fea_for(dir: &std::path::Path, fw: &str, d3: bool) -> Option<(std::path::PathBuf, FeatureFile)> {
    let names = crate::vb::list_files(&dir.display().to_string(), "*.fea");
    if let Some(want) = fea_file_name(fw, d3) {
        if let Some(n) = names.iter().find(|n| n.eq_ignore_ascii_case(&want)) {
            let p = dir.join(n);
            if let Ok(mut ff) = std::fs::read(&p).map_err(|e| e.to_string()).and_then(|b| FeatureFile::parse(&b)) {
                // The last option (99) takes the phone's own version (sub15FD).
                if let Some(x) = ff.features.iter_mut().find(|x| x.address() == crate::fea::FIRMWARE_ADDR) {
                    x.set_value_bytes(format!("{:<16}", fw.trim()).as_bytes());
                }
                return Some((p, ff));
            }
        }
    }
    None
}

fn find_fea(app: &App, fw: &str) -> Option<(std::path::PathBuf, FeatureFile)> {
    let d3 = app.modem.connected().map(|(t, _)| t.d3).unwrap_or(true);
    if let Some(x) = find_fea_for(&app.dir("FEA"), fw, d3) {
        return Some(x);
    }
    let dir = app.dir("FEA");
    let mut names = crate::vb::list_files(&dir.display().to_string(), "*.fea");
    names.sort();
    for n in names {
        let p = dir.join(&n);
        if let Ok(ff) = std::fs::read(&p).map_err(|e| e.to_string()).and_then(|b| FeatureFile::parse(&b)) {
            if ff.firmware().trim().eq_ignore_ascii_case(fw.trim()) {
                return Some((p, ff));
            }
        }
    }
    None
}

// ------------------------------------------------------------------- AOS scripts

fn aos_menu() -> MenuItem {
    use crate::docs::*;
    let mut items = Vec::new();
    for (i, name) in AOS_INSTRUCTIONS.iter().enumerate() {
        let n = i + 1;
        let kids: Vec<MenuItem> = match n {
            4 | 9 => DTMF_KEYS.iter().map(|k| MenuItem::new(&format!("aos:{n}:{k}"), *k)).collect(),
            8 => PAUSES.iter().map(|k| MenuItem::new(&format!("aos:{n}:{k}"), *k)).collect(),
            11 => CARD_PARTS[..4].iter().map(|k| MenuItem::new(&format!("aos:{n}:{k}"), *k)).collect(),
            _ => vec![],
        };
        if kids.is_empty() {
            items.push(MenuItem::new(&format!("aos:{n}:"), *name));
        } else {
            items.push(MenuItem::sub(&format!("aosm{n}"), *name, kids));
        }
    }
    MenuItem::sub("meninstructions", "&Instructions", items)
}

fn aos_show(app: &mut App) {
    let cur = app.s.aos.current;
    let scripts = app.s.aos.scripts.clone();
    let pick = app.s.aos.pick.clone();
    let f = app.form("frmAOSScript");
    for k in 1..=3 {
        let l = f.ci("List1", k as i32);
        l.visible = k == cur;
        let sel = l.list_index;
        l.list = scripts[k].steps.iter().map(|(t, _)| t.clone()).collect();
        l.list_index = sel.min(l.list.len() as i32 - 1);
        f.ci("List2", k as i32).list = scripts[k].steps.iter().map(|(_, c)| format!("{c:02X}")).collect();
    }
    f.c("MaskedEdit1").set_masked(&scripts[cur].phone);
    f.c("MaskedEdit2").set_masked(&scripts[cur].auth);
    f.c("lblCurrFile").caption = if scripts[cur].file.is_empty() { String::new() } else { format!("Working File: {}", scripts[cur].file) };
    f.c("Label3").caption = pick.as_ref().map(|p| p.0.clone()).unwrap_or_default();
    let has_pick = pick.is_some();
    let n = scripts[cur].steps.len();
    f.c("Command1").enabled = has_pick && n < 15;
    f.c("Command4").enabled = has_pick && n < 15 && f.geti("List1", cur as i32).list_index >= 0;
    f.c("Command2").enabled = f.geti("List1", cur as i32).list_index >= 0;
    f.c("Command3").enabled = n > 0;
}

fn aos_store_fields(app: &mut App) {
    let cur = app.s.aos.current;
    let f = app.form("frmAOSScript");
    let (ph, au) = (f.get("MaskedEdit1").text.clone(), f.get("MaskedEdit2").text.clone());
    app.s.aos.scripts[cur].phone = ph;
    app.s.aos.scripts[cur].auth = au;
}

fn aos_event(app: &mut App, ev: Ev) {
    let cur = app.s.aos.current;
    match ev {
        Ev::Close => close_doc(app, "frmAOSScript"),
        Ev::Change(..) => {
            aos_store_fields(app);
            app.changed("frmAOSScript", true);
        }
        Ev::Click(c, i) => {
            match c.as_str() {
                "Option1" | "Option2" | "Option3" => {
                    aos_store_fields(app);
                    app.s.aos.current = c[6..].parse().unwrap_or(1);
                }
                "Command1" | "Command4" => {
                    if app.s.aos.scripts[cur].steps.len() >= 15 {
                        app.msg("", "", "No more entries are allowed.", 0, "AOS Scripts");
                    } else if let Some(p) = app.s.aos.pick.clone() {
                        let at = app.form("frmAOSScript").geti("List1", cur as i32).list_index;
                        if c == "Command4" && at >= 0 {
                            app.s.aos.scripts[cur].steps.insert(at as usize, p);
                        } else {
                            app.s.aos.scripts[cur].steps.push(p);
                        }
                        app.changed("frmAOSScript", true);
                    }
                }
                "Command2" => {
                    let at = app.form("frmAOSScript").geti("List1", cur as i32).list_index;
                    if at >= 0 && (at as usize) < app.s.aos.scripts[cur].steps.len() {
                        app.s.aos.scripts[cur].steps.remove(at as usize);
                        app.form("frmAOSScript").ci("List1", cur as i32).list_index = -1;
                        app.changed("frmAOSScript", true);
                    }
                }
                "Command3" => {
                    app.s.aos.scripts[cur].steps.clear();
                    app.changed("frmAOSScript", true);
                }
                _ => {}
            }
            let _ = i;
            aos_show(app);
        }
        Ev::Menu(m) => {
            if let Some(rest) = m.strip_prefix("aos:") {
                let (n, param) = rest.split_once(':').unwrap_or((rest, ""));
                let base = crate::docs::AOS_INSTRUCTIONS[n.parse::<usize>().unwrap_or(1) - 1];
                let text = if param.is_empty() { base.to_string() } else { format!("{base}: {param}") };
                if let Some(code) = proto::aos_code(&text) {
                    app.s.aos.pick = Some((text, code));
                }
                aos_show(app);
                return;
            }
            match m.as_str() {
                "menfileopen" => {
                    let t = format!("Open Script # {cur}");
                    let Some(p) = app.file_dialog(false, &t, "AOS Scripts (*.AOS)", "aos", "AOS", "") else { return };
                    match files::read_aos(&p) {
                        Ok(a) => {
                            let steps = a
                                .codes
                                .iter()
                                .filter(|c| c.as_str() != "FF")
                                .filter_map(|c| u8::from_str_radix(c, 16).ok())
                                .map(|c| (proto::aos_text(c), c))
                                .collect();
                            app.s.aos.scripts[cur] = AosScript { steps, phone: a.phone, auth: a.auth, file: p.display().to_string() };
                            app.changed("frmAOSScript", false);
                        }
                        Err(_) => app.msg("", "", "This is not an AOS file.", 48, "AOS Scripts"),
                    }
                    aos_show(app);
                }
                "menfilesave" | "menfilesaveas" => {
                    aos_store_fields(app);
                    let mut path = app.s.aos.scripts[cur].file.clone();
                    if m == "menfilesaveas" || path.is_empty() {
                        let t = format!("Save Script # {cur}");
                        match app.file_dialog(true, &t, "AOS Scripts (*.AOS)", "aos", "AOS", "") {
                            Some(p) => path = p.display().to_string(),
                            None => return,
                        }
                    }
                    let s = &app.s.aos.scripts[cur];
                    let mut codes: Vec<String> = s.steps.iter().map(|(_, c)| format!("{c:02X}")).collect();
                    codes.push("FF".into());
                    let f = files::AosFile { codes, phone: s.phone.clone(), auth: s.auth.clone() };
                    match files::write_aos(std::path::Path::new(&path), &f) {
                        Ok(()) => {
                            app.s.aos.scripts[cur].file = path;
                            app.changed("frmAOSScript", false);
                        }
                        Err(e) => app.say(&format!("Unknown Error: {e}"), "AOS Scripts"),
                    }
                    aos_show(app);
                }
                "menfileprint" => {
                    let mut lines = vec!["AOS SCRIPTS".to_string(), String::new()];
                    for k in 1..=3 {
                        let s = &app.s.aos.scripts[k];
                        lines.push(format!("AOS Script #{k} : {}", s.file));
                        lines.push(String::new());
                        if s.steps.is_empty() {
                            lines.push("No script instructions.".into());
                        }
                        for (i, (t, _)) in s.steps.iter().enumerate() {
                            lines.push(format!("Step {:02} - {t}", i + 1));
                        }
                        lines.push(String::new());
                        let ph = if s.phone.chars().any(|c| c.is_ascii_digit()) { s.phone.clone() } else { "None.".into() };
                        let au = if s.auth.chars().any(|c| c.is_ascii_digit()) { s.auth.clone() } else { "None.".into() };
                        lines.push(format!("AOS {k} Phone #: {ph}"));
                        lines.push(format!("AOS {k} Auth Code: {au}"));
                        lines.push(String::new());
                    }
                    app.print("AOS Scripts", &lines);
                }
                "menfileexit" => close_doc(app, "frmAOSScript"),
                "mentransdown" => {
                    aos_store_fields(app);
                    let s = app.s.aos.scripts[cur].clone();
                    let steps: Vec<String> = s.steps.iter().map(|(t, _)| t.clone()).collect();
                    match proto::aos_bytes(&s.phone, &s.auth.replace('_', ""), &steps) {
                        Ok(b) => {
                            let q = app.t("TELELINK", "goverwrite", "This will overwrite the data currently stored in your phone.  Do you want to continue?");
                            app.s.aos.pick = app.s.aos.pick.clone();
                            app.s.logs.aos_bytes = Some((cur, b));
                            let t = app.g.title.clone();
                            app.msg("frmAOSScript", "download", &q, 36, &t);
                        }
                        Err(e) => app.say(&e, "AOS Scripts"),
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

// ------------------------------------------------------------------- CSN tables

fn csn_line(r: &CsnRow) -> String {
    format!("{:<16} {:<12} {:>6}/{:<4}  {:>6}/{:<4}", r.0, r.1, r.2, r.3, r.4, r.5)
}

fn csn_show(app: &mut App, name: &str) {
    let rows = app.s.csn.rows.get(name).cloned().unwrap_or_default();
    let f = app.form(name);
    let l = f.c("List1");
    let sel = l.list_index;
    l.list = rows.iter().map(csn_line).collect();
    l.list_index = sel.min(l.list.len() as i32 - 1);
    let has = l.list_index >= 0;
    f.c("cmdEdit").enabled = has;
    f.c("cmdDelete").enabled = has;
    f.c("cmdClear").enabled = !rows.is_empty();
}

fn csn_entries(app: &App, name: &str) -> Result<Vec<Vec<u8>>, String> {
    app.s.csn.rows.get(name).map(|v| v.iter().map(|r| proto::csn_bytes(&r.0, &r.1, &r.2, &r.3, &r.4, &r.5)).collect()).unwrap_or(Ok(vec![]))
}

fn csn_event(app: &mut App, name: &str, ev: Ev) {
    let sec = "CSN TABLE";
    match ev {
        Ev::Close => close_doc(app, name),
        Ev::Click(c, _) | Ev::DblClick(c, _) => {
            match c.as_str() {
                "cmdAdd" => {
                    if app.s.csn.rows.get(name).map(|v| v.len()).unwrap_or(0) >= 120 {
                        let (m, t) = (app.tor(sec, "csnlimit", "Only 120 CSN entries are allowed."), app.tor(sec, "csnlimitwarning", "Too Many CSN Entries!"));
                        app.msg("", "", &m, 48, &t);
                    } else {
                        app.s.csn.entry_for = name.into();
                        app.s.csn.entry_row = None;
                        app.unload("frmCSNEntry");
                        app.show("frmCSNEntry", true);
                    }
                }
                "cmdEdit" | "List1" if c == "cmdEdit" || matches!(ev_kind_dbl(&c), true) => {
                    let sel = app.form(name).get("List1").list_index;
                    if sel >= 0 {
                        app.s.csn.entry_for = name.into();
                        app.s.csn.entry_row = Some(sel as usize);
                        app.unload("frmCSNEntry");
                        app.show("frmCSNEntry", true);
                        let r = app.s.csn.rows[name][sel as usize].clone();
                        let f = app.form("frmCSNEntry");
                        f.c("mePhoneNumber").text = r.0;
                        let k = f.get("cboRateType").list.iter().position(|x| *x == r.1).unwrap_or(0);
                        f.c("cboRateType").list_index = k as i32;
                        f.c("cboRateType").text = r.1;
                        f.c("meInitialAmount").set_masked(&r.2);
                        f.c("meInitialPeriod").set_masked(&r.3);
                        f.c("meSecondAmount").set_masked(&r.4);
                        f.c("meSecondPeriod").set_masked(&r.5);
                    }
                }
                "cmdDelete" => {
                    let sel = app.form(name).get("List1").list_index;
                    if sel >= 0 {
                        app.s.csn.rows.get_mut(name).unwrap().remove(sel as usize);
                        app.form(name).c("List1").list_index = -1;
                        app.changed(name, true);
                    }
                }
                "cmdClear" => {
                    if app.doc(name).dirty {
                        let m = app.tor(sec, "savechangesmessage", "Do you wish to save your changes before clearing?");
                        app.msg(name, "save_clear", &m, 35, "Save changes?");
                    } else {
                        csn_clear(app, name);
                    }
                }
                _ => {}
            }
            csn_show(app, name);
        }
        Ev::Menu(m) => match m.as_str() {
            "menfileopen" => {
                let t = app.tor(sec, "opencsntable", "Open CSN Table");
                let Some(p) = app.file_dialog(false, &t, "CSN Table (*.CSN)", "csn", "CSN", "") else { return };
                match files::read_csn(&p) {
                    Ok(es) => {
                        let rows: Vec<CsnRow> = es.iter().filter_map(|e| proto::csn_from_bytes(e)).collect();
                        app.s.csn.rows.insert(name.into(), rows);
                        app.form(name).c("lblWorkingFile").caption = p.display().to_string();
                        app.doc(name).file = Some(p);
                        app.changed(name, false);
                    }
                    Err(_) => {
                        let m = app.tor(sec, "notcsn", "This is not a .CSN file.");
                        app.msg("", "", &m, 48, "CSN Table");
                    }
                }
                csn_show(app, name);
            }
            "menfilesave" | "menfilesaveas" => csn_save(app, name, m == "menfilesaveas"),
            "menfileprint" => {
                let rows = app.s.csn.rows.get(name).cloned().unwrap_or_default();
                let mut lines = vec![app.tor(sec, "csnCSNTable", "CSN Table"), String::new()];
                lines.push(format!("{:<16} {:<12} {:>11}  {:>11}", "Phone Number", "Rate Type", "Initial", "Secondary"));
                lines.extend(rows.iter().map(csn_line));
                app.print("CSN Table", &lines);
            }
            "menfileexport" => {
                let Some(p) = app.file_dialog(true, "Save CSN Table Export", "Text (*.TXT)", "txt", "CSN", "") else { return };
                let rows = app.s.csn.rows.get(name).cloned().unwrap_or_default();
                let body: Vec<String> = rows.iter().map(|r| format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"", r.0, r.1, r.2, r.3, r.4, r.5)).collect();
                let _ = std::fs::write(p, body.join("\r\n"));
            }
            "menfileexit" => close_doc(app, name),
            "mentransdown" => match csn_entries(app, name) {
                Ok(e) if !e.is_empty() => {
                    let q = app.t("TELELINK", "goverwrite", "This will overwrite the data currently stored in your phone.  Do you want to continue?");
                    let t = app.g.title.clone();
                    app.msg(name, "download", &q, 36, &t);
                }
                Ok(_) => {
                    let m = app.t("TELELINK", "xNothingToDownload", "There is nothing to download.");
                    app.say(&m, "CSN Table");
                }
                Err(e) => app.say(&e, "CSN Table"),
            },
            "mentransup" => app.transfer(name, "upload", Op::ReadCsn),
            _ => {}
        },
        _ => {}
    }
}

fn ev_kind_dbl(_c: &str) -> bool {
    false
}

fn csn_clear(app: &mut App, name: &str) {
    app.s.csn.rows.insert(name.into(), Vec::new());
    app.form(name).c("lblWorkingFile").caption = String::new();
    app.doc(name).file = None;
    app.changed(name, false);
    csn_show(app, name);
}

fn csn_save(app: &mut App, name: &str, ask: bool) {
    let p = match (ask, app.doc(name).file.clone()) {
        (false, Some(p)) => p,
        _ => {
            let t = app.tor("CSN TABLE", "savecsntable", "Save CSN Table");
            match app.file_dialog(true, &t, "CSN Table (*.CSN)", "csn", "CSN", "") {
                Some(p) => p,
                None => return,
            }
        }
    };
    if !p.extension().is_some_and(|e| e.eq_ignore_ascii_case("csn")) {
        let m = app.tor("CSN TABLE", "usecsn", "You must use the .CSN extention to save this file.");
        app.say(&m, "CSN Table");
        return;
    }
    match csn_entries(app, name).and_then(|e| files::write_csn(&p, &e)) {
        Ok(()) => {
            app.form(name).c("lblWorkingFile").caption = p.display().to_string();
            app.doc(name).file = Some(p);
            app.changed(name, false);
        }
        Err(e) => app.say(&e, "CSN Table"),
    }
}

fn csn_entry_ok(app: &mut App) {
    let s = "CSN ENTRY";
    let f = app.form("frmCSNEntry");
    let phone = f.text("mePhoneNumber").trim().to_string();
    let rate = f.get("cboRateType").selected().cloned().unwrap_or_else(|| "Default".into());
    let a1 = f.get("meInitialAmount").text.clone();
    let p1 = f.get("meInitialPeriod").clip_text();
    let a2 = f.get("meSecondAmount").text.clone();
    let p2 = f.get("meSecondPeriod").clip_text();
    let t = |app: &App, k: &str, d: &str| app.tor(s, k, d);
    let bad = |app: &mut App, k: &str, d: &str, kt: &str, dt: &str| {
        let (m, ti) = (t(app, k, d), t(app, kt, dt));
        app.msg("", "", &m, 48, &ti);
    };
    if phone.is_empty() {
        return bad(app, "enterphonenumber", "You must enter the phone number.", "missingphonenumber", "Missing Phone Number");
    }
    if !phone.chars().all(|c| c.is_ascii_digit() || c == '*' || c == '-') {
        return bad(app, "onlyaccepted", "Only 0, 9, * or - are allowed in the phone number.", "invalidphonenumber", "Invalid Phone Number");
    }
    let amt_ok = |a: &str| a.trim_end().ends_with('0') || a.trim_end().ends_with('5');
    if a1.contains('_') && a1.chars().all(|c| !c.is_ascii_digit()) {
        return bad(app, "enterinitialamount", "You must enter an initial amount.", "invalidinitailamount", "Invalid Initial Amount");
    }
    if !amt_ok(&a1.replace('_', "0")) {
        return bad(app, "onlyacceptedinitialamount", "Only 0 or 5 are accepted in the initial amount.", "invalidinitialamount", "Invald Initial Amount");
    }
    if p1.is_empty() {
        return bad(app, "enterinitialperiod", "You must enter an initial period.", "missinginitialperiod", "Missing Initial Period");
    }
    if a2.chars().all(|c| !c.is_ascii_digit()) {
        return bad(app, "entersecondamount", "You must enter a second amount.", "missingsecondamount", "Missing Second Amount");
    }
    if !amt_ok(&a2.replace('_', "0")) {
        return bad(app, "onlyinsecondamount", "Only 0 or 5 are accepted in the second amount.", "invalidsecondamount", "Invalid Second Amount");
    }
    if p2.is_empty() {
        return bad(app, "entersecondperiod", "You must enter a second period.", "missingsecondperiod", "Missing Second Period");
    }
    let row: CsnRow = (phone, rate, a1.replace('_', "0"), p1, a2.replace('_', "0"), p2);
    let table = app.s.csn.entry_for.clone();
    let rows = app.s.csn.rows.entry(table.clone()).or_default();
    match app.s.csn.entry_row {
        Some(i) if i < rows.len() => rows[i] = row,
        _ => rows.push(row),
    }
    app.unload("frmCSNEntry");
    app.changed(&table, true);
    csn_show(app, &table);
}

// ------------------------------------------------------------------- Phone list

fn status_text(app: &App, code: u8) -> String {
    let k = files::status_key(code);
    app.tor("PHONE LIST", k, k)
}

fn phones_show(app: &mut App) {
    let show_u = app.s.phones.show_unsuccessful;
    let rows: Vec<Vec<String>> = app
        .s
        .phones
        .list
        .iter()
        .filter(|p| !show_u || p.status != 1)
        .map(|p| {
            vec![
                p.phone.clone(),
                p.ptype.clone(),
                p.id.to_ascii_uppercase(),
                p.firmware.clone(),
                status_text(app, p.status),
                p.address.clone(),
                p.city.clone(),
                p.state.clone(),
                p.zip.clone(),
                format!("{}", p.status),
                p.pic.clone(),
            ]
        })
        .collect();
    let f = app.form("frmPhoneList");
    let g = f.c("Grid1");
    let head = g.cells.first().cloned().unwrap_or_default();
    g.cells = std::iter::once(head).chain(rows).collect();
    if g.row as usize >= g.cells.len() {
        g.row = 0;
    }
    let sel = g.row > 0;
    f.c("cmdEdit").enabled = sel;
    f.c("cmdDelete").enabled = sel;
    let n = app.s.phones.list.len();
    let f = app.form("frmPhoneList");
    f.c("cmdClear").enabled = n > 0;
    f.c("cmdRunBatch").enabled = n > 0;
}

fn phone_row(app: &mut App) -> Option<usize> {
    let r = app.form("frmPhoneList").get("Grid1").row;
    if r <= 0 {
        return None;
    }
    let ph = app.form("frmPhoneList").get("Grid1").cells.get(r as usize)?.first()?.clone();
    app.s.phones.list.iter().position(|p| p.phone == ph)
}

fn phone_entry(app: &mut App, row: Option<usize>) {
    app.s.phones.edit_row = row;
    app.unload("frmUpdatePhoneListEntry");
    app.show("frmUpdatePhoneListEntry", true);
    let p = row.and_then(|r| app.s.phones.list.get(r).cloned()).unwrap_or(Phone { ptype: "D3".into(), pic: "101-0000".into(), ..Default::default() });
    let f = app.form("frmUpdatePhoneListEntry");
    f.c("MaskedEdit1").set_masked(&p.phone);
    f.c("MaskedEdit7").set_masked(&p.ptype);
    f.c("MaskedEdit2").set_masked(&p.id);
    f.c("MaskedEdit8").set_masked(p.pic.trim_start_matches("101-"));
    f.c("Address").text = p.address;
    f.c("City").text = p.city;
    f.c("State").text = p.state;
    f.c("Zip").text = p.zip;
    f.c("Label2").caption = p.firmware;
}

fn phone_entry_ok(app: &mut App) {
    let s = "PHONE LIST ENTRY";
    let f = app.form("frmUpdatePhoneListEntry");
    let phone = f.text("MaskedEdit1");
    let mut ptype = f.text("MaskedEdit7").to_ascii_uppercase();
    let id = f.get("MaskedEdit2").clip_text();
    let pic = f.text("MaskedEdit8");
    let (addr, city, state, zip, fw) = (f.text("Address"), f.text("City"), f.text("State"), f.text("Zip"), f.get("Label2").caption.clone());
    let title = app.tor(s, "plTitle", "Phone List");
    if phone.chars().all(|c| !c.is_ascii_digit()) {
        let m = app.tor(s, "plEnterNumber", "You must enter a phone number.");
        return app.msg("", "", &m, 48, &title);
    }
    if phone.contains('_') {
        // "filling in from the left with astericks (*)": unused leading places become *.
        let m = app.tor(s, "plFillIn", "Please fill in the phone number completely - filling in from the left with astericks (*).");
        return app.msg("", "", &m, 48, &title);
    }
    if ptype.contains('_') {
        let m = app.tor(s, "plEnterType", "You must enter the phone type.");
        return app.msg("", "", &m, 48, &title);
    }
    if ptype != "D1" {
        ptype = "D3".into();
    }
    if id.len() != 6 {
        let m = app.tor(s, "plIDInvalid", "The ID you have entered is invalid.");
        return app.msg("", "", &m, 48, &title);
    }
    let row = app.s.phones.edit_row;
    let dup = app.s.phones.list.iter().enumerate().any(|(i, p)| p.phone == phone && Some(i) != row);
    if dup {
        let (m, t) = (app.tor("PHONE LIST", "duplicateentry", "There is already an entry with that phone number."), app.tor("PHONE LIST", "dataentryerror", "Data Entry Error"));
        return app.msg("", "", &m, 48, &t);
    }
    if row.is_none() && app.s.phones.list.len() >= 500 {
        let m = app.tor("PHONE LIST", "limit500", "Phone lists are limited to 500 entries.");
        return app.msg("", "", &m, 48, &title);
    }
    let status = row.and_then(|r| app.s.phones.list.get(r)).map(|p| p.status).unwrap_or(0);
    let p = Phone { phone, ptype, id, firmware: fw, address: addr, city, state, zip, status, pic };
    match row {
        Some(r) => app.s.phones.list[r] = p,
        None => app.s.phones.list.push(p),
    }
    app.unload("frmUpdatePhoneListEntry");
    app.changed("frmPhoneList", true);
    phones_show(app);
}

fn phones_open(app: &mut App) {
    let t = app.tor("PHONE LIST", "openphonelist", "Open Phone List");
    let Some(p) = app.file_dialog(false, &t, "Phone List (*.PLW)", "plw", "PLW", "") else { return };
    match files::read_plw(&p) {
        Ok(l) if !l.is_empty() || std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0) == 0 => {
            if l.is_empty() {
                let m = app.tor("PHONE LIST", "fileisempty", "The file you have selected is empty.");
                app.say(&m, "Phone List");
            }
            app.s.phones.list = l;
            app.form("frmPhoneList").c("LblWorkingFile").caption = p.display().to_string();
            app.doc("frmPhoneList").file = Some(p);
            app.changed("frmPhoneList", false);
        }
        _ => {
            let m = app.tor("PHONE LIST", "onlyplwallowed", "Only .PLW files can be saved here.");
            app.say(&m, "Phone List");
        }
    }
    phones_show(app);
}

fn phones_save(app: &mut App, ask: bool) {
    let p = match (ask, app.doc("frmPhoneList").file.clone()) {
        (false, Some(p)) => p,
        _ => {
            let t = app.tor("PHONE LIST", "savephonelist", "Save Phone List");
            match app.file_dialog(true, &t, "Phone List (*.PLW)", "plw", "PLW", "") {
                Some(p) => p,
                None => return,
            }
        }
    };
    if !p.extension().is_some_and(|e| e.eq_ignore_ascii_case("plw")) {
        let m = app.tor("PHONE LIST", "useplwtosave", "You must use the .PLW extension to save this file.");
        app.say(&m, "Phone List");
        return;
    }
    match files::write_plw(&p, &app.s.phones.list) {
        Ok(()) => {
            app.form("frmPhoneList").c("LblWorkingFile").caption = p.display().to_string();
            app.doc("frmPhoneList").file = Some(p);
            app.changed("frmPhoneList", false);
        }
        Err(e) => app.say(&format!("Unknown Error: {e}"), "Phone List"),
    }
}

fn phones_event(app: &mut App, ev: Ev) {
    let sec = "PHONE LIST";
    match ev {
        Ev::Close => close_doc(app, "frmPhoneList"),
        Ev::DblClick(c, _) if c == "Grid1" => {
            if let Some(r) = phone_row(app) {
                if app.g.lookup {
                    let p = app.s.phones.list[r].clone();
                    app.g.lookup = false;
                    app.unload("frmPhoneList");
                    super::connect::fill(app, &p.phone, &p.id, &p.ptype);
                } else {
                    phone_entry(app, Some(r));
                }
            }
        }
        Ev::Click(c, _) => match c.as_str() {
            "Grid1" => phones_show(app),
            "cmdAdd" => {
                if app.s.phones.list.len() >= 500 {
                    let m = app.tor(sec, "limit500", "Phone lists are limited to 500 entries.");
                    app.say(&m, "Phone List");
                } else {
                    phone_entry(app, None);
                }
            }
            "cmdEdit" => match phone_row(app) {
                Some(r) => phone_entry(app, Some(r)),
                None => {
                    let m = app.tor(sec, "selectanitemfirst", "Please select an item first.");
                    app.say(&m, "Phone List");
                }
            },
            "cmdDelete" => {
                if let Some(r) = phone_row(app) {
                    app.s.phones.list.remove(r);
                    app.form("frmPhoneList").c("Grid1").row = 0;
                    app.changed("frmPhoneList", true);
                    phones_show(app);
                }
            }
            "cmdClear" => {
                if app.doc("frmPhoneList").dirty {
                    let m = app.tor(sec, "savechanges", "Do you wish to save your changes before clearing?");
                    app.msg("frmPhoneList", "save_clear", &m, 35, "Save Changes?");
                } else {
                    phones_clear(app);
                }
            }
            "cmdTag" => {
                if let Some(r) = phone_row(app) {
                    app.s.phones.list[r].status = 12;
                    app.changed("frmPhoneList", true);
                    phones_show(app);
                }
            }
            "cmdRunBatch" => {
                let flagged: Vec<Phone> = app.s.phones.list.iter().filter(|p| p.status == 12).cloned().collect();
                if flagged.is_empty() {
                    let m = app.tor(sec, "plNoneFlagged", "No items flagged for re-run.");
                    app.say(&m, "Phone List");
                } else {
                    phones_save(app, false);
                    app.s.batch.rerun = Some(flagged);
                    app.show("frmBatchExec", false);
                }
            }
            "Command1" => {
                app.s.phones.list.sort_by(|a, b| a.phone.cmp(&b.phone));
                phones_show(app);
            }
            "Command3" | "Command4" => {
                let f = app.form("frmPhoneList");
                let (ph, id) = (f.text("txtPhoneNumber"), f.get("meIDBox").clip_text());
                let key = if c == "Command3" { ph.chars().filter(|c| c.is_ascii_digit()).collect::<String>() } else { id };
                if key.is_empty() {
                    let (m, t) = (app.tor(sec, "plGT", "Data to search by would be a good thing"), app.tor(sec, "plSE", "Search Error"));
                    app.say(&m, &t);
                    return;
                }
                let rows = app.form("frmPhoneList").get("Grid1").cells.clone();
                let col = if c == "Command3" { 0 } else { 2 };
                let hit = rows.iter().enumerate().skip(1).find(|(_, r)| r.get(col).is_some_and(|v| v.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().contains(&key)));
                match hit {
                    Some((i, _)) => {
                        app.form("frmPhoneList").c("Grid1").row = i as i32;
                        phones_show(app);
                    }
                    None => {
                        let (m, t) = (app.tor(sec, "plSorry", "Sorry, no match found."), app.tor(sec, "plSI", "Search Info"));
                        app.say(&m, &t);
                    }
                }
            }
            "optShowU" | "optShowAll" => {
                app.s.phones.show_unsuccessful = c == "optShowU";
                phones_show(app);
            }
            _ => {}
        },
        Ev::Menu(m) => match m.as_str() {
            "menfileopen" => phones_open(app),
            "menfilesave" => phones_save(app, false),
            "menfilesaveas" => phones_save(app, true),
            "menfileprintsingle" | "menfileprintdouble" => {
                let head = app.tor(sec, if app.g.phone_list_as_log { "plBatchPrintout" } else { "plPlPrintout" }, "PHONE LIST PRINTOUT");
                let mut lines = vec![head, String::new()];
                for p in app.s.phones.list.clone() {
                    lines.push(format!("{:<15} {:<3} {:<7} {:<7} {:<22} {} {}, {} {}", p.phone, p.ptype, p.id, p.firmware, status_text(app, p.status), p.address, p.city, p.state, p.zip));
                    if m == "menfileprintdouble" {
                        lines.push(String::new());
                    }
                }
                app.print("Phone List", &lines);
            }
            "menfileexport" => {
                let Some(p) = app.file_dialog(true, "Export", "Text (*.TXT)", "txt", "PLW", "") else { return };
                let body: Vec<String> = app
                    .s
                    .phones
                    .list
                    .iter()
                    .map(|x| format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",{}", x.phone, x.ptype, x.id, x.firmware, x.address, x.city, x.state, x.zip, x.pic, x.status))
                    .collect();
                let _ = std::fs::write(p, body.join("\r\n"));
            }
            "menfileexit" => close_doc(app, "frmPhoneList"),
            _ => {}
        },
        _ => {}
    }
}

fn phones_clear(app: &mut App) {
    app.s.phones.list.clear();
    app.form("frmPhoneList").c("LblWorkingFile").caption = String::new();
    app.doc("frmPhoneList").file = None;
    app.changed("frmPhoneList", false);
    phones_show(app);
}

// ------------------------------------------------------------------- shared

/// Form_QueryUnload (sub0F7D): AOS Scripts, Programmable Features, Phone List and the UPT
/// table ask when there are unsaved changes; the CSN tables (T_CSN, T_CSNI) close at once.
fn close_doc(app: &mut App, name: &str) {
    let asks = matches!(name, "frmAOSScript" | "frmProgrammableFeatures" | "frmPhoneList" | "frmUPTTable");
    if asks && app.doc(name).dirty {
        let title = app.g.title.clone();
        app.msg(name, "save_exit", "Do you wish to save your changes before exiting?", 35, &title);
    } else {
        app.unload(name);
    }
}

pub fn event(app: &mut App, name: &str, ev: Ev) {
    match name {
        "frmProgrammableFeatures" => match ev {
            Ev::Close => close_doc(app, name),
            Ev::Click(c, i) => {
                let n = app.s.fea.file.features.len();
                match c.as_str() {
                    "Label1" | "Label2" | "Frame3D1" => fea_toggle(app, i.max(0) as usize),
                    "cmdToggleTagAll" => app.s.fea.file.features.iter_mut().for_each(|x| x.tagged = !x.tagged),
                    "cmdTagAll" => app.s.fea.file.features.iter_mut().for_each(|x| x.tagged = true),
                    "cmdUntagAll" => app.s.fea.file.features.iter_mut().for_each(|x| x.tagged = false),
                    "cmdClear" => {
                        app.s.fea = FeaState::default();
                        app.form(name).c("Label3").caption = String::new();
                        app.doc(name).file = None;
                        app.changed(name, false);
                    }
                    "cmdNext" if (app.s.fea.page + 1) * 12 < n => app.s.fea.page += 1,
                    "cmdPrevious" if app.s.fea.page > 0 => app.s.fea.page -= 1,
                    _ => {}
                }
                fea_page(app);
            }
            // A double-click's two clicks tag and untag again, then the value opens for editing.
            Ev::DblClick(c, i) if c == "Label2" || c == "Label1" || c == "Frame3D1" => fea_edit(app, i.max(0) as usize),
            Ev::Menu(m) => match m.as_str() {
                "menfileopen" => fea_open(app),
                "menfilesave" => fea_save(app, false, false),
                "menfilesaveas" => fea_save(app, true, false),
                "menfilesavetag" => fea_save(app, true, true),
                "menfileprint" => {
                    let mut lines = vec![app.tor("PROGRAMMABLE FEATURES", "pfPF", "Programmable Features"), String::new()];
                    for x in &app.s.fea.file.features {
                        lines.push(format!("{:<4} {:<22} {}", x.number(), x.name(), x.display()));
                    }
                    app.print("Programmable Features", &lines);
                }
                "menfileexport" => {
                    let Some(p) = app.file_dialog(true, "Export Programmable Features", "Text (*.TXT)", "txt", "FEA", "") else { return };
                    let body: Vec<String> = app.s.fea.file.features.iter().map(|x| format!("\"{}\",\"{}\",\"{}\"", x.number(), x.name(), x.display())).collect();
                    let _ = std::fs::write(p, body.join("\r\n"));
                }
                "menfileexit" => close_doc(app, name),
                "mentransdown" | "mentranspdown" => {
                    let q = app.t("TELELINK", "goverwrite", "This will overwrite the data currently stored in your phone.  Do you want to continue?");
                    let t = app.g.title.clone();
                    app.msg(name, if m == "mentransdown" { "download" } else { "tagged" }, &q, 36, &t);
                }
                "mentransup" => fea_upload(app),
                _ => {}
            },
            _ => {}
        },
        "frmGetProgrammableFeatures" => match ev {
            Ev::Click(c, _) if c == "Command1" => fea_apply_edit(app),
            Ev::Click(c, _) if c == "Command2" => {
                app.s.fea.editing = None;
                app.unload(name);
            }
            Ev::Close => app.unload(name),
            _ => {}
        },
        "frmAOSScript" => aos_event(app, ev),
        "frmCSNTable" | "frmCSNiTable" | "frmUPTTable" => {
            if let Ev::DblClick(c, _) = &ev {
                if c == "List1" {
                    return csn_event(app, name, Ev::Click("cmdEdit".into(), -1));
                }
            }
            if let Ev::Click(c, _) = &ev {
                if c == "List1" {
                    csn_show(app, name);
                    return;
                }
            }
            csn_event(app, name, ev)
        }
        "frmCSNEntry" => match ev {
            Ev::Click(c, _) if c == "cmdOk" => csn_entry_ok(app),
            Ev::Click(c, _) if c == "cmdCancel" => app.unload(name),
            Ev::Close => app.unload(name),
            _ => {}
        },
        "frmPhoneList" => phones_event(app, ev),
        "frmUpdatePhoneListEntry" => match ev {
            Ev::Click(c, _) if c == "Command1" => phone_entry_ok(app),
            Ev::Click(c, _) if c == "Command2" => app.unload(name),
            Ev::Close => app.unload(name),
            _ => {}
        },
        _ => {}
    }
}

pub fn answer(app: &mut App, name: &str, tag: &str, a: Answer) {
    match tag {
        "save_exit" => match a {
            Answer::Yes => {
                match name {
                    "frmProgrammableFeatures" => fea_save(app, false, false),
                    "frmPhoneList" => phones_save(app, false),
                    "frmAOSScript" => aos_event(app, Ev::Menu("menfilesave".into())),
                    _ => csn_save(app, name, false),
                }
                if !app.doc(name).dirty {
                    app.unload(name);
                }
            }
            Answer::No => app.unload(name),
            _ => {}
        },
        "save_clear" => match a {
            Answer::Yes => {
                if name == "frmPhoneList" {
                    phones_save(app, false);
                    phones_clear(app);
                } else {
                    csn_save(app, name, false);
                    csn_clear(app, name);
                }
            }
            Answer::No => {
                if name == "frmPhoneList" {
                    phones_clear(app)
                } else {
                    csn_clear(app, name)
                }
            }
            _ => {}
        },
        "download" | "tagged" if a == Answer::Yes => match name {
            "frmProgrammableFeatures" => fea_download(app, tag == "tagged"),
            "frmAOSScript" => {
                if let Some((n, b)) = app.s.logs.aos_bytes.take() {
                    app.transfer(name, "download", Op::WriteAos(n, b));
                }
            }
            _ => match csn_entries(app, name) {
                Ok(e) => app.transfer(name, "download", Op::WriteCsn(e)),
                Err(e) => app.say(&e, "CSN Table"),
            },
        },
        _ => {}
    }
}

pub fn done(app: &mut App, name: &str, _what: &str, r: Result<Res, String>) {
    let title = app.form(name).caption.clone();
    match &r {
        Ok(Res::Areas(areas)) if name == "frmProgrammableFeatures" => {
            for x in app.s.fea.file.features.iter_mut() {
                let (a, n) = (x.address() as usize, x.len());
                for (start, data) in areas {
                    let s = *start as usize;
                    if a >= s && a + n <= s + data.len() {
                        x.set_value_bytes(&data[a - s..a - s + n]);
                    }
                }
            }
            app.changed(name, true);
            fea_page(app);
        }
        Ok(Res::Csn(rows)) => {
            app.s.csn.rows.insert(name.into(), rows.clone());
            app.changed(name, true);
            csn_show(app, name);
        }
        _ => {}
    }
    super::report_outcome(app, &title, &r);
}

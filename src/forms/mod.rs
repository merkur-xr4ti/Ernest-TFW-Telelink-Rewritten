//! The form code of TFW.MAK, one module per group of forms. `load` is Form_Load, `event` the
//! control event procedures, `answer` the code after a MsgBox, `tick` the Timer controls.

pub mod batch;
pub mod connect;
pub mod direct;
pub mod lists;
pub mod logs;
pub mod main_form;
pub mod reports;
pub mod sysconf;

use crate::app::App;
use crate::modem::{Job, Op, Res};
use crate::vb::{Answer, Ev};
use std::collections::HashMap;
use std::path::PathBuf;

/// What a form keeps besides its controls (the form-level `Dim m...` variables).
#[derive(Default)]
pub struct States {
    pub main: main_form::MainState,
    pub connect: connect::ConnectState,
    /// Working file, unsaved-changes flag and running transfer of each editing window.
    pub doc: HashMap<String, Doc>,
    pub fea: lists::FeaState,
    pub aos: lists::AosState,
    pub csn: lists::CsnState,
    pub phones: lists::PhoneState,
    pub batch: batch::BatchState,
    pub logs: logs::LogState,
    pub report: reports::ReportState,
}

#[derive(Default)]
pub struct Doc {
    pub file: Option<PathBuf>,
    pub dirty: bool,
    /// A transfer waiting for the connection, or running: (what, the operation, its job).
    pub pending: Option<(String, Op)>,
    pub job: Option<(String, Job)>,
}

impl App {
    pub fn doc(&mut self, form: &str) -> &mut Doc {
        self.s.doc.entry(form.to_string()).or_default()
    }
    /// Start a transfer for a form: on the open call, or after frmConnect makes one.
    pub fn transfer(&mut self, form: &str, what: &str, op: Op) {
        if self.modem.connected().is_some() {
            let job = self.modem.run(op);
            self.doc(form).job = Some((what.to_string(), job));
        } else {
            self.doc(form).pending = Some((what.to_string(), op));
            self.g.after_connect = Some((form.to_string(), what.to_string()));
            self.show("frmConnect", true);
        }
    }
    /// The "Don't forget to save your changes!" label and the dirty flag.
    pub fn changed(&mut self, form: &str, dirty: bool) {
        self.doc(form).dirty = dirty;
        if let Some(f) = self.try_form(form) {
            if let Some(i) = f.find("lblChange", -1) {
                f.ctls[i].visible = dirty;
                f.ctls[i].tag = if dirty { "Yes".into() } else { "No".into() };
            }
        }
    }
}

/// The connection is up: run what a Direct window asked for.
pub fn connected(app: &mut App) {
    if let Some((form, what)) = app.g.after_connect.take() {
        if what == "upload_after" {
            app.s.logs.fea_upload_after_connect = false;
            lists::fea_upload(app);
            return;
        }
        if what == "download_after" || what == "tagged_after" {
            lists::fea_download(app, what == "tagged_after");
            return;
        }
        if let Some((what, op)) = app.doc(&form).pending.take() {
            let job = app.modem.run(op);
            app.doc(&form).job = Some((what, job));
        }
    }
}

pub fn load(app: &mut App, name: &str) {
    match name {
        "Telelink" | "frmAbout" | "frmSplash" | "frmVersionInfo" | "frmModemSearch" | "frmNoModems" | "ModemDetection" | "frmMessage" => main_form::load(app, name),
        "frmConnect" => connect::load(app),
        "frmSpeedDial" | "frmTimeDate" | "ScrollIt" | "frmRateTable" | "firmware" => direct::load(app, name),
        "frmProgrammableFeatures" | "frmGetProgrammableFeatures" | "frmAOSScript" | "frmCSNTable" | "frmCSNiTable" | "frmUPTTable" | "frmCSNEntry" | "frmPhoneList" | "frmUpdatePhoneListEntry" => {
            lists::load(app, name)
        }
        "frmBatchSetup" | "frmBatchSetupChina" | "frmBatchExec" | "frmBatchSchedule" | "frmIncomingSchedule" => batch::load(app, name),
        "frmSysConfig" => sysconf::load(app),
        "frmSMDR" | "frmCollectionStatus" | "frmCallFlags" | "frmIncome" | "frmLoadingCollectionsData" => logs::load(app, name),
        _ => reports::load(app, name),
    }
}

/// Form_Activate (after Show).
pub fn activate(app: &mut App, name: &str) {
    if name == "frmConnect" {
        connect::activate(app);
    }
}

pub fn unload(app: &mut App, name: &str) {
    if let Some(d) = app.s.doc.get_mut(name) {
        if let Some((_, j)) = &d.job {
            j.cancel.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
    app.s.doc.remove(name);
}

pub fn event(app: &mut App, name: &str, ev: Ev) {
    // "Quick Exit" (mnuqe) on a window closes only that window, asking about unsaved changes
    // first (frmProgrammableFeatures sub08BC: sub0F7D, then Hide), the same as its close box.
    let ev = if ev == Ev::Menu("mnuqe".into()) {
        if name == "Telelink" {
            main_form::quick_exit(app);
            return;
        }
        Ev::Close
    } else {
        ev
    };
    match name {
        "Telelink" | "frmAbout" | "frmSplash" | "frmVersionInfo" | "frmModemSearch" | "frmNoModems" | "ModemDetection" | "frmMessage" => main_form::event(app, name, ev),
        "frmConnect" => connect::event(app, ev),
        "frmSpeedDial" | "frmTimeDate" | "ScrollIt" | "frmRateTable" | "firmware" => direct::event(app, name, ev),
        "frmProgrammableFeatures" | "frmGetProgrammableFeatures" | "frmAOSScript" | "frmCSNTable" | "frmCSNiTable" | "frmUPTTable" | "frmCSNEntry" | "frmPhoneList" | "frmUpdatePhoneListEntry" => {
            lists::event(app, name, ev)
        }
        "frmBatchSetup" | "frmBatchSetupChina" | "frmBatchExec" | "frmBatchSchedule" | "frmIncomingSchedule" => batch::event(app, name, ev),
        "frmSysConfig" => sysconf::event(app, ev),
        "frmSMDR" | "frmCollectionStatus" | "frmCallFlags" | "frmIncome" | "frmLoadingCollectionsData" => logs::event(app, name, ev),
        _ => reports::event(app, name, ev),
    }
}

pub fn answer(app: &mut App, owner: &str, tag: &str, a: Answer) {
    match owner {
        "Telelink" => main_form::answer(app, tag, a),
        "frmConnect" => connect::answer(app, tag, a),
        "frmSpeedDial" | "frmTimeDate" | "ScrollIt" | "frmRateTable" | "firmware" => direct::answer(app, owner, tag, a),
        "frmProgrammableFeatures" | "frmAOSScript" | "frmCSNTable" | "frmCSNiTable" | "frmUPTTable" | "frmPhoneList" | "frmUpdatePhoneListEntry" | "frmCSNEntry" | "frmGetProgrammableFeatures" => {
            lists::answer(app, owner, tag, a)
        }
        "frmBatchSetup" | "frmBatchSetupChina" | "frmBatchExec" | "frmBatchSchedule" | "frmIncomingSchedule" => batch::answer(app, owner, tag, a),
        "frmSysConfig" => sysconf::answer(app, tag, a),
        "frmSMDR" | "frmCollectionStatus" | "frmCallFlags" | "frmIncome" => logs::answer(app, owner, tag, a),
        _ => reports::answer(app, owner, tag, a),
    }
}

pub fn later(app: &mut App, owner: &str, tag: &str) {
    match owner {
        "Telelink" => main_form::later(app, tag),
        _ => {}
    }
}

/// Timer controls and finished transfers.
pub fn tick(app: &mut App) {
    main_form::tick(app);
    connect::tick(app);
    batch::tick(app);
    direct::scroll_tick(app);
    transfer_status(app);
    // Transfers of the Direct windows.
    let names: Vec<String> = app.s.doc.keys().cloned().collect();
    for n in names {
        let done = match app.s.doc.get(&n).and_then(|d| d.job.as_ref()) {
            Some((what, j)) => j.take().map(|r| (what.clone(), r)),
            None => None,
        };
        if let Some((what, r)) = done {
            app.doc(&n).job = None;
            transfer_done(app, &n, &what, r);
        }
    }
}

/// While a window's transfer runs, frmMessage (TMESSAGE.FRM, the original's "please wait"
/// form) shows what is being sent or received and how far it has got; the windows with their
/// own progress bar (Scrolling Message, Rate Table, Firmware) fill it in too.
fn transfer_status(app: &mut App) {
    let running: Vec<(String, String, String)> = app
        .s
        .doc
        .iter()
        .filter_map(|(n, d)| d.job.as_ref().filter(|(_, j)| j.running()).map(|(w, j)| (n.clone(), w.clone(), j.status())))
        .collect();
    let Some((form, what, status)) = running.into_iter().next() else {
        if app.s.main.transfer_msg {
            app.s.main.transfer_msg = false;
            app.unload("frmMessage");
        }
        return;
    };
    let pct: Option<i32> = status.split_whitespace().find_map(|w| w.strip_suffix('%')).and_then(|n| n.parse().ok());
    let verb = if what == "upload" { app.t("RATE TABLE", "rtUploading", "Uploading") } else { app.t("TELELINK", "bDownloading", "Downloading") };
    let title = app.try_form(&form).map(|f| f.caption.clone()).unwrap_or_default();
    if !app.s.main.transfer_msg {
        app.s.main.transfer_msg = true;
        app.show("frmMessage", false);
        let f = app.form("frmMessage");
        let l = f.c("lblMessage");
        (l.top, l.height, l.left, l.width) = (240.0, 1095.0, 240.0, 4140.0);
    }
    let shown = if status.is_empty() { format!("{verb}...") } else { status };
    let f = app.form("frmMessage");
    f.caption = format!("{title} - {verb}");
    f.c("lblMessage").caption = shown;
    let gauge = match form.as_str() {
        "ScrollIt" | "frmRateTable" => Some("Gauge1"),
        "firmware" => Some("control18"),
        _ => None,
    };
    if let (Some(g), Some(p)) = (gauge, pct) {
        if let Some(f) = app.try_form(&form) {
            let c = f.c(g);
            c.visible = true;
            c.max = 100;
            c.value = p.clamp(0, 100);
        }
    }
}

fn transfer_done(app: &mut App, form: &str, what: &str, r: Result<Res, String>) {
    if let Some(f) = app.try_form(form) {
        for g in ["Gauge1", "control18"] {
            if let Some(i) = f.find(g, -1) {
                f.ctls[i].visible = false;
                f.ctls[i].value = 0;
            }
        }
    }
    match form {
        "frmSpeedDial" | "frmTimeDate" | "ScrollIt" | "frmRateTable" | "firmware" => direct::done(app, form, what, r),
        "frmProgrammableFeatures" | "frmAOSScript" | "frmCSNTable" | "frmCSNiTable" | "frmUPTTable" => lists::done(app, form, what, r),
        "frmSMDR" | "frmCollectionStatus" => logs::done(app, form, what, r),
        _ => {}
    }
}

/// A transfer's outcome as the original reported it.
pub fn report_outcome(app: &mut App, title: &str, r: &Result<Res, String>) {
    match r {
        Ok(_) => {
            let t = app.t("TELELINK", "xSuccessfulTransfer", "Successful Transfer");
            app.msg("", "", &t, 64, title);
        }
        Err(e) => app.msg("", "", e, 48, title),
    }
}

/// The standard Transfer menu of a Direct window (mentransfer / mentransdown / mentransup).
pub fn transfer_menu(app: &App, section: &str, extra: Vec<crate::vb::MenuItem>) -> crate::vb::MenuItem {
    use crate::vb::MenuItem;
    let mut items = vec![MenuItem::new("mentransdown", app.tor(section, "mentransdown", "&Download")), MenuItem::new("mentransup", app.tor(section, "mentransup", "&Upload"))];
    items.extend(extra);
    MenuItem::sub("mentransfer", app.tor(section, "mentransfer", "&Transfer"), items)
}

/// Days of the week as the text table names them (gv11C6..gv1226), Sunday first.
pub fn day_names(app: &App) -> Vec<String> {
    ["sunday", "monday", "tuesday", "wednesday", "thursday", "friday", "saturday"]
        .iter()
        .zip(["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"])
        .map(|(k, d)| app.tor("TELELINK", k, d))
        .collect()
}

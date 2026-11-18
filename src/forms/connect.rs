//! CONNECT.FRM: "Connect to a Phone". Dials, waits for the modem's CONNECT with a countdown,
//! sends the ID and reports the outcome; then the Direct window that asked carries on.

use crate::app::App;
use crate::modem::{DialConf, Job, Res, Target};
use crate::vb::{Answer, Ev};
use std::time::Instant;

#[derive(Default)]
pub struct ConnectState {
    pub job: Option<Job>,
    pub started: Option<Instant>,
    pub timeout: i32,

}

const SEC: &str = "CONNECT";

pub fn load(app: &mut App) {
    let t = |k: &str, d: &str| app.tor(SEC, k, d);
    let caps = [
        ("Label4", t("conPhoneNumber", "Phone Number")),
        ("Label5", t("conIDNumber", "ID Number")),
        ("cmdOk", t("conOK", "&OK")),
        ("cmdLookup", t("conLookup", "&Lookup")),
        ("cmdCancel", t("conCancel", "&Cancel")),
        ("cmdSkip", t("conSkip", "&Skip")),
        ("Label8", t("conTime", "Time Remaining")),
        ("Label10", t("conPhoneNumber", "Phone Number")),
        ("Label11", t("conIDNumber", "ID Number")),
    ];
    let title = t("Title", "Connect to a Phone");
    let frame = format!(" {} ", t("FrameTitle", "Phone To Call"));
    let ptype = t("conPhoneType", "Phone Type");
    let usa = app.g.usa;
    let d3 = app.form("Telelink").text("cmdPhoneType") == app.g.d3;
    let f = app.form("frmConnect");
    f.caption = title;
    for (n, c) in caps {
        f.set_caption(n, -1, &(if n.starts_with("cmd") { c } else { c }));
    }
    f.c("fraUSA").caption = frame.clone();
    f.c("fraChina").caption = frame;
    f.c("optD3").caption = format!("{ptype} D3 / ETX");
    f.c("optD1").caption = format!("{ptype} D1 / Data");
    // Form_Load positions the frames (fraUSA at 120,120 2420 high).
    f.c("fraUSA").top = 120.0;
    f.c("fraUSA").height = 2420.0;
    f.c("fraUSA").visible = usa;
    f.c("fraChina").visible = !usa;
    f.c("fraCallStatus").visible = false;
    f.c("optD3").value = d3 as i32;
    f.c("optD1").value = (!d3) as i32;
    f.c("cmdOk").default = true;
    app.s.connect = ConnectState::default();
}

pub fn activate(_app: &mut App) {}

fn fail(app: &mut App, text: &str, title: &str) {
    app.msg("frmConnect", "invalid", text, 48, title);
}

fn ok(app: &mut App) {
    let f = app.form("frmConnect");
    let (pic, one, area, num, id) = if f.get("fraUSA").visible {
        (f.get("mePIC").clip_text(), f.get("meOnePlus").clip_text(), f.get("meAreaCode").clip_text(), f.get("mePhoneNumber").clip_text(), f.get("meID").clip_text())
    } else {
        (String::new(), String::new(), String::new(), f.get("meChinaPhoneNumber").text.chars().filter(|c| c.is_ascii_digit()).collect(), f.get("meChinaID").clip_text())
    };
    let d3 = f.value("optD3") == 1;
    let t = |app: &App, k: &str, d: &str| app.tor(SEC, k, d);
    if !pic.is_empty() && pic.len() < 3 {
        let (a, b) = (t(app, "xEnterPIC", "Please enter a valid PIC Number."), t(app, "xErrPIC", "Error: Invalid PIC Number"));
        return fail(app, &a, &b);
    }
    if !one.is_empty() && one != "1" {
        let (a, b) = (t(app, "xEnter1", "Please enter a 1 or press the DELETE key for this entry."), t(app, "xErr1", "Error: Invalid One Plus"));
        return fail(app, &a, &b);
    }
    if !area.is_empty() && area.len() != 3 {
        let (a, b) = (t(app, "xEnterArea", "Please enter a valid Area Code"), t(app, "xErrArea", "Error: Invalid Area Code"));
        return fail(app, &a, &b);
    }
    if num.len() < 7 {
        let (a, b) = (t(app, "xEnterPhone", "Please enter a valid Phone Number."), t(app, "xErrPhone", "Error: Invalid Phone Number"));
        return fail(app, &a, &b);
    }
    if id.len() != 6 {
        let (a, b) = (t(app, "xEnterID", "Please enter a valid ID Number."), t(app, "xErrID", "Error: Invalid ID Number"));
        return fail(app, &a, &b);
    }
    let mut dial = String::new();
    if !pic.is_empty() {
        dial.push_str(&format!("101{pic}"));
    }
    dial.push_str(&one);
    dial.push_str(&area);
    dial.push_str(&num);
    let shown = if area.is_empty() { format!("{}-{}", &num[..3], &num[3..]) } else { format!("{area}-{}-{}", &num[..3], &num[3..]) };
    app.g.conn_area = area.clone();
    app.g.conn_number = num.clone();
    let timeout = if area.is_empty() && one.is_empty() { app.g.dial_local } else { app.g.dial_ld };
    let target = Target { dial, shown: shown.clone(), id, d3 };
    let conf = DialConf {
        init_addons: app.g.init_addons.clone(),
        d3_addons: app.g.d3_addons.clone(),
        d1_addons: app.g.d1_addons.clone(),
        pbx: app.g.pbx.clone(),
        dcw: app.g.dcw.clone(),
        timeout: (timeout + 5).max(30) as u32, // ATS7= the timeout + 5 (frmConnect)
    };
    let waiting = app.tor(SEC, "conWaiting", "Waiting for connection...");
    let f = app.form("frmConnect");
    f.c("fraUSA").visible = false;
    f.c("fraChina").visible = false;
    f.c("fraCallStatus").visible = true;
    f.c("lblStatus").caption = waiting;
    f.c("lblNumberCalled").caption = shown;
    f.c("lblTimeRemaining").caption = format!("{timeout}");
    f.c("cmdOk").enabled = false;
    f.c("cmdLookup").enabled = false;
    f.c("optD1").enabled = false;
    f.c("optD3").enabled = false;
    app.g.outgoing = true;
    let job = app.modem.dial(target, conf);
    app.s.connect = ConnectState { job: Some(job), started: Some(Instant::now()), timeout };
}

fn back_to_entry(app: &mut App) {
    let usa = app.g.usa;
    let f = app.form("frmConnect");
    f.c("fraUSA").visible = usa;
    f.c("fraChina").visible = !usa;
    f.c("fraCallStatus").visible = false;
    f.c("cmdOk").enabled = true;
    f.c("cmdLookup").enabled = true;
    f.c("optD1").enabled = true;
    f.c("optD3").enabled = true;
    app.s.connect.job = None;
}

pub fn tick(app: &mut App) {
    let Some(job) = app.s.connect.job.clone() else { return };
    if !app.loaded("frmConnect") {
        app.s.connect.job = None;
        return;
    }
    let left = app.s.connect.started.map(|t| app.s.connect.timeout - t.elapsed().as_secs() as i32).unwrap_or(0).max(0);
    let status = job.status();
    {
        let f = app.form("frmConnect");
        f.c("lblTimeRemaining").caption = format!("{left}");
        if !status.is_empty() {
            f.c("lblStatus").caption = status;
        }
    }
    if let Some(r) = job.take() {
        match r {
            Ok(Res::Connected { firmware }) => {
                let _ = firmware;
                app.g.connected = true;
                app.s.connect.job = None;
                app.unload("frmConnect");
                super::connected(app);
            }
            Ok(_) => back_to_entry(app),
            Err(e) => {
                back_to_entry(app);
                let t = app.tor(SEC, "Title", "Connect to a Phone");
                app.msg("frmConnect", "failed", &e, 48, &t);
            }
        }
    }
}

pub fn answer(_app: &mut App, _tag: &str, _a: Answer) {}

pub fn event(app: &mut App, ev: Ev) {
    match ev {
        Ev::Click(c, _) => match c.as_str() {
            "cmdOk" => ok(app),
            "cmdCancel" => {
                if let Some(j) = &app.s.connect.job {
                    j.cancel.store(true, std::sync::atomic::Ordering::SeqCst);
                    let m = app.t("TELELINK", "itcAbortedByUser", "Aborted By User.");
                    app.form("frmConnect").c("lblStatus").caption = m;
                } else {
                    app.g.after_connect = None;
                    app.unload("frmConnect");
                }
            }
            "cmdLookup" => {
                app.g.lookup = true;
                app.g.phone_list_as_log = false;
                app.unload("frmPhoneList");
                app.show("frmPhoneList", true);
            }
            _ => {}
        },
        Ev::Close => {
            if app.s.connect.job.is_none() {
                app.g.after_connect = None;
                app.unload("frmConnect");
            }
        }
        _ => {}
    }
}

/// From the phone list's Lookup: fill in the phone to call.
pub fn fill(app: &mut App, phone: &str, id: &str, ptype: &str) {
    let d: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    let (one, area, num) = match d.len() {
        11 => (d[..1].to_string(), d[1..4].to_string(), d[4..].to_string()),
        10 => (String::new(), d[..3].to_string(), d[3..].to_string()),
        _ => (String::new(), String::new(), d.clone()),
    };
    let f = app.form("frmConnect");
    f.c("meOnePlus").set_masked(&one);
    f.c("meAreaCode").set_masked(&area);
    f.c("mePhoneNumber").set_masked(&num);
    f.c("meID").set_masked(id);
    f.c("meChinaPhoneNumber").text = d;
    f.c("meChinaID").set_masked(id);
    let d1 = ptype.trim().eq_ignore_ascii_case("D1");
    f.c("optD1").value = d1 as i32;
    f.c("optD3").value = (!d1) as i32;
}

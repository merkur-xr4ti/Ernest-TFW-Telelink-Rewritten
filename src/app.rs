//! The VB3 "project": every form of TFW.MAK, the global variables the forms share, message
//! boxes, and the run loop that shows each visible form in its own window.

use crate::forms;
use crate::modem::Modem;
use crate::text::{Profile, TextTable};
use crate::vb::{self, Ev, Form, MsgBox};
use eframe::egui::{self, Vec2, ViewportBuilder, ViewportCommand, ViewportId};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// The .FRM layout blocks of TFW.MAK, by form name.
pub fn frm_source(name: &str) -> Option<&'static str> {
    Some(match name {
        "Telelink" => include_str!("../forms/TELELINK.FRM"),
        "frmReport" => include_str!("../forms/REPORT.FRM"),
        "frmAOSScript" => include_str!("../forms/AOSSCRPT.FRM"),
        "frmUPTTable" => include_str!("../forms/T_UPT.FRM"),
        "frmProgrammableFeatures" => include_str!("../forms/FEATURES.FRM"),
        "ScrollIt" => include_str!("../forms/SCROLLIT.FRM"),
        "frmSpeedDial" => include_str!("../forms/SPEEDIAL.FRM"),
        "frmTimeDate" => include_str!("../forms/TIMEDATE.FRM"),
        "frmConnect" => include_str!("../forms/CONNECT.FRM"),
        "frmCSNEntry" => include_str!("../forms/CSNENTRY.FRM"),
        "frmPhoneList" => include_str!("../forms/T_PHONE.FRM"),
        "frmUpdatePhoneListEntry" => include_str!("../forms/U_PHONE.FRM"),
        "frmBatchSetup" => include_str!("../forms/BATSETUP.FRM"),
        "frmBatchExec" => include_str!("../forms/BATCHEXE.FRM"),
        "frmIncome" => include_str!("../forms/INCOME.FRM"),
        "frmSplash" => include_str!("../forms/SPLASH.FRM"),
        "frmGetProgrammableFeatures" => include_str!("../forms/GETPF.FRM"),
        "frmBatchSchedule" => include_str!("../forms/BATSCHED.FRM"),
        "frmSysConfig" => include_str!("../forms/SYSCONF.FRM"),
        "frmMessage" => include_str!("../forms/TMESSAGE.FRM"),
        "frmCallFlags" => include_str!("../forms/CALLFLGS.FRM"),
        "frmIncomingSchedule" => include_str!("../forms/INCSCHED.FRM"),
        "frmRateTable" => include_str!("../forms/T_RATE.FRM"),
        "frmBatchSetupChina" => include_str!("../forms/BATSETCH.FRM"),
        "ModemDetection" => include_str!("../forms/MODEMDET.FRM"),
        "frmModemSearch" => include_str!("../forms/FRMMODEM.FRM"),
        "frmNoModems" => include_str!("../forms/FRMNOMOD.FRM"),
        "frmMDA" => include_str!("../forms/FRMMDA.FRM"),
        "frmCSNiTable" => include_str!("../forms/T_CSNI.FRM"),
        "frmCSNTable" => include_str!("../forms/T_CSN.FRM"),
        "frmSMDR" => include_str!("../forms/SMDR.FRM"),
        "frmAbout" => include_str!("../forms/ABOUT.FRM"),
        "frmCollectionStatus" => include_str!("../forms/COLLECT.FRM"),
        "frmUpdateSMDRCDF" => include_str!("../forms/FRMUPDAT.FRM"),
        "frmLoadingCollectionsData" => include_str!("../forms/FRMLOADI.FRM"),
        "frmVersionInfo" => include_str!("../forms/FRMVERSI.FRM"),
        "frmCDRGenerator" => include_str!("../forms/FRMCDRGE.FRM"),
        "frmProcessingReports" => include_str!("../forms/FRMPROCE.FRM"),
        "frmCDRWorking" => include_str!("../forms/CDRWORK.FRM"),
        "frmCDRViewer" => include_str!("../forms/FRMCDRVI.FRM"),
        "firmware" => include_str!("../forms/FIRMWARE.FRM"),
        _ => return None,
    })
}

const VBX: &str = include_str!("../forms/vbx.txt");

/// Globals of Globals.bas that the forms share, under readable names (the gv number each one
/// stands for is in its comment).
#[derive(Default)]
pub struct G {
    /// gv1192 msTitle, the main window's caption and every message box's title.
    pub title: String,
    pub on: String,   // gv08BE
    pub off: String,  // gv08CE
    pub yes: String,  // gv08AE
    pub no: String,   // gv089E
    pub d1: String,   // gv08DE
    pub d3: String,   // gv08EE
    /// gv08FE "Not Connected To A Phone".
    pub not_connected: String,
    /// gv0894 upper-case country; gv0300 = USA when `usa`.
    pub country: String,
    pub usa: bool,
    /// gv02DE: the phone list extension (PLW).
    pub plw: String,
    /// gv02E8 / gv02E6 dial timeouts (Local / OnePlus).
    pub dial_local: i32,
    pub dial_ld: i32,
    /// gv17BC: beep with message boxes.
    pub sound: bool,
    /// gv0328 serial port, gv17CA inter-command delay (ms).
    pub serial_port: i32,
    pub delay: i32,
    /// gv032A PBX prefix, gv032E disable-call-waiting code.
    pub pbx: String,
    pub dcw: String,
    /// gv17CE/gv17D2 init string + addons, gv17DA D3 addons, gv17E2 D1 addons.
    pub init_cmd: String,
    pub init_addons: String,
    pub d3_addons: String,
    pub d1_addons: String,
    /// gv0294: an AutoCall batch is running.
    pub autocall_running: bool,
    /// gv029E: a batch is running (incoming calls held off).
    pub batch_running: bool,
    /// gv0344: a call is up.
    pub connected: bool,
    /// gv03C0/gv03C4: area code and number of the connected phone.
    pub conn_area: String,
    pub conn_number: String,
    /// gv178E: the connection was made by this program (not an incoming call).
    pub outgoing: bool,
    /// gv03B4: frmCallFlags mode ("New" or "Review"); gv03A4 frmIncome mode.
    pub flags_mode: String,
    pub income_mode: String,
    /// gv03AA: frmPhoneList opened as the batch log viewer.
    pub phone_list_as_log: bool,
    /// gv0846: frmPhoneList opened for Lookup from frmConnect.
    pub lookup: bool,
    /// What frmConnect should do once connected (a Direct window's transfer).
    pub after_connect: Option<(String, String)>,
    /// gv0948: debug toggle (double-click the date).
    pub debug: bool,
    /// gv0F68.. BatchAutoPrint settings.
    pub ap_unpolled: String,
    pub ap_minimum: String,
    pub ap_monthly: String,
    pub ap_flags: String,
    pub ap_mda: String,
    /// gv093A printer font size.
    pub font_size: i32,
    /// gv0884 [Language] International.
    pub international: String,
    /// The modem answered at start-up.
    pub modem_ok: bool,
}

pub struct Shot {
    pub path: PathBuf,
    pub frames: u32,
    pub requested: bool,
    pub done: Arc<AtomicBool>,
}

pub struct App {
    pub prof: Profile,
    pub txt: TextTable,
    pub forms: Vec<Form>,
    pub msgs: Vec<MsgBox>,
    msg_seq: u64,
    pub modem: Modem,
    pub g: G,
    pub s: forms::States,
    pub shot: Option<Shot>,
    pub quit: bool,
    last_title: String,
    /// One-shot jobs to run after a delay (VB code that waited with sub01A3 / DoEvents loops).
    pub later: Vec<(Instant, String, String)>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, screenshot: Option<PathBuf>) -> Self {
        vb::install_fonts(&cc.egui_ctx);
        style(&cc.egui_ctx);
        let prof = Profile::new();
        let intl = prof.get("Language", "International");
        let country = prof.get("System", "Country");
        let txt = TextTable::load(intl.eq_ignore_ascii_case("ON"), &country);
        let mut app = App {
            prof,
            txt,
            forms: Vec::new(),
            msgs: Vec::new(),
            msg_seq: 0,
            modem: Modem::start(),
            g: G::default(),
            s: forms::States::default(),
            shot: screenshot.map(|path| Shot { path, frames: 0, requested: false, done: Arc::new(AtomicBool::new(false)) }),
            quit: false,
            last_title: String::new(),
            later: Vec::new(),
        };
        app.load("Telelink");
        app
    }

    /// Without a window (`--selftest`): load every form and run a few of its events.
    pub fn selftest() -> i32 {
        let prof = Profile::new();
        let txt = TextTable::load(false, "USA");
        let mut app = App {
            prof,
            txt,
            forms: Vec::new(),
            msgs: Vec::new(),
            msg_seq: 0,
            modem: Modem::start(),
            g: G::default(),
            s: forms::States::default(),
            shot: Some(Shot { path: PathBuf::new(), frames: 0, requested: false, done: Arc::new(AtomicBool::new(false)) }),
            quit: false,
            last_title: String::new(),
            later: Vec::new(),
        };
        app.load("Telelink");
        let names = [
            "frmReport", "frmAOSScript", "frmUPTTable", "frmProgrammableFeatures", "ScrollIt", "frmSpeedDial", "frmTimeDate", "frmConnect", "frmCSNEntry", "frmPhoneList",
            "frmUpdatePhoneListEntry", "frmBatchSetup", "frmBatchExec", "frmIncome", "frmSplash", "frmGetProgrammableFeatures", "frmBatchSchedule", "frmSysConfig", "frmMessage",
            "frmCallFlags", "frmIncomingSchedule", "frmRateTable", "frmBatchSetupChina", "ModemDetection", "frmModemSearch", "frmNoModems", "frmMDA", "frmCSNiTable", "frmCSNTable",
            "frmSMDR", "frmAbout", "frmCollectionStatus", "frmUpdateSMDRCDF", "frmLoadingCollectionsData", "frmVersionInfo", "frmCDRGenerator", "frmProcessingReports",
            "frmCDRWorking", "frmCDRViewer", "firmware",
        ];
        let mut bad = 0;
        for n in names {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                app.show(n, false);
                let ctls: Vec<(String, i32)> = app.form(n).ctls.iter().map(|c| (c.name.clone(), c.index)).collect();
                for (c, i) in ctls {
                    if c.starts_with("cmdOk") || c.starts_with("Command1") || c == "cmdCancel" || c == "cmdTest" {
                        continue;
                    }
                    forms::event(&mut app, n, Ev::Click(c.clone(), i));
                    forms::event(&mut app, n, Ev::Change(c, i));
                }
                forms::tick(&mut app);
                app.unload(n);
            }));
            match r {
                Ok(()) => println!("ok   {n}"),
                Err(_) => {
                    println!("FAIL {n}");
                    bad += 1;
                }
            }
            app.msgs.clear();
        }
        bad
    }

    pub fn screenshot_mode(&self) -> bool {
        self.shot.is_some()
    }

    /// fn0589 for the program's own captions.
    pub fn t(&self, section: &str, key: &str, fallback: &str) -> String {
        self.txt.t(section, key, fallback)
    }
    /// A caption that keeps the form's design-time text when the table lacks the key.
    pub fn tor(&self, section: &str, key: &str, design: &str) -> String {
        self.txt.or(section, key, design)
    }

    pub fn loaded(&self, name: &str) -> bool {
        self.forms.iter().any(|f| f.name == name)
    }
    pub fn visible(&self, name: &str) -> bool {
        self.forms.iter().any(|f| f.name == name && f.visible)
    }
    pub fn form(&mut self, name: &str) -> &mut Form {
        if !self.loaded(name) {
            self.load(name);
        }
        self.forms.iter_mut().find(|f| f.name == name).expect("form")
    }
    pub fn try_form(&mut self, name: &str) -> Option<&mut Form> {
        self.forms.iter_mut().find(|f| f.name == name)
    }

    /// `Load Form`: parse its layout and run Form_Load.
    pub fn load(&mut self, name: &str) {
        if self.loaded(name) {
            return;
        }
        let Some(src) = frm_source(name) else { return };
        let mut f = Form::parse(src, VBX);
        f.name = name.to_string();
        self.forms.push(f);
        forms::load(self, name);
    }
    /// `Form.Show` (modal = `Show 1`).
    pub fn show(&mut self, name: &str, modal: bool) {
        self.load(name);
        if let Some(f) = self.try_form(name) {
            f.visible = true;
            f.modal = modal;
        }
        forms::activate(self, name);
    }
    /// `Unload Form`.
    pub fn unload(&mut self, name: &str) {
        if name == "Telelink" {
            self.quit = true;
            return;
        }
        if self.loaded(name) {
            forms::unload(self, name);
            self.forms.retain(|f| f.name != name);
        }
    }

    /// MsgBox; the answer comes back to `forms::answer(owner, tag)`.
    pub fn msg(&mut self, owner: &str, tag: &str, text: &str, flags: i32, title: &str) {
        self.msg_seq += 1;
        if self.g.sound {
            beep();
        }
        self.msgs.push(MsgBox { id: self.msg_seq, text: text.to_string(), title: title.to_string(), flags, owner: owner.to_string(), tag: tag.to_string() });
    }
    /// A plain OK message.
    pub fn say(&mut self, text: &str, title: &str) {
        self.msg("", "", text, 48, title);
    }

    /// Run `forms::later(owner, tag)` after `ms` milliseconds.
    pub fn after(&mut self, ms: u64, owner: &str, tag: &str) {
        self.later.push((Instant::now() + Duration::from_millis(ms), owner.to_string(), tag.to_string()));
    }

    pub fn dir(&self, sub: &str) -> PathBuf {
        crate::text::path(sub)
    }

    /// The common dialog (Telelink.CMDialog1): Open or Save As with the original's filter,
    /// folder and title. Returns the chosen path.
    pub fn file_dialog(&self, save: bool, title: &str, filter_name: &str, ext: &str, dir: &str, file: &str) -> Option<PathBuf> {
        if self.shot.as_ref().is_some_and(|s| s.path.as_os_str().is_empty()) {
            return None; // --selftest
        }
        let d = self.dir(dir);
        let _ = std::fs::create_dir_all(&d);
        let mut fd = rfd::FileDialog::new().set_title(title).set_directory(&d);
        if !ext.is_empty() {
            fd = fd.add_filter(filter_name, &[ext, &ext.to_ascii_uppercase()]);
        }
        if !file.is_empty() {
            fd = fd.set_file_name(file);
        }
        if save { fd.save_file() } else { fd.pick_file() }
    }

    /// Printer.Print: the lines go to a text file the system prints.
    pub fn print(&mut self, title: &str, lines: &[String]) {
        let dir = self.dir("TEMP");
        let _ = std::fs::create_dir_all(&dir);
        let p = dir.join(format!("{}.txt", title.replace([' ', '/', '\\', ':'], "_")));
        let body = lines.join("\r\n");
        match std::fs::write(&p, body) {
            Ok(()) => {
                let _ = std::process::Command::new("notepad.exe").arg("/p").arg(&p).spawn();
            }
            Err(e) => self.say(&format!("Error {e}"), title),
        }
    }

    fn frame(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        // Timers and waits.
        forms::tick(self);
        let now = Instant::now();
        let due: Vec<(String, String)> = self.later.iter().filter(|(t, _, _)| *t <= now).map(|(_, o, g)| (o.clone(), g.clone())).collect();
        self.later.retain(|(t, _, _)| *t > now);
        for (o, g) in due {
            forms::later(self, &o, &g);
        }

        let blocked_all = !self.msgs.is_empty();
        let top_modal = self.forms.iter().rev().find(|f| f.visible && f.modal).map(|f| f.name.clone());
        let blocked = |name: &str| blocked_all || top_modal.as_deref().is_some_and(|m| m != name);

        // The main form fills the program's own window.
        let root = std::env::var("TFW_PREVIEW_FORM").unwrap_or_else(|_| "Telelink".into());
        if root != "Telelink" && !self.loaded(&root) {
            self.show(&root, false);
        }
        let main_blocked = blocked(root.as_str());
        if let Some(main) = self.forms.iter_mut().find(|f| f.name == root.as_str()) {
            if main.caption != self.last_title {
                self.last_title = main.caption.clone();
                ctx.send_viewport_cmd(ViewportCommand::Title(main.caption.clone()));
            }
            if root != "Telelink" {
                let s = main.size_px();
                ctx.send_viewport_cmd(ViewportCommand::MinInnerSize(s));
                ctx.send_viewport_cmd(ViewportCommand::MaxInnerSize(s));
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(s));
            }
            vb::render(ui, main, main_blocked);
        }
        if ctx.input(|i| i.viewport().close_requested()) && !self.quit {
            ctx.send_viewport_cmd(ViewportCommand::CancelClose);
            if let Some(m) = self.try_form("Telelink") {
                m.events.push(Ev::Close);
            }
        }

        // Every other visible form in a window of its own.
        let names: Vec<String> = self.forms.iter().filter(|f| f.visible && f.name != root).map(|f| f.name.clone()).collect();
        for name in names {
            let b = blocked(&name);
            let Some(f) = self.forms.iter_mut().find(|f| f.name == name) else { continue };
            let size = f.size_px();
            let builder = ViewportBuilder::default()
                .with_title(f.caption.clone())
                .with_inner_size(size)
                .with_min_inner_size(size)
                .with_max_inner_size(size)
                .with_position(egui::pos2((f.left / vb::TWIPS).max(0.0), (f.top / vb::TWIPS).max(0.0) + 20.0))
                .with_resizable(false)
                .with_maximize_button(false)
                .with_close_button(f.control_box);
            ctx.show_viewport_immediate(ViewportId::from_hash_of(("form", name.as_str())), builder, |ui, _| {
                egui::CentralPanel::default().frame(egui::Frame::NONE).show(ui, |ui| vb::render(ui, f, b));
                if ui.ctx().input(|i| i.viewport().close_requested()) {
                    ui.ctx().send_viewport_cmd(ViewportCommand::CancelClose);
                    if !b {
                        f.events.push(Ev::Close);
                    }
                }
            });
        }

        // The front message box.
        if let Some(m) = self.msgs.first().cloned() {
            let size = m.size_px(&ctx);
            let builder = ViewportBuilder::default()
                .with_title(m.title.clone())
                .with_inner_size(size)
                .with_resizable(false)
                .with_minimize_button(false)
                .with_maximize_button(false)
                .with_window_level(egui::WindowLevel::AlwaysOnTop);
            let ans = ctx.show_viewport_immediate(ViewportId::from_hash_of(("msgbox", m.id)), builder, |ui, _| {
                let mut a = None;
                egui::CentralPanel::default().frame(egui::Frame::NONE).show(ui, |ui| a = m.ui(ui));
                if ui.ctx().input(|i| i.viewport().close_requested()) {
                    ui.ctx().send_viewport_cmd(ViewportCommand::CancelClose);
                    a = Some(*m.buttons().last().unwrap());
                }
                a
            });
            if let Some(a) = ans {
                self.msgs.remove(0);
                if !m.owner.is_empty() {
                    forms::answer(self, &m.owner, &m.tag, a);
                }
            }
        }

        // Event procedures.
        for _ in 0..4 {
            let mut any = false;
            let names: Vec<String> = self.forms.iter().map(|f| f.name.clone()).collect();
            for name in names {
                let evs = match self.try_form(&name) {
                    Some(f) => std::mem::take(&mut f.events),
                    None => continue,
                };
                for ev in evs {
                    any = true;
                    forms::event(self, &name, ev);
                }
            }
            if !any {
                break;
            }
        }
    }
}

pub fn beep() {
    #[cfg(windows)]
    {
        unsafe extern "system" {
            fn MessageBeep(t: u32) -> i32;
        }
        unsafe {
            MessageBeep(0);
        }
    }
}

fn style(ctx: &egui::Context) {
    let mut v = egui::Visuals::light();
    v.panel_fill = vb::BTN_FACE;
    v.window_fill = vb::BTN_FACE;
    v.extreme_bg_color = egui::Color32::WHITE;
    v.selection.bg_fill = vb::NAVY;
    v.selection.stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
    v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);
    v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, egui::Color32::BLACK);
    v.menu_corner_radius = egui::CornerRadius::ZERO;
    v.window_corner_radius = egui::CornerRadius::ZERO;
    ctx.set_theme(egui::Theme::Light);
    ctx.set_visuals_of(egui::Theme::Light, v.clone());
    ctx.set_visuals_of(egui::Theme::Dark, v);
    ctx.all_styles_mut(|s| {
        s.spacing.item_spacing = Vec2::new(4.0, 2.0);
        s.interaction.selectable_labels = false;
    });
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ui, |ui| self.frame(ui));
        if self.quit {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        // --screenshot: draw about 120 frames, save the window's picture and exit.
        if let Some(shot) = &mut self.shot {
            shot.frames += 1;
            ctx.request_repaint();
            if shot.frames >= 120 && !shot.requested {
                shot.requested = true;
                let path = shot.path.clone();
                let done = shot.done.clone();
                let repaint = ctx.clone();
                ctx.request_screenshot(move |img| {
                    if let Err(e) = save_png(&path, &img) {
                        eprintln!("screenshot failed: {e}");
                    }
                    done.store(true, Ordering::SeqCst);
                    repaint.request_repaint();
                });
            }
            if shot.done.load(Ordering::SeqCst) || shot.frames > 1500 {
                self.quit = true;
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        } else {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
    }
}

fn save_png(path: &Path, img: &egui::ColorImage) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let [w, h] = img.size;
    let mut buf = Vec::with_capacity(w * h * 4);
    for c in &img.pixels {
        buf.extend_from_slice(&[c.r(), c.g(), c.b(), 255]);
    }
    image::save_buffer(path, &buf, w as u32, h as u32, image::ColorType::Rgba8).map_err(|e| e.to_string())
}


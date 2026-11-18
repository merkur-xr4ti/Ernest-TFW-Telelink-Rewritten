//! A small Visual Basic 3 form runtime: it reads the original .FRM layout blocks (twips), keeps
//! every control's properties the way the VB code addresses them (`Name(Index).Property`), draws
//! them in the Windows 3.1 look and turns clicks and edits into events for the form code.

use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontFamily, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2,
    text::{CCursor, CCursorRange, LayoutJob, TextFormat},
};

pub const TWIPS: f32 = 15.0;
pub const MENU_H: f32 = 20.0;
pub const BTN_FACE: Color32 = Color32::from_rgb(192, 192, 192);
pub const DARK: Color32 = Color32::from_rgb(128, 128, 128);
pub const NAVY: Color32 = Color32::from_rgb(0, 0, 128);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Form,
    Label,
    TextBox,
    CommandButton,
    CheckBox,
    OptionButton,
    Frame,
    SSFrame,
    SSCommand,
    SSCheck,
    SSRibbon,
    SSPanel,
    ListBox,
    FileListBox,
    ComboBox,
    MaskEdBox,
    Grid,
    Gauge,
    SpinButton,
    Graph,
    Image,
    Shape,
    Hidden,
}

impl Kind {
    fn from_vb(t: &str) -> Kind {
        match t {
            "Form" => Kind::Form,
            "Label" => Kind::Label,
            "TextBox" => Kind::TextBox,
            "CommandButton" => Kind::CommandButton,
            "CheckBox" => Kind::CheckBox,
            "OptionButton" => Kind::OptionButton,
            "Frame" => Kind::Frame,
            "SSFrame" => Kind::SSFrame,
            "SSCommand" => Kind::SSCommand,
            "SSCheck" => Kind::SSCheck,
            "SSRibbon" => Kind::SSRibbon,
            "SSPanel" => Kind::SSPanel,
            "ListBox" => Kind::ListBox,
            "FileListBox" => Kind::FileListBox,
            "ComboBox" => Kind::ComboBox,
            "MaskEdBox" => Kind::MaskEdBox,
            "Grid" => Kind::Grid,
            "Gauge" => Kind::Gauge,
            "SpinButton" => Kind::SpinButton,
            "GRAPH" => Kind::Graph,
            "Image" => Kind::Image,
            "UnknownControl_0x16" => Kind::Shape,
            _ => Kind::Hidden,
        }
    }
    fn is_container(self) -> bool {
        matches!(self, Kind::Frame | Kind::SSFrame | Kind::SSPanel)
    }
}

#[derive(Clone, Debug)]
pub struct Ctl {
    pub kind: Kind,
    pub name: String,
    pub index: i32,
    pub parent: Option<usize>,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub caption: String,
    pub text: String,
    pub value: i32,
    pub visible: bool,
    pub enabled: bool,
    pub fore: Option<Color32>,
    pub back: Option<Color32>,
    pub bold: bool,
    pub size: f32,
    pub font: String,
    pub align: u8,
    pub border: u8,
    pub mask: String,
    pub max_len: usize,
    pub multiline: bool,
    pub list: Vec<String>,
    pub list_index: i32,
    pub sorted: bool,
    pub tag: String,
    pub style: i32,
    /// Grid cells, row 0 is the fixed header row.
    pub cells: Vec<Vec<String>>,
    pub col_widths: Vec<f32>,
    pub row: i32,
    pub max: i32,
    /// FileListBox folder and pattern.
    pub path: String,
    pub pattern: String,
    /// Graph data points and type (0 bars, 1 line).
    pub data: Vec<f32>,
    pub labels: Vec<String>,
    pub graph_type: i32,
    pub default: bool,
    /// When a FileListBox last read its folder (it rescans at most once a second).
    pub scanned: Option<std::time::Instant>,
}

impl Ctl {
    fn new(kind: Kind, name: &str) -> Ctl {
        Ctl {
            kind,
            name: name.to_string(),
            index: -1,
            parent: None,
            left: 0.0,
            top: 0.0,
            width: 0.0,
            height: 0.0,
            caption: String::new(),
            text: String::new(),
            value: 0,
            visible: true,
            enabled: true,
            fore: None,
            back: None,
            bold: true,
            size: 8.25,
            font: "MS Sans Serif".into(),
            align: 0,
            border: if matches!(kind, Kind::TextBox | Kind::ListBox | Kind::FileListBox) { 1 } else { 0 },
            mask: String::new(),
            max_len: 0,
            multiline: false,
            list: Vec::new(),
            list_index: -1,
            sorted: false,
            tag: String::new(),
            style: 0,
            cells: Vec::new(),
            col_widths: Vec::new(),
            row: 0,
            max: 100,
            path: String::new(),
            pattern: "*.*".into(),
            data: Vec::new(),
            labels: Vec::new(),
            graph_type: 0,
            default: false,
            scanned: None,
        }
    }

    /// `AddItem` (sorted lists keep their order).
    pub fn add_item(&mut self, s: impl Into<String>) {
        let s = s.into();
        if self.sorted {
            let low = s.to_ascii_lowercase();
            let at = self.list.iter().position(|x| x.to_ascii_lowercase() > low).unwrap_or(self.list.len());
            self.list.insert(at, s);
        } else {
            self.list.push(s);
        }
    }
    pub fn clear(&mut self) {
        self.list.clear();
        self.list_index = -1;
    }
    pub fn selected(&self) -> Option<&String> {
        if self.list_index >= 0 { self.list.get(self.list_index as usize) } else { None }
    }
    /// A masked box's characters without literals and prompts (ClipText).
    pub fn clip_text(&self) -> String {
        if self.mask.is_empty() {
            return self.text.clone();
        }
        let m: Vec<char> = self.mask.chars().collect();
        self.text
            .chars()
            .enumerate()
            .filter(|(i, c)| m.get(*i).is_some_and(|mc| is_placeholder(*mc)) && *c != '_')
            .map(|(_, c)| c)
            .collect()
    }
    /// Set a masked box from plain characters.
    pub fn set_masked(&mut self, raw: &str) {
        self.text = if self.mask.is_empty() { raw.to_string() } else { mask_format(raw, &self.mask) };
    }
}

fn is_placeholder(c: char) -> bool {
    matches!(c, '#' | '&' | 'A' | 'a' | '?' | '9' | 'C')
}

fn fits(c: char, m: char) -> bool {
    match m {
        '#' | '9' => c.is_ascii_digit(),
        '?' => c.is_ascii_alphabetic(),
        'A' | 'a' => c.is_ascii_alphanumeric(),
        _ => !c.is_control(),
    }
}

/// Lay characters into a mask's places; unfilled places show the prompt `_`.
pub fn mask_format(raw: &str, mask: &str) -> String {
    let mut src = raw.chars().filter(|c| *c != '_').peekable();
    let mut out = String::new();
    for m in mask.chars() {
        if is_placeholder(m) {
            let mut put = '_';
            while let Some(&c) = src.peek() {
                src.next();
                if fits(c, m) {
                    put = c;
                    break;
                }
            }
            out.push(put);
        } else {
            out.push(m);
            if src.peek() == Some(&m) {
                src.next();
            }
        }
    }
    out
}

fn parse_color(v: &str) -> Option<Color32> {
    let h = v.split('\'').next().unwrap_or("").trim().trim_start_matches("&H").trim_end_matches('&');
    let n = u32::from_str_radix(h, 16).ok()?;
    if n & 0x8000_0000 != 0 {
        return None; // system colour
    }
    Some(Color32::from_rgb((n & 0xFF) as u8, ((n >> 8) & 0xFF) as u8, ((n >> 16) & 0xFF) as u8))
}

fn unquote(v: &str) -> String {
    let v = v.trim();
    if let Some(rest) = v.strip_prefix('"') {
        let mut out = String::new();
        let mut it = rest.chars().peekable();
        while let Some(c) = it.next() {
            if c == '"' {
                if it.peek() == Some(&'"') {
                    it.next();
                    out.push('"');
                } else {
                    break;
                }
            } else {
                out.push(c);
            }
        }
        out
    } else {
        v.split('\'').next().unwrap_or("").trim().to_string()
    }
}

#[derive(Clone, Debug, Default)]
pub struct MenuItem {
    pub name: String,
    pub caption: String,
    pub enabled: bool,
    pub visible: bool,
    pub checked: bool,
    pub children: Vec<MenuItem>,
}

impl MenuItem {
    pub fn new(name: &str, caption: impl Into<String>) -> MenuItem {
        MenuItem { name: name.into(), caption: caption.into(), enabled: true, visible: true, checked: false, children: vec![] }
    }
    pub fn sub(name: &str, caption: impl Into<String>, children: Vec<MenuItem>) -> MenuItem {
        MenuItem { children, ..MenuItem::new(name, caption) }
    }
    pub fn sep() -> MenuItem {
        MenuItem::new("-", "-")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ev {
    Click(String, i32),
    DblClick(String, i32),
    Change(String, i32),
    LostFocus(String, i32),
    SpinUp(String, i32),
    SpinDown(String, i32),
    Menu(String),
    /// The window's close box.
    Close,
}

#[derive(Clone, Debug)]
pub struct Form {
    pub name: String,
    pub caption: String,
    pub client_w: f32,
    pub client_h: f32,
    pub left: f32,
    pub top: f32,
    pub control_box: bool,
    pub back: Color32,
    pub ctls: Vec<Ctl>,
    pub menus: Vec<MenuItem>,
    pub events: Vec<Ev>,
    pub visible: bool,
    pub enabled: bool,
    /// Shown with `Show 1`: other windows wait until it closes.
    pub modal: bool,
}

impl Form {
    /// Parse a .FRM layout block. `extra` holds geometry for the VBX controls the decompiled
    /// file lost (lines `Form Control Index Left Top Width Height [ Prop:Key=Value|...]`).
    pub fn parse(frm: &str, extra: &str) -> Form {
        let mut form = Form {
            name: String::new(),
            caption: String::new(),
            client_w: 4000.0,
            client_h: 3000.0,
            left: 1000.0,
            top: 1000.0,
            control_box: true,
            back: BTN_FACE,
            ctls: Vec::new(),
            menus: Vec::new(),
            events: Vec::new(),
            visible: false,
            enabled: true,
            modal: false,
        };
        let mut stack: Vec<Option<usize>> = Vec::new(); // None = the form itself
        let mut current: Option<Option<usize>> = None;
        for line in frm.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("Begin ") {
                let mut parts = rest.split_whitespace();
                let ty = parts.next().unwrap_or("");
                let name = parts.next().unwrap_or("");
                let kind = Kind::from_vb(ty);
                if kind == Kind::Form {
                    form.name = name.to_string();
                    stack.push(None);
                    current = Some(None);
                } else {
                    let parent = stack.last().copied().flatten();
                    let mut c = Ctl::new(kind, name);
                    c.parent = parent;
                    form.ctls.push(c);
                    let idx = form.ctls.len() - 1;
                    stack.push(Some(idx));
                    current = Some(Some(idx));
                }
                continue;
            }
            if t == "End" {
                stack.pop();
                current = stack.last().copied();
                continue;
            }
            let Some((k, v)) = t.split_once('=') else { continue };
            let k = k.trim();
            match current {
                Some(None) => {
                    let n = || v.split('\'').next().unwrap_or("").trim().parse::<f32>().unwrap_or(0.0);
                    match k {
                        "Caption" => form.caption = unquote(v),
                        "ClientWidth" => form.client_w = n(),
                        "ClientHeight" => form.client_h = n(),
                        "Left" => form.left = n(),
                        "Top" => form.top = n(),
                        "ControlBox" => form.control_box = n() != 0.0,
                        "BackColor" => {
                            if let Some(c) = parse_color(v) {
                                form.back = c
                            }
                        }
                        _ => {}
                    }
                }
                Some(Some(i)) => set_prop(&mut form.ctls[i], k, v),
                None => {}
            }
        }
        // Controls of one name without an Index property: the decompiler lists them last-first.
        let mut names: Vec<String> = form.ctls.iter().map(|c| c.name.clone()).collect();
        names.sort();
        names.dedup();
        for n in names {
            let at: Vec<usize> = form.ctls.iter().enumerate().filter(|(_, c)| c.name == n && c.index < 0).map(|(i, _)| i).collect();
            if at.len() > 1 {
                let count = at.len() as i32;
                for (k, i) in at.into_iter().enumerate() {
                    form.ctls[i].index = count - 1 - k as i32;
                }
            }
        }
        // Lost VBX geometry and properties.
        for line in extra.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }
            let (head, props) = match line.find(" Prop:") {
                Some(i) => (&line[..i], Some(&line[i + 6..])),
                None => (line, None),
            };
            let mut p = head.split_whitespace();
            if p.next() != Some(form.name.as_str()) {
                continue;
            }
            let name = p.next().unwrap_or("");
            let index: i32 = p.next().and_then(|x| x.parse().ok()).unwrap_or(-1);
            let nums: Vec<f32> = p.filter_map(|x| x.parse().ok()).collect();
            let Some(ci) = form.ctls.iter().position(|c| c.name.eq_ignore_ascii_case(name) && (c.index == index || index < 0)) else {
                continue;
            };
            if nums.len() == 4 {
                let c = &mut form.ctls[ci];
                (c.left, c.top, c.width, c.height) = (nums[0], nums[1], nums[2], nums[3]);
            }
            if let Some(rest) = props {
                for kv in rest.split('|') {
                    if let Some((k, v)) = kv.split_once('=') {
                        set_prop(&mut form.ctls[ci], k.trim(), v.trim());
                    }
                }
            }
        }
        form
    }

    /// Add a control that isn't in the original form (VB type name, e.g. "TextBox"), inside
    /// `parent` when given. Position and size in twips; `props` as in a .FRM (`Caption=...`).
    pub fn add(&mut self, vb_type: &str, name: &str, index: i32, parent: Option<&str>, rect: [f32; 4], props: &[(&str, &str)]) {
        let mut c = Ctl::new(Kind::from_vb(vb_type), name);
        c.index = index;
        c.parent = parent.and_then(|p| self.find(p, -1));
        (c.left, c.top, c.width, c.height) = (rect[0], rect[1], rect[2], rect[3]);
        for (k, v) in props {
            set_prop(&mut c, k, v);
        }
        self.ctls.push(c);
    }

    pub fn find(&self, name: &str, index: i32) -> Option<usize> {
        self.ctls.iter().position(|c| c.name.eq_ignore_ascii_case(name) && (index < 0 || c.index == index))
    }
    /// `Name` (or the first of an array).
    pub fn c(&mut self, name: &str) -> &mut Ctl {
        let i = self.find(name, -1).unwrap_or_else(|| panic!("{}: no control {name}", self.name));
        &mut self.ctls[i]
    }
    /// `Name(index)`.
    pub fn ci(&mut self, name: &str, index: i32) -> &mut Ctl {
        let i = self.find(name, index).unwrap_or_else(|| panic!("{}: no control {name}({index})", self.name));
        &mut self.ctls[i]
    }
    pub fn get(&self, name: &str) -> &Ctl {
        &self.ctls[self.find(name, -1).unwrap_or_else(|| panic!("{}: no control {name}", self.name))]
    }
    pub fn geti(&self, name: &str, index: i32) -> &Ctl {
        &self.ctls[self.find(name, index).unwrap_or_else(|| panic!("{}: no control {name}({index})", self.name))]
    }
    pub fn has(&self, name: &str, index: i32) -> bool {
        self.find(name, index).is_some()
    }
    pub fn text(&self, name: &str) -> String {
        self.get(name).text.clone()
    }
    pub fn texti(&self, name: &str, i: i32) -> String {
        self.geti(name, i).text.clone()
    }
    pub fn value(&self, name: &str) -> i32 {
        self.get(name).value
    }
    pub fn valuei(&self, name: &str, i: i32) -> i32 {
        self.geti(name, i).value
    }
    /// Set `Caption` of every control of that name that exists (quietly skips missing ones).
    pub fn set_caption(&mut self, name: &str, index: i32, cap: &str) {
        if let Some(i) = self.find(name, index) {
            self.ctls[i].caption = cap.to_string();
        }
    }

    pub fn menu_mut(&mut self, name: &str) -> Option<&mut MenuItem> {
        fn walk<'a>(items: &'a mut [MenuItem], name: &str) -> Option<&'a mut MenuItem> {
            for it in items.iter_mut() {
                if it.name.eq_ignore_ascii_case(name) {
                    return Some(it);
                }
                if let Some(x) = walk(&mut it.children, name) {
                    return Some(x);
                }
            }
            None
        }
        walk(&mut self.menus, name)
    }
    pub fn menu_enable(&mut self, name: &str, on: bool) {
        if let Some(m) = self.menu_mut(name) {
            m.enabled = on;
        }
    }

    /// Absolute position of a control in the client area (twips).
    fn abs_origin(&self, i: usize) -> (f32, f32) {
        let c = &self.ctls[i];
        let (mut x, mut y) = (c.left, c.top);
        let mut p = c.parent;
        while let Some(pi) = p {
            x += self.ctls[pi].left;
            y += self.ctls[pi].top;
            p = self.ctls[pi].parent;
        }
        (x, y)
    }
    fn usable(&self, i: usize) -> bool {
        let mut p = Some(i);
        while let Some(k) = p {
            if !self.ctls[k].enabled {
                return false;
            }
            p = self.ctls[k].parent;
        }
        self.enabled
    }

    pub fn size_px(&self) -> Vec2 {
        let menu = if self.menus.iter().any(|m| m.visible) { MENU_H } else { 0.0 };
        Vec2::new(self.client_w / TWIPS, self.client_h / TWIPS + menu)
    }
}

fn set_prop(c: &mut Ctl, k: &str, v: &str) {
    let num = || v.split('\'').next().unwrap_or("").trim().parse::<f32>().unwrap_or(0.0);
    match k {
        "Caption" => c.caption = unquote(v),
        "Text" => c.text = unquote(v),
        "Left" => c.left = num(),
        "Top" => c.top = num(),
        "Width" => c.width = num(),
        "Height" => c.height = num(),
        "Index" => c.index = num() as i32,
        "Visible" => c.visible = num() != 0.0,
        "Enabled" => c.enabled = num() != 0.0,
        "Alignment" => c.align = num() as u8,
        "BorderStyle" => c.border = num() as u8,
        "ForeColor" => c.fore = parse_color(v),
        "BackColor" => c.back = parse_color(v),
        "FontBold" => c.bold = num() != 0.0,
        "FontSize" => c.size = num(),
        "FontName" => c.font = unquote(v),
        "Value" => c.value = if num() != 0.0 { 1 } else { 0 },
        "Mask" => c.mask = unquote(v),
        // A TextBox with PasswordChar shows dots; its text is kept in `text` as usual.
        "PasswordChar" => c.mask = unquote(v),
        "MaxLength" => c.max_len = num() as usize,
        "MultiLine" => c.multiline = num() != 0.0,
        "Sorted" => c.sorted = num() != 0.0,
        "Tag" => c.tag = unquote(v),
        "Style" => c.style = num() as i32,
        "Pattern" => c.pattern = unquote(v),
        "Max" => c.max = num() as i32,
        "Default" => c.default = num() != 0.0,
        "Cols" => {
            let n = num() as usize;
            c.col_widths = vec![1200.0; n];
            c.cells = vec![vec![String::new(); n]];
        }
        _ => {}
    }
}

/// Strip VB accelerator marks: `&File` → `File`, `&&` → `&`.
pub fn plain(s: &str) -> String {
    let mut out = String::new();
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if c == '&' {
            if it.peek() == Some(&'&') {
                it.next();
                out.push('&');
            }
        } else {
            out.push(c);
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Fonts (the system's own TrueType fonts, loaded at start-up)

pub fn install_fonts(ctx: &egui::Context) {
    let mut defs = egui::FontDefinitions::default();
    let dir = std::env::var("WINDIR").map(|w| format!("{w}\\Fonts")).unwrap_or_else(|_| "C:\\Windows\\Fonts".into());
    let fallback = defs.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let mut add = |family: &str, files: &[&str]| {
        for f in files {
            if let Ok(b) = std::fs::read(format!("{dir}\\{f}")) {
                defs.font_data.insert(family.to_string(), std::sync::Arc::new(egui::FontData::from_owned(b)));
                let mut list = vec![family.to_string()];
                list.extend(fallback.clone());
                defs.families.insert(FontFamily::Name(family.into()), list);
                return;
            }
        }
        defs.families.insert(FontFamily::Name(family.into()), fallback.clone());
    };
    add("sans", &["micross.ttf", "tahoma.ttf", "arial.ttf"]);
    add("sansb", &["tahomabd.ttf", "arialbd.ttf", "micross.ttf"]);
    add("mono", &["cour.ttf", "consola.ttf"]);
    add("monob", &["courbd.ttf", "consolab.ttf", "cour.ttf"]);
    add("arial", &["arial.ttf", "micross.ttf"]);
    add("arialb", &["arialbd.ttf", "tahomabd.ttf"]);
    if let Some(list) = defs.families.get(&FontFamily::Name("sans".into())).cloned() {
        defs.families.insert(FontFamily::Proportional, list);
    }
    ctx.set_fonts(defs);
}

pub fn font_of(name: &str, bold: bool, size_pt: f32) -> FontId {
    let n = name.to_ascii_lowercase();
    let fam = if n.contains("courier") || n.contains("terminal") || n.contains("fixedsys") {
        if bold { "monob" } else { "mono" }
    } else if n.contains("arial") {
        if bold { "arialb" } else { "arial" }
    } else if bold {
        "sansb"
    } else {
        "sans"
    };
    // The bold TrueType stand-ins run wider than MS Sans Serif's bitmap bold; a point smaller
    // keeps the original captions on the lines they had.
    let px = size_pt * 96.0 / 72.0 * if bold && !fam.starts_with("mono") { 0.9 } else { 1.0 };
    FontId::new(px.round().max(8.0), FontFamily::Name(fam.into()))
}

pub fn ui_font() -> FontId {
    font_of("MS Sans Serif", false, 8.25)
}
pub fn ui_font_bold() -> FontId {
    font_of("MS Sans Serif", true, 8.25)
}

// ---------------------------------------------------------------------------------------------
// Drawing helpers in the Windows 3.1 style

pub fn bevel(p: &egui::Painter, r: Rect, raised: bool, width: f32) {
    let (tl, br) = if raised { (Color32::WHITE, DARK) } else { (DARK, Color32::WHITE) };
    for k in 0..width as i32 {
        let r = r.shrink(k as f32);
        let (a, b, c, d) = (r.left_top(), r.right_top() - Vec2::new(1.0, 0.0), r.left_bottom() - Vec2::new(0.0, 1.0), r.right_bottom() - Vec2::new(1.0, 1.0));
        p.line_segment([a, b], Stroke::new(1.0, tl));
        p.line_segment([a, c], Stroke::new(1.0, tl));
        p.line_segment([c, d], Stroke::new(1.0, br));
        p.line_segment([b, d], Stroke::new(1.0, br));
    }
}

pub fn sunken_box(p: &egui::Painter, r: Rect, fill: Color32) {
    p.rect_filled(r, CornerRadius::ZERO, fill);
    p.rect_stroke(r, CornerRadius::ZERO, Stroke::new(1.0, Color32::BLACK), StrokeKind::Inside);
}

pub fn button_face(p: &egui::Painter, r: Rect, pressed: bool, default: bool) {
    p.rect_filled(r, CornerRadius::ZERO, BTN_FACE);
    p.rect_stroke(r, CornerRadius::same(2), Stroke::new(1.0, Color32::BLACK), StrokeKind::Inside);
    let inner = r.shrink(if default { 2.0 } else { 1.0 });
    if default {
        p.rect_stroke(r.shrink(1.0), CornerRadius::ZERO, Stroke::new(1.0, Color32::BLACK), StrokeKind::Inside);
    }
    if pressed {
        p.rect_stroke(inner, CornerRadius::ZERO, Stroke::new(1.0, DARK), StrokeKind::Inside);
    } else {
        bevel(p, inner, true, 2.0);
    }
}

fn text_job(text: &str, font: FontId, color: Color32, wrap: f32, align: u8) -> LayoutJob {
    let mut job = LayoutJob::single_section(text.to_string(), TextFormat { font_id: font, color, ..Default::default() });
    job.wrap.max_width = wrap;
    job.halign = match align {
        1 => egui::Align::RIGHT,
        2 => egui::Align::Center,
        _ => egui::Align::LEFT,
    };
    job
}

/// Paint wrapped text in a rectangle, aligned like a VB label (0 left, 1 right, 2 centre).
pub fn paint_text(p: &egui::Painter, r: Rect, text: &str, font: FontId, color: Color32, align: u8, wrap: bool) {
    let w = if wrap { r.width().max(1.0) } else { f32::INFINITY };
    let job = text_job(text, font, color, w, align);
    let g = p.fonts_mut(|f| f.layout_job(job));
    // With halign the galley is laid out around its anchor: left edge, centre or right edge.
    let x = match align {
        1 => r.right(),
        2 => r.center().x,
        _ => r.left(),
    };
    p.with_clip_rect(r.expand(1.0)).galley(Pos2::new(x, r.top()), g, color);
}

pub fn text_width(p: &egui::Painter, text: &str, font: FontId) -> f32 {
    p.fonts_mut(|f| f.layout_no_wrap(text.to_string(), font, Color32::BLACK)).size().x
}

// ---------------------------------------------------------------------------------------------
// Rendering a form

/// Draw the menu bar and the client area of a form; collect its events.
pub fn render(ui: &mut egui::Ui, form: &mut Form, blocked: bool) {
    let full = ui.max_rect();
    let p = ui.painter().clone();
    p.rect_filled(full, CornerRadius::ZERO, form.back);
    let mut origin = full.min;
    if form.menus.iter().any(|m| m.visible) {
        let bar = Rect::from_min_size(full.min, Vec2::new(full.width(), MENU_H));
        p.rect_filled(bar, CornerRadius::ZERO, Color32::WHITE);
        p.line_segment([bar.left_bottom(), bar.right_bottom()], Stroke::new(1.0, Color32::BLACK));
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(bar.shrink2(Vec2::new(2.0, 0.0))).layout(egui::Layout::left_to_right(egui::Align::Center)));
        if blocked || !form.enabled {
            child.disable();
        }
        menu_bar(&mut child, form);
        origin.y += MENU_H;
    }
    let area = Rect::from_min_max(origin, full.max);
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(area));
    if blocked || !form.enabled {
        child.disable();
    }
    // Top-level controls: the first listed is the topmost, so draw in reverse.
    let order: Vec<usize> = (0..form.ctls.len()).filter(|&i| form.ctls[i].parent.is_none()).rev().collect();
    for i in order {
        draw_tree(&mut child, form, i, origin, blocked);
    }
}

fn draw_tree(ui: &mut egui::Ui, form: &mut Form, i: usize, origin: Pos2, blocked: bool) {
    if !form.ctls[i].visible {
        return;
    }
    draw_ctl(ui, form, i, origin, blocked);
    if form.ctls[i].kind.is_container() {
        let kids: Vec<usize> = (0..form.ctls.len()).filter(|&k| form.ctls[k].parent == Some(i)).rev().collect();
        for k in kids {
            draw_tree(ui, form, k, origin, blocked);
        }
    }
}

fn menu_bar(ui: &mut egui::Ui, form: &mut Form) {
    let mut clicked: Option<String> = None;
    ui.spacing_mut().button_padding = Vec2::new(6.0, 1.0);
    ui.spacing_mut().item_spacing.x = 0.0;
    {
        let w = &mut ui.visuals_mut().widgets;
        for s in [&mut w.inactive, &mut w.noninteractive] {
            s.weak_bg_fill = Color32::TRANSPARENT;
            s.bg_fill = Color32::TRANSPARENT;
            s.bg_stroke = Stroke::NONE;
        }
        for s in [&mut w.hovered, &mut w.active, &mut w.open] {
            s.weak_bg_fill = NAVY;
            s.bg_fill = NAVY;
            s.bg_stroke = Stroke::NONE;
            s.corner_radius = CornerRadius::ZERO;
            s.fg_stroke = Stroke::new(1.0, Color32::WHITE);
        }
    }
    for m in form.menus.iter().filter(|m| m.visible) {
        let cap = plain(&m.caption);
        if m.children.is_empty() {
            let b = ui.add_enabled(m.enabled, egui::Button::new(egui::RichText::new(cap).font(ui_font()).color(Color32::BLACK)).frame(false));
            if b.clicked() {
                clicked = Some(m.name.clone());
            }
        } else {
            ui.add_enabled_ui(m.enabled, |ui| {
                ui.menu_button(egui::RichText::new(cap).font(ui_font()).color(Color32::BLACK), |ui| {
                    ui.set_min_width(170.0);
                    menu_items(ui, &m.children, &mut clicked);
                });
            });
        }
    }
    if let Some(c) = clicked {
        form.events.push(Ev::Menu(c));
    }
}

fn menu_items(ui: &mut egui::Ui, items: &[MenuItem], clicked: &mut Option<String>) {
    for it in items.iter().filter(|m| m.visible) {
        if it.name == "-" {
            ui.separator();
            continue;
        }
        let mut cap = plain(&it.caption);
        if it.checked {
            cap = format!("\u{2713} {cap}");
        }
        if it.children.is_empty() {
            if ui.add_enabled(it.enabled, egui::Button::new(egui::RichText::new(cap).font(ui_font())).frame(false)).clicked() {
                *clicked = Some(it.name.clone());
                ui.close();
            }
        } else {
            ui.add_enabled_ui(it.enabled, |ui| {
                ui.menu_button(egui::RichText::new(cap).font(ui_font()), |ui| menu_items(ui, &it.children, clicked));
            });
        }
    }
}

fn draw_ctl(ui: &mut egui::Ui, form: &mut Form, i: usize, origin: Pos2, blocked: bool) {
    let (ax, ay) = form.abs_origin(i);
    let usable = form.usable(i) && !blocked;
    let fname = form.name.clone();
    let back = form.back;
    let c = &mut form.ctls[i];
    let r = Rect::from_min_size(origin + Vec2::new(ax, ay) / TWIPS, Vec2::new(c.width, c.height) / TWIPS);
    let id = egui::Id::unique((fname.as_str(), c.name.as_str(), c.index, i));
    let p = ui.painter().clone();
    let font = font_of(&c.font, c.bold, c.size);
    let fore = c.fore.unwrap_or(Color32::BLACK);
    let gray = DARK;
    let mut events: Vec<Ev> = Vec::new();
    let (name, index) = (c.name.clone(), c.index);
    let mut option_clicked = false;
    match c.kind {
        Kind::Form | Kind::Hidden => {}
        Kind::Label => {
            if let Some(b) = c.back {
                if b != back {
                    p.rect_filled(r, CornerRadius::ZERO, b);
                }
            }
            if c.border == 1 {
                p.rect_stroke(r, CornerRadius::ZERO, Stroke::new(1.0, Color32::BLACK), StrokeKind::Inside);
            }
            let tr = if c.border == 1 { r.shrink2(Vec2::new(2.0, 1.0)) } else { r };
            paint_text(&p, tr, &plain(&c.caption), font, fore, c.align, true);
            let resp = ui.interact(r, id, Sense::click());
            if usable && resp.clicked() {
                events.push(Ev::Click(name.clone(), index));
            }
            if usable && resp.double_clicked() {
                events.push(Ev::DblClick(name.clone(), index));
            }
        }
        Kind::CommandButton | Kind::SSCommand => {
            let resp = ui.interact(r, id, Sense::click());
            let pressed = usable && resp.is_pointer_button_down_on();
            button_face(&p, r, pressed, c.default);
            let cap = plain(if c.kind == Kind::SSCommand && c.caption.is_empty() { &c.text } else { &c.caption });
            let off = if pressed { Vec2::splat(1.0) } else { Vec2::ZERO };
            let tr = r.shrink(3.0).translate(off);
            let g = p.fonts_mut(|f| f.layout_job(text_job(&cap, font.clone(), Color32::BLACK, tr.width(), 2)));
            let pos = Pos2::new(tr.center().x, tr.center().y - g.size().y / 2.0);
            if usable {
                p.galley(pos, g, fore);
            } else {
                p.galley_with_override_text_color(pos + Vec2::splat(1.0), g.clone(), Color32::WHITE);
                p.galley_with_override_text_color(pos, g, gray);
            }
            if usable && resp.clicked() {
                events.push(Ev::Click(name.clone(), index));
            }
        }
        Kind::SSRibbon => {
            let resp = ui.interact(r, id, Sense::click());
            button_face(&p, r, c.value != 0 || (usable && resp.is_pointer_button_down_on()), false);
            let g = p.fonts_mut(|f| f.layout_job(text_job(&plain(&c.caption), font.clone(), fore, r.width() - 4.0, 2)));
            p.galley(Pos2::new(r.center().x, r.center().y - g.size().y / 2.0), g, if usable { fore } else { gray });
            if usable && resp.clicked() {
                c.value = 1;
                events.push(Ev::Click(name.clone(), index));
            }
        }
        Kind::CheckBox | Kind::SSCheck => {
            let resp = ui.interact(r, id, Sense::click());
            let bx = if c.align == 1 {
                Rect::from_min_size(Pos2::new(r.right() - 13.0, r.center().y - 6.5), Vec2::splat(13.0))
            } else {
                Rect::from_min_size(Pos2::new(r.left(), r.center().y - 6.5), Vec2::splat(13.0))
            };
            sunken_box(&p, bx, Color32::WHITE);
            if c.value == 1 {
                let s = Stroke::new(1.5, if usable { Color32::BLACK } else { gray });
                p.line_segment([bx.left_top() + Vec2::splat(2.5), bx.right_bottom() - Vec2::splat(2.5)], s);
                p.line_segment([bx.right_top() + Vec2::new(-2.5, 2.5), bx.left_bottom() + Vec2::new(2.5, -2.5)], s);
            }
            let tr = if c.align == 1 { Rect::from_min_max(r.min, Pos2::new(bx.left() - 4.0, r.max.y)) } else { Rect::from_min_max(Pos2::new(bx.right() + 4.0, r.min.y), r.max) };
            let cap = plain(if c.caption.is_empty() { &c.text } else { &c.caption });
            let g = p.fonts_mut(|f| f.layout_job(text_job(&cap, font.clone(), fore, tr.width().max(1.0), 0)));
            p.galley_with_override_text_color(Pos2::new(tr.left(), tr.center().y - g.size().y / 2.0), g, if usable { fore } else { gray });
            if usable && resp.clicked() {
                c.value = if c.value == 1 { 0 } else { 1 };
                events.push(Ev::Click(name.clone(), index));
            }
        }
        Kind::OptionButton => {
            let resp = ui.interact(r, id, Sense::click());
            let ctr = Pos2::new(r.left() + 6.5, r.center().y);
            p.circle_filled(ctr, 6.0, Color32::WHITE);
            p.circle_stroke(ctr, 6.0, Stroke::new(1.0, Color32::BLACK));
            if c.value == 1 {
                p.circle_filled(ctr, 2.5, if usable { Color32::BLACK } else { gray });
            }
            let tr = Rect::from_min_max(Pos2::new(r.left() + 17.0, r.min.y), r.max);
            let g = p.fonts_mut(|f| f.layout_job(text_job(&plain(&c.caption), font.clone(), fore, tr.width().max(1.0), 0)));
            p.galley_with_override_text_color(Pos2::new(tr.left(), tr.center().y - g.size().y / 2.0), g, if usable { fore } else { gray });
            if usable && resp.clicked() {
                option_clicked = true;
            }
        }
        Kind::Frame | Kind::SSFrame => {
            let cap = plain(if c.caption.is_empty() { &c.text } else { &c.caption });
            let fh = font.size;
            let top = if cap.is_empty() { r.top() } else { r.top() + fh / 2.0 };
            let br = Rect::from_min_max(Pos2::new(r.left(), top), r.max);
            // A 3-D frame's BackColor fills it (frmProgrammableFeatures colours tagged options).
            let back = match c.back {
                Some(b) if c.kind == Kind::SSFrame => {
                    p.rect_filled(r, CornerRadius::ZERO, b);
                    b
                }
                _ => back,
            };
            let resp = ui.interact(r, id, Sense::click());
            if usable && resp.clicked() {
                events.push(Ev::Click(name.clone(), index));
            }
            if usable && resp.double_clicked() {
                events.push(Ev::DblClick(name.clone(), index));
            }
            if c.kind == Kind::SSFrame {
                p.rect_stroke(br.translate(Vec2::splat(1.0)), CornerRadius::ZERO, Stroke::new(1.0, Color32::WHITE), StrokeKind::Inside);
                p.rect_stroke(br, CornerRadius::ZERO, Stroke::new(1.0, DARK), StrokeKind::Inside);
            } else {
                p.rect_stroke(br, CornerRadius::ZERO, Stroke::new(1.0, Color32::BLACK), StrokeKind::Inside);
            }
            if !cap.is_empty() {
                let w = text_width(&p, &cap, font.clone());
                let tr = Rect::from_min_size(Pos2::new(r.left() + 7.0, r.top()), Vec2::new(w + 4.0, fh + 2.0));
                p.rect_filled(tr, CornerRadius::ZERO, back);
                p.text(Pos2::new(tr.left() + 2.0, r.top()), Align2::LEFT_TOP, cap, font, if usable { fore } else { gray });
            }
        }
        Kind::SSPanel => {
            p.rect_filled(r, CornerRadius::ZERO, BTN_FACE);
            bevel(&p, r, true, 1.0);
            let inner = r.shrink(3.0);
            bevel(&p, inner, false, 1.0);
            if c.value > 0 {
                let w = inner.width() * (c.value.min(100) as f32 / 100.0);
                p.rect_filled(Rect::from_min_size(inner.min + Vec2::splat(1.0), Vec2::new((w - 2.0).max(0.0), inner.height() - 2.0)), CornerRadius::ZERO, NAVY);
            }
            paint_text(&p, inner, &plain(&c.caption), font, fore, 2, true);
        }
        Kind::TextBox => {
            sunken_box(&p, r, c.back.unwrap_or(Color32::WHITE));
            let mut te = if c.multiline { egui::TextEdit::multiline(&mut c.text) } else { egui::TextEdit::singleline(&mut c.text) };
            te = te.font(font).frame(egui::Frame::NONE).text_color(fore).desired_width(r.width() - 6.0).margin(egui::Margin::symmetric(3, 2)).id(id).password(!c.mask.is_empty());
            if c.max_len > 0 {
                te = te.char_limit(c.max_len);
            }
            if c.align == 2 {
                te = te.align(egui::Align2::CENTER_CENTER);
            }
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r.shrink(1.0)));
            child.set_clip_rect(r.shrink(1.0));
            if !usable {
                child.disable();
            }
            let out = te.show(&mut child);
            if out.response.response.changed() {
                events.push(Ev::Change(name.clone(), index));
            }
            if out.response.response.lost_focus() {
                events.push(Ev::LostFocus(name.clone(), index));
            }
        }
        Kind::MaskEdBox => {
            sunken_box(&p, r, Color32::WHITE);
            if !c.mask.is_empty() && c.text.chars().count() != c.mask.chars().count() {
                c.text = mask_format(&c.text, &c.mask);
            }
            let te = egui::TextEdit::singleline(&mut c.text)
                .font(font)
                .frame(egui::Frame::NONE)
                .text_color(fore)
                .desired_width(r.width() - 6.0)
                .margin(egui::Margin::symmetric(3, 2))
                .id(id);
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r.shrink(1.0)));
            child.set_clip_rect(r.shrink(1.0));
            if !usable {
                child.disable();
            }
            let mut out = te.show(&mut child);
            if out.response.response.changed() && !c.mask.is_empty() {
                // Re-lay the typed characters into the mask; keep the caret after the last typed one.
                let caret: usize = out.cursor_range.map(|cr| cr.primary.index.into()).unwrap_or(0);
                let mask: Vec<char> = c.mask.chars().collect();
                let typed: Vec<char> = c.text.chars().collect();
                // Characters before the caret that are input (not prompts, not literals in place).
                let mut typed_before = 0;
                for (k, ch) in typed.iter().take(caret).enumerate() {
                    let literal_here = mask.get(k).is_some_and(|m| !is_placeholder(*m) && m == ch);
                    if *ch != '_' && !literal_here {
                        typed_before += 1;
                    }
                }
                let formatted = mask_format(&typed.iter().filter(|ch| **ch != '_').collect::<String>(), &c.mask);
                let mut pos = mask.len();
                let mut seen = 0;
                for (k, m) in mask.iter().enumerate() {
                    if is_placeholder(*m) {
                        if seen == typed_before {
                            pos = k;
                            break;
                        }
                        seen += 1;
                    }
                }
                c.text = formatted;
                out.state.cursor.set_char_range(Some(CCursorRange::one(CCursor::new(pos))));
                out.state.store(ui.ctx(), id);
            }
            if out.response.response.changed() {
                events.push(Ev::Change(name.clone(), index));
            }
            if out.response.response.lost_focus() {
                events.push(Ev::LostFocus(name.clone(), index));
            }
        }
        Kind::ListBox | Kind::FileListBox => {
            if c.kind == Kind::FileListBox {
                refresh_files(c);
            }
            sunken_box(&p, r, Color32::WHITE);
            let row_h = font.size + 3.0;
            let inner = r.shrink(1.0);
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner));
            if !usable {
                child.disable();
            }
            child.set_clip_rect(inner);
            let n = c.list.len();
            let w = inner.width();
            egui::ScrollArea::vertical().id_salt(id).auto_shrink([false, false]).max_height(inner.height()).show_rows(&mut child, row_h, n, |ui, range| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                for k in range {
                    let (rr, resp) = ui.allocate_exact_size(Vec2::new((w - 14.0).max(10.0), row_h), Sense::click());
                    let sel = c.list_index == k as i32;
                    if sel {
                        ui.painter().rect_filled(rr, CornerRadius::ZERO, NAVY);
                    }
                    ui.painter().with_clip_rect(rr).text(
                        rr.left_center() + Vec2::new(2.0, 0.0),
                        Align2::LEFT_CENTER,
                        expand_tabs(&c.list[k]),
                        font.clone(),
                        if sel { Color32::WHITE } else { fore },
                    );
                    if resp.clicked() {
                        c.list_index = k as i32;
                        events.push(Ev::Click(name.clone(), index));
                    }
                    if resp.double_clicked() {
                        events.push(Ev::DblClick(name.clone(), index));
                    }
                }
            });
        }
        Kind::ComboBox => {
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r));
            if !usable {
                child.disable();
            }
            let sel = c.selected().cloned().unwrap_or_else(|| c.text.clone());
            let mut pick = None;
            egui::ComboBox::from_id_salt(id).selected_text(egui::RichText::new(sel).font(font.clone())).width(r.width() - 4.0).show_ui(&mut child, |ui| {
                for (k, it) in c.list.iter().enumerate() {
                    if ui.selectable_label(c.list_index == k as i32, egui::RichText::new(it).font(font.clone())).clicked() {
                        pick = Some(k);
                    }
                }
            });
            if let Some(k) = pick {
                c.list_index = k as i32;
                c.text = c.list[k].clone();
                events.push(Ev::Click(name.clone(), index));
            }
        }
        Kind::Grid => {
            sunken_box(&p, r, Color32::WHITE);
            let inner = r.shrink(1.0);
            let row_h = font.size + 5.0;
            let widths: Vec<f32> = c.col_widths.iter().map(|w| w / TWIPS).collect();
            let total_w: f32 = widths.iter().sum::<f32>().max(inner.width() - 14.0);
            let header = c.cells.first().cloned().unwrap_or_default();
            let body_n = c.cells.len().saturating_sub(1);
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner));
            if !usable {
                child.disable();
            }
            child.set_clip_rect(inner);
            egui::ScrollArea::both().id_salt(id).auto_shrink([false, false]).show(&mut child, |ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                let draw_row = |ui: &mut egui::Ui, cells: &[String], fixed: bool, sel: bool| -> egui::Response {
                    let (rr, resp) = ui.allocate_exact_size(Vec2::new(total_w, row_h), Sense::click());
                    let pp = ui.painter();
                    let mut x = rr.left();
                    for (k, w) in widths.iter().enumerate() {
                        let cr = Rect::from_min_size(Pos2::new(x, rr.top()), Vec2::new(*w, row_h));
                        let fill = if fixed { BTN_FACE } else if sel { NAVY } else { Color32::WHITE };
                        pp.rect_filled(cr, CornerRadius::ZERO, fill);
                        pp.rect_stroke(cr, CornerRadius::ZERO, Stroke::new(1.0, DARK), StrokeKind::Inside);
                        if let Some(t) = cells.get(k) {
                            pp.with_clip_rect(cr.shrink(1.0)).text(cr.left_center() + Vec2::new(3.0, 0.0), Align2::LEFT_CENTER, t, font.clone(), if sel && !fixed { Color32::WHITE } else { Color32::BLACK });
                        }
                        x += w;
                    }
                    resp
                };
                draw_row(ui, &header, true, false);
                for k in 0..body_n {
                    let row = k + 1;
                    let cells = c.cells[row].clone();
                    let resp = draw_row(ui, &cells, false, c.row == row as i32);
                    if resp.clicked() {
                        c.row = row as i32;
                        events.push(Ev::Click(name.clone(), index));
                    }
                    if resp.double_clicked() {
                        events.push(Ev::DblClick(name.clone(), index));
                    }
                }
            });
        }
        Kind::Gauge => {
            sunken_box(&p, r, Color32::WHITE);
            let f = (c.value.max(0) as f32 / c.max.max(1) as f32).min(1.0);
            let fill = Rect::from_min_size(r.min + Vec2::splat(1.0), Vec2::new((r.width() - 2.0) * f, r.height() - 2.0));
            p.rect_filled(fill, CornerRadius::ZERO, NAVY);
            p.text(r.center(), Align2::CENTER_CENTER, format!("{}%", (f * 100.0).round()), ui_font_bold(), if f > 0.5 { Color32::WHITE } else { Color32::BLACK });
        }
        Kind::SpinButton => {
            let up = Rect::from_min_max(r.min, Pos2::new(r.right(), r.center().y));
            let dn = Rect::from_min_max(Pos2::new(r.left(), r.center().y), r.max);
            for (k, rr) in [up, dn].into_iter().enumerate() {
                let resp = ui.interact(rr, id.with(k), Sense::click());
                button_face(&p, rr, usable && resp.is_pointer_button_down_on(), false);
                let ctr = rr.center();
                let pts = if k == 0 {
                    vec![ctr + Vec2::new(-3.0, 1.5), ctr + Vec2::new(3.0, 1.5), ctr + Vec2::new(0.0, -1.5)]
                } else {
                    vec![ctr + Vec2::new(-3.0, -1.5), ctr + Vec2::new(3.0, -1.5), ctr + Vec2::new(0.0, 1.5)]
                };
                p.add(egui::Shape::convex_polygon(pts, Color32::BLACK, Stroke::NONE));
                if usable && resp.clicked() {
                    events.push(if k == 0 { Ev::SpinUp(name.clone(), index) } else { Ev::SpinDown(name.clone(), index) });
                }
            }
        }
        Kind::Graph => draw_graph(&p, r, c),
        Kind::Image => {
            let resp = ui.interact(r, id, Sense::click());
            if usable && resp.clicked() {
                events.push(Ev::Click(name.clone(), index));
            }
        }
        Kind::Shape => {
            p.rect_stroke(r, CornerRadius::ZERO, Stroke::new(1.0, Color32::BLACK), StrokeKind::Inside);
        }
    }
    if option_clicked {
        let parent = form.ctls[i].parent;
        let was = form.ctls[i].value;
        for k in 0..form.ctls.len() {
            if form.ctls[k].kind == Kind::OptionButton && form.ctls[k].parent == parent {
                form.ctls[k].value = 0;
            }
        }
        form.ctls[i].value = 1;
        if was != 1 {
            events.push(Ev::Click(name, index));
        }
    }
    form.events.extend(events);
}


fn expand_tabs(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if ch == '\t' {
            let n = 8 - out.chars().count() % 8;
            out.extend(std::iter::repeat_n(' ', n));
        } else {
            out.push(ch);
        }
    }
    out
}

fn draw_graph(p: &egui::Painter, r: Rect, c: &Ctl) {
    p.rect_filled(r, CornerRadius::ZERO, Color32::WHITE);
    p.rect_stroke(r, CornerRadius::ZERO, Stroke::new(1.0, Color32::BLACK), StrokeKind::Inside);
    p.text(Pos2::new(r.center().x, r.top() + 4.0), Align2::CENTER_TOP, plain(&c.caption), ui_font_bold(), Color32::BLACK);
    if c.data.is_empty() {
        return;
    }
    let plot = Rect::from_min_max(r.min + Vec2::new(56.0, 26.0), r.max - Vec2::new(12.0, 26.0));
    let maxv = c.data.iter().cloned().fold(0.0f32, f32::max).max(0.01);
    p.line_segment([plot.left_bottom(), plot.right_bottom()], Stroke::new(1.0, Color32::BLACK));
    p.line_segment([plot.left_top(), plot.left_bottom()], Stroke::new(1.0, Color32::BLACK));
    for k in 0..=4 {
        let v = maxv * k as f32 / 4.0;
        let y = plot.bottom() - plot.height() * k as f32 / 4.0;
        p.text(Pos2::new(plot.left() - 4.0, y), Align2::RIGHT_CENTER, format!("{v:.2}"), ui_font(), Color32::BLACK);
    }
    let n = c.data.len();
    let step = plot.width() / n as f32;
    let colors = [Color32::from_rgb(0, 0, 255), Color32::from_rgb(255, 0, 0), Color32::from_rgb(0, 128, 0), Color32::from_rgb(255, 0, 255), Color32::from_rgb(0, 128, 128)];
    let mut prev: Option<Pos2> = None;
    for (k, v) in c.data.iter().enumerate() {
        let h = plot.height() * (v / maxv);
        let x = plot.left() + step * k as f32;
        if c.graph_type == 1 {
            let pt = Pos2::new(x + step / 2.0, plot.bottom() - h);
            if let Some(pp) = prev {
                p.line_segment([pp, pt], Stroke::new(2.0, colors[0]));
            }
            p.circle_filled(pt, 2.5, colors[0]);
            prev = Some(pt);
        } else {
            let br = Rect::from_min_max(Pos2::new(x + step * 0.15, plot.bottom() - h), Pos2::new(x + step * 0.85, plot.bottom()));
            p.rect_filled(br, CornerRadius::ZERO, colors[k % colors.len()]);
            p.rect_stroke(br, CornerRadius::ZERO, Stroke::new(1.0, Color32::BLACK), StrokeKind::Inside);
        }
        if let Some(l) = c.labels.get(k) {
            if n <= 16 || k % (n / 12 + 1) == 0 {
                p.text(Pos2::new(x + step / 2.0, plot.bottom() + 3.0), Align2::CENTER_TOP, l, ui_font(), Color32::BLACK);
            }
        }
    }
}

/// Fill a FileListBox from its folder and pattern (like VB's FileListBox: names only, sorted).
pub fn refresh_files(c: &mut Ctl) {
    if c.path.is_empty() || c.scanned.is_some_and(|t| t.elapsed().as_millis() < 1000) {
        return;
    }
    c.scanned = Some(std::time::Instant::now());
    let names = list_files(&c.path, &c.pattern);
    if names != c.list {
        let keep = c.selected().cloned();
        c.list = names;
        c.list_index = keep.and_then(|k| c.list.iter().position(|x| *x == k)).map(|x| x as i32).unwrap_or(-1);
    }
}

pub fn list_files(dir: &str, pattern: &str) -> Vec<String> {
    let pats: Vec<String> = pattern.split(';').map(|s| s.trim().to_ascii_lowercase()).collect();
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|rd| {
            rd.flatten()
                .filter(|e| e.path().is_file())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .filter(|n| pats.iter().any(|p| wildcard(p, &n.to_ascii_lowercase())))
                .collect()
        })
        .unwrap_or_default();
    names.sort_by_key(|n| n.to_ascii_lowercase());
    names
}

pub fn wildcard(p: &str, s: &str) -> bool {
    fn go(p: &[char], s: &[char]) -> bool {
        match (p.first(), s.first()) {
            (None, None) => true,
            (Some('*'), _) => go(&p[1..], s) || (!s.is_empty() && go(p, &s[1..])),
            (Some('?'), Some(_)) => go(&p[1..], &s[1..]),
            (Some(a), Some(b)) => a == b && go(&p[1..], &s[1..]),
            _ => false,
        }
    }
    let p: Vec<char> = if p == "*.*" { vec!['*'] } else { p.chars().collect() };
    go(&p, &s.chars().collect::<Vec<_>>())
}

// ---------------------------------------------------------------------------------------------
// MsgBox

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    Ok = 1,
    Cancel = 2,
    Abort = 3,
    Retry = 4,
    Ignore = 5,
    Yes = 6,
    No = 7,
}

#[derive(Clone, Debug)]
pub struct MsgBox {
    pub id: u64,
    pub text: String,
    pub title: String,
    /// VB MsgBox type: buttons (0 OK, 1 OK/Cancel, 2 Abort/Retry/Ignore, 3 Yes/No/Cancel, 4 Yes/No,
    /// 5 Retry/Cancel) + icon (16 stop, 32 question, 48 exclamation, 64 information).
    pub flags: i32,
    /// Which form asked and what for, so the answer reaches the right code.
    pub owner: String,
    pub tag: String,
}

impl MsgBox {
    pub fn buttons(&self) -> Vec<Answer> {
        match self.flags & 7 {
            1 => vec![Answer::Ok, Answer::Cancel],
            2 => vec![Answer::Abort, Answer::Retry, Answer::Ignore],
            3 => vec![Answer::Yes, Answer::No, Answer::Cancel],
            4 => vec![Answer::Yes, Answer::No],
            5 => vec![Answer::Retry, Answer::Cancel],
            _ => vec![Answer::Ok],
        }
    }
    pub fn size_px(&self, ctx: &egui::Context) -> Vec2 {
        let g = ctx.fonts_mut(|f| f.layout_job(text_job(&self.text, ui_font(), Color32::BLACK, 380.0, 0)));
        let icon = if self.flags & 0x70 != 0 { 48.0 } else { 0.0 };
        let w = (g.size().x + icon + 44.0).max(self.buttons().len() as f32 * 84.0 + 24.0).max(170.0);
        let h = g.size().y.max(if icon > 0.0 { 34.0 } else { 0.0 }) + 76.0;
        Vec2::new(w, h)
    }
    /// Draw the box; returns the button pressed.
    pub fn ui(&self, ui: &mut egui::Ui) -> Option<Answer> {
        let full = ui.max_rect();
        let p = ui.painter().clone();
        p.rect_filled(full, CornerRadius::ZERO, Color32::WHITE);
        let mut x = full.left() + 18.0;
        let icon = self.flags & 0x70;
        if icon != 0 {
            let c = Pos2::new(x + 16.0, full.top() + 32.0);
            let big = |s: f32| FontId::new(s, FontFamily::Name("sansb".into()));
            match icon {
                16 => {
                    p.circle_filled(c, 15.0, Color32::from_rgb(255, 0, 0));
                    p.line_segment([c + Vec2::new(-6.0, -6.0), c + Vec2::new(6.0, 6.0)], Stroke::new(3.0, Color32::WHITE));
                    p.line_segment([c + Vec2::new(6.0, -6.0), c + Vec2::new(-6.0, 6.0)], Stroke::new(3.0, Color32::WHITE));
                }
                32 => {
                    p.circle_filled(c, 15.0, Color32::from_rgb(0, 0, 255));
                    p.text(c, Align2::CENTER_CENTER, "?", big(20.0), Color32::WHITE);
                }
                48 => {
                    p.add(egui::Shape::convex_polygon(vec![c + Vec2::new(0.0, -15.0), c + Vec2::new(15.0, 13.0), c + Vec2::new(-15.0, 13.0)], Color32::from_rgb(255, 255, 0), Stroke::new(1.0, Color32::BLACK)));
                    p.text(c + Vec2::new(0.0, 3.0), Align2::CENTER_CENTER, "!", big(18.0), Color32::BLACK);
                }
                _ => {
                    p.circle_filled(c, 15.0, Color32::from_rgb(0, 0, 255));
                    p.text(c, Align2::CENTER_CENTER, "i", big(20.0), Color32::WHITE);
                }
            }
            x += 48.0;
        }
        let tr = Rect::from_min_max(Pos2::new(x, full.top() + 18.0), Pos2::new(full.right() - 14.0, full.bottom() - 44.0));
        paint_text(&p, tr, &self.text, ui_font(), Color32::BLACK, 0, true);
        let bar = Rect::from_min_max(Pos2::new(full.left(), full.bottom() - 42.0), full.max);
        p.rect_filled(bar, CornerRadius::ZERO, Color32::from_rgb(240, 240, 240));
        let btns = self.buttons();
        let bw = 75.0;
        let total = btns.len() as f32 * (bw + 8.0) - 8.0;
        let mut bx = full.center().x - total / 2.0;
        let mut out = None;
        for (k, b) in btns.iter().enumerate() {
            let r = Rect::from_min_size(Pos2::new(bx, bar.top() + 9.0), Vec2::new(bw, 24.0));
            let resp = ui.interact(r, egui::Id::unique(("msgbox", self.id, k)), Sense::click());
            button_face(&p, r, resp.is_pointer_button_down_on(), k == 0);
            let label = match b {
                Answer::Ok => "OK",
                Answer::Cancel => "Cancel",
                Answer::Abort => "Abort",
                Answer::Retry => "Retry",
                Answer::Ignore => "Ignore",
                Answer::Yes => "Yes",
                Answer::No => "No",
            };
            p.text(r.center(), Align2::CENTER_CENTER, label, ui_font(), Color32::BLACK);
            if resp.clicked() {
                out = Some(*b);
            }
            bx += bw + 8.0;
        }
        if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            out = Some(btns[0]);
        }
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            out = Some(*btns.last().unwrap());
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn masks() {
        assert_eq!(mask_format("5550101234", "###-###-####"), "555-010-1234");
        assert_eq!(mask_format("555", "###-###-####"), "555-___-____");
        assert_eq!(mask_format("555-010", "###-###-####"), "555-010-____");
        let mut c = Ctl::new(Kind::MaskEdBox, "x");
        c.mask = "&-&&&-&&&-&&&&".into();
        c.set_masked("15550101234");
        assert_eq!(c.text, "1-555-010-1234");
        assert_eq!(c.clip_text(), "15550101234");
    }
    #[test]
    fn parses_frm() {
        let f = Form::parse(include_str!("../forms/SPEEDIAL.FRM"), include_str!("../forms/vbx.txt"));
        assert_eq!(f.name, "frmSpeedDial");
        assert_eq!(f.geti("MaskedEdit1", 0).top, 330.0);
        assert_eq!(f.get("Label1").caption, "Speed Dial 0");
    }
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//! TFW (TeleLink for Windows) rebuilt in Rust: the VB3 project in C:\TFWSOURCE, form by form.
//!
//! `--screenshot <file.png>` starts as usual, draws about 120 frames, saves the main window's
//! picture and exits (without touching the modem).

mod app;
mod calltest;
mod docs;
mod fea;
mod files;
mod forms;
mod ini;
mod modem;
mod proto;
mod sip;
mod text;
mod vb;

use eframe::egui;
use std::path::PathBuf;

fn main() -> eframe::Result {
    let mut screenshot = None;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        if a == "--screenshot" {
            screenshot = args.next().map(PathBuf::from);
        } else if a == "--calltest" {
            let rest: Vec<String> = args.by_ref().collect();
            std::process::exit(calltest::run(&rest));
        } else if a == "--selftest" {
            std::process::exit(app::App::selftest());
        }
    }
    // The main form: ClientWidth 8850 x ClientHeight 4860 twips, plus its menu bar; a fixed
    // double border and no control box (TELELINK.FRM).
    let size = egui::vec2(8850.0 / vb::TWIPS, 4860.0 / vb::TWIPS + vb::MENU_H);
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Telelink")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_max_inner_size(size)
        .with_resizable(false)
        .with_maximize_button(false)
        .with_position(egui::pos2(75.0 / vb::TWIPS, 2115.0 / vb::TWIPS));
    if screenshot.is_some() {
        viewport = viewport.with_active(false);
        #[cfg(windows)]
        {
            viewport = viewport.with_window_level(egui::WindowLevel::AlwaysOnBottom);
        }
    }
    #[allow(unused_mut)]
    let mut options = eframe::NativeOptions { viewport, run_and_return: false, ..Default::default() };
    #[cfg(target_os = "macos")]
    if screenshot.is_some() {
        options.event_loop_builder = Some(Box::new(|b| {
            use winit::platform::macos::EventLoopBuilderExtMacOS;
            b.with_activate_ignoring_other_apps(false);
        }));
    }
    eframe::run_native("TFW", options, Box::new(move |cc| Ok(Box::new(app::App::new(cc, screenshot)))))
}

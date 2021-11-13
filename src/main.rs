#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = eframe_template::TemplateApp::default();
    let native_options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_size: Some(eframe::egui::Vec2::new(1000.0, 400.0)),
        resizable: true,
        transparent: false,
    };
    eframe::run_native(Box::new(app), native_options);
}

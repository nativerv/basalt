//! Demo app for egui

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

const APP_TITLE: &str = "Basalt";

use basalt::BasaltApp;

// When compiling natively:
fn main() -> Result<(), eframe::Error> {
  {
    // Silence wgpu log spam (https://github.com/gfx-rs/wgpu/issues/3206)
    let mut rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| String::from("info"));
    for loud_crate in ["naga", "wgpu_core", "wgpu_hal"] {
      if !rust_log.contains(&format!("{loud_crate}=")) {
        rust_log += &format!(",{loud_crate}=warn");
      }
    }
    std::env::set_var("RUST_LOG", rust_log);
  }

  env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

  let options = eframe::NativeOptions {
    drag_and_drop_support: true,
    initial_window_size: Some([1280.0, 1024.0].into()),

    #[cfg(feature = "wgpu")]
    renderer: eframe::Renderer::Wgpu,

    ..Default::default()
  };
  eframe::run_native(
    APP_TITLE,
    options,
    // TODO: look at what you can do with the `creation_context`
    Box::new(|_creation_context| Box::<BasaltApp>::default()),
  )
}

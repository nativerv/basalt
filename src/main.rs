// Hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Lints
#![warn(clippy::nursery)]

const APP_TITLE: &str = "Basalt";

use basalt::BasaltApp;

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

  // Actual entry point of the app
  eframe::run_native(
    APP_TITLE,
    options,
    // TODO: look at what you can do with the `CreationContext`
    // NOTE: one of the things that you can do is to implement
    // and assign set custom `Storage`:
    // https://discord.com/channels/900275882684477440/904461220592119849/1128120151137583155
    Box::new(|_creation_context| Box::<BasaltApp>::new(BasaltApp::new())),
  )
}

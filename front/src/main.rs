#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
  env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

  let native_options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_inner_size([400.0, 300.0])
      .with_min_inner_size([300.0, 220.0]),
    ..Default::default()
  };
  eframe::run_native(
    "Toys!",
    native_options,
    Box::new(|cc| Box::new(front::ToyApp::new(cc))),
  )
}

#[cfg(target_arch = "wasm32")]
fn main() {
  eframe::WebLogger::init(log::LevelFilter::Debug).ok();

  let web_options = eframe::WebOptions::default();

  wasm_bindgen_futures::spawn_local(async {
    eframe::WebRunner::new()
      .start(
        "toys", // hardcode it
        web_options,
        Box::new(|cc| Box::new(front::ToyApp::new(cc))),
      )
      .await
      .expect("failed to start eframe");
  });
}

use crate::multi_metadata::MultiMetadataWindow;
use crate::single_metadata::SingleMetadataWindow;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ToyApp {
  single_metadata: SingleMetadataWindow,
  multi_metadata: MultiMetadataWindow,

  settings: bool,
}

impl Default for ToyApp {
  fn default() -> Self {
    Self {
      // Example stuff:
      single_metadata: SingleMetadataWindow::default(),
      multi_metadata: MultiMetadataWindow::default(),
      settings: false,
    }
  }
}

impl ToyApp {
  /// Called once before the first frame.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    // This is also where you can customize the look and feel of egui using
    // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

    setup_custom_fonts(&cc.egui_ctx);

    // Load previous app state (if any).
    // Note that you must enable the `persistence` feature for this to work.
    if let Some(storage) = cc.storage {
      return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
    }

    Default::default()
  }
}

impl eframe::App for ToyApp {
  /// Called by the frame work to save state before shutdown.
  fn save(&mut self, storage: &mut dyn eframe::Storage) {
    eframe::set_value(storage, eframe::APP_KEY, self);
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
    // For inspiration and more examples, go to https://emilk.github.io/egui

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
      // The top panel is often a good place for a menu bar:

      egui::menu::bar(ui, |ui| {
        // NOTE: no File->Quit on web pages!
        let is_web = cfg!(target_arch = "wasm32");
        if !is_web {
          ui.menu_button("File", |ui| {
            if ui.button("Quit").clicked() {
              ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
          });
          ui.add_space(16.0);
        }

        egui::widgets::global_dark_light_mode_buttons(ui);
      });
    });

    egui::SidePanel::left("item_list")
      .resizable(true)
      .default_width(100.0)
      .show(ctx, |ui| {
        ui.heading("Items");

        ui.separator();

        ui.collapsing("Web3", |ui| {
          // ui.checkbox(&mut self.metadata, "Create metadata");
          ui.collapsing("메타데이터 만들기", |ui| {
            self.single_metadata.show(ui);
            self.multi_metadata.show(ui);
          });
        });

        ui.separator();

        ui.checkbox(&mut self.settings, "🔧 Settings");
      });

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        egui::warn_if_debug_build(ui);
      });
    });

    self.single_metadata.update(ctx, _frame);
    self.multi_metadata.update(ctx, _frame);

    egui::Window::new("🔧 Settings")
      .open(&mut self.settings)
      .vscroll(true)
      .show(ctx, |ui| {
        ctx.settings_ui(ui);
      });
  }
}

fn setup_custom_fonts(ctx: &egui::Context) {
  let mut fonts = egui::FontDefinitions::default();

  fonts.font_data.insert(
    "font".to_owned(),
    egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
  );

  fonts
    .families
    .entry(egui::FontFamily::Proportional)
    .or_default()
    .insert(0, "font".to_owned());

  fonts
    .families
    .entry(egui::FontFamily::Monospace)
    .or_default()
    .push("font".to_owned());

  ctx.set_fonts(fonts);
}

use crate::chain_settings::ChainSettingsWindow;
use crate::contract_erc721::ERC721ContractWindow;
// use crate::contract_erc721::ERC721ContractWindow;
use crate::metadata::SingleMetadataWindow;
use crate::wallet_balance::WalletBalanceWindow;
use crate::wallet_settings::WalletSettingsWindow;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ToyApp {
  metadata: SingleMetadataWindow,
  wallet_balance: WalletBalanceWindow,

  wallet_settings: WalletSettingsWindow,
  chain_settings: ChainSettingsWindow,

  erc20_contract: ERC721ContractWindow,

  settings: bool,
}

impl Default for ToyApp {
  fn default() -> Self {
    Self {
      metadata: SingleMetadataWindow::default(),
      wallet_balance: WalletBalanceWindow::default(),

      wallet_settings: WalletSettingsWindow::default(),
      chain_settings: ChainSettingsWindow::default(),

      erc20_contract: ERC721ContractWindow::default(),

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
            self.metadata.show(ui);
          });

          ui.collapsing("지갑", |ui| {
            self.wallet_balance.show(ui);
            self.wallet_settings.show(ui);
          });

          ui.collapsing("컨트랙트 콜", |ui| {
            self.erc20_contract.show(ui);
            // self.erc721_contract.show(ui);
          });

          self.chain_settings.show(ui);
        });

        ui.separator();

        ui.checkbox(&mut self.settings, "🔧 Settings");
      });

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        egui::warn_if_debug_build(ui);
      });
    });

    self.metadata.update(ctx, _frame);
    self.wallet_settings.update(ctx, _frame);
    self.chain_settings.update(ctx, _frame);
    self.wallet_balance.update(
      ctx,
      _frame,
      &self.chain_settings,
      &self.wallet_settings,
    );
    self
      .erc20_contract
      .update(ctx, _frame, &mut self.chain_settings);
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

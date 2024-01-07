use std::collections::BTreeMap;

use ethers::types::U256;
use poll_promise::Promise;

use crate::chain_settings::ChainSettingsWindow;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WalletBalanceWindow {
  pub is_open: bool,

  #[serde(skip)]
  pub balances: Option<Promise<BTreeMap<String, BTreeMap<String, String>>>>,
}

impl Default for WalletBalanceWindow {
  fn default() -> Self {
    Self {
      is_open: false,
      balances: None,
    }
  }
}

impl WalletBalanceWindow {
  pub fn show(&mut self, ui: &mut egui::Ui) {
    ui.checkbox(&mut self.is_open, "지갑 잔액");
  }

  pub fn update(
    &mut self,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
    chain_settings: &ChainSettingsWindow,
  ) {
    egui::Window::new("지갑 잔액 조회")
      .open(&mut self.is_open)
      .vscroll(true)
      .show(ctx, |ui| {
        if ui.button("조회").clicked() {
          self.balances = Some(chain_settings.get_balance());
        }
        egui::Grid::new("coin_balance_grid")
          .striped(true)
          .min_col_width(50.0)
          .show(ui, |ui| {
            ui.label("address");
            for s in chain_settings.chain_settings.iter() {
              ui.label(&s.chain_name);
            }
            ui.end_row();

            if let Some(s) = &self.balances {
              if let Some(r) = s.ready() {
                for (address, balance) in r.iter() {
                  ui.label(address);
                  for (_, balance) in balance.iter() {
                    ui.label(balance);
                  }
                  ui.end_row();
                }
              } else {
                ui.add(egui::widgets::ProgressBar::new(0.5));
              }

              ui.end_row();
            }
          });
      });
  }
}

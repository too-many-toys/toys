use ethers::{
  prelude::{Http, Provider},
  types::Address,
};
use poll_promise::Promise;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WalletSettingsWindow {
  pub is_open: bool,

  pub wallet_addresses: Vec<WalletInfo>,
  pub add_wallet_name: String,
  pub add_wallet_address: String,
}

impl Default for WalletSettingsWindow {
  fn default() -> Self {
    Self {
      is_open: false,
      wallet_addresses: vec![],
      add_wallet_name: "".to_string(),
      add_wallet_address: "".to_string(),
    }
  }
}

impl WalletSettingsWindow {
  pub fn show(&mut self, ui: &mut egui::Ui) {
    ui.checkbox(&mut self.is_open, "지갑 설정");
  }

  pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::Window::new("지갑 설정")
      .open(&mut self.is_open)
      .vscroll(true)
      .show(ctx, |ui| {
        ui.collapsing("지갑 설정", |ui| {
          ui.horizontal(|ui| {
            ui.label("지갑 이름");
            ui.text_edit_singleline(&mut self.add_wallet_name);
            ui.label("지갑 주소");
            ui.text_edit_singleline(&mut self.add_wallet_address);
            if ui.button("지갑 주소 추가").clicked() {
              let wallet_info = WalletInfo {
                name: self.add_wallet_name.clone(),
                address: self.add_wallet_address.clone(),
              };
              self.wallet_addresses.push(wallet_info);

              self.add_wallet_address = "".to_string();
              self.add_wallet_name = "".to_string();
            }
          });

          for i in 0..self.wallet_addresses.len() {
            let res = ui.horizontal(|ui| {
              let font_color = if let Ok(_) =
                self.wallet_addresses[i].address.clone().parse::<Address>()
              {
                egui::Color32::from_rgb(255, 255, 255)
              } else {
                egui::Color32::from_rgb(255, 0, 0)
              };

              ui.colored_label(
                font_color,
                format!(
                  "{}: {}",
                  self.wallet_addresses[i].name,
                  self.wallet_addresses[i].address
                ),
              );

              if ui.button("삭제").clicked() {
                self.wallet_addresses.remove(i);
                false
              } else {
                true
              }
            });

            if res.inner == false {
              break;
            }
          }
        });
      });
  }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ChainSettings {
  pub chain_name: String,
  pub rpc_url: String,
  #[serde(skip)]
  pub provider: Option<Promise<Provider<Http>>>,
}

impl Default for ChainSettings {
  fn default() -> Self {
    Self {
      chain_name: "".to_string(),
      rpc_url: "".to_string(),
      provider: None,
    }
  }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct WalletInfo {
  pub name: String,
  pub address: String,
}

impl Default for WalletInfo {
  fn default() -> Self {
    Self {
      name: "".to_string(),
      address: "".to_string(),
    }
  }
}

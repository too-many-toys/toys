use std::collections::BTreeMap;

use ethers::{
  prelude::{Http, Provider},
  providers::Middleware,
  types::{Address, U256},
};
use poll_promise::Promise;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ChainSettingsWindow {
  pub is_open: bool,

  pub wallet_addresses: Vec<WalletInfo>,
  pub add_wallet_name: String,
  pub add_wallet_address: String,

  pub chain_settings: Vec<ChainSettings>,
  pub add_chain_settings: ChainSettings,
}

impl Default for ChainSettingsWindow {
  fn default() -> Self {
    Self {
      is_open: false,
      wallet_addresses: vec![],
      add_wallet_name: "".to_string(),
      add_wallet_address: "".to_string(),

      chain_settings: vec![],
      add_chain_settings: ChainSettings::default(),
    }
  }
}

impl ChainSettingsWindow {
  pub fn show(&mut self, ui: &mut egui::Ui) {
    ui.checkbox(&mut self.is_open, "체인 설정");
  }

  pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::Window::new("지갑, 체인 설정")
      .open(&mut self.is_open)
      .vscroll(true)
      .show(ctx, |ui| {
        ui.collapsing("체인 설정", |ui| {
          ui.horizontal(|ui| {
            ui.label("체인 이름");
            ui.text_edit_singleline(&mut self.add_chain_settings.chain_name);
            ui.label("체인 RPC URL");
            ui.text_edit_singleline(&mut self.add_chain_settings.rpc_url);
          });

          if ui.button("체인 RPC URL 추가").clicked() {
            let setting = ChainSettings {
              chain_name: self.add_chain_settings.chain_name.clone(),
              rpc_url: self.add_chain_settings.rpc_url.clone(),
            };
            self.chain_settings.push(setting);
            self.add_chain_settings = ChainSettings::default();
          }

          for i in 0..self.chain_settings.len() {
            let res = ui.horizontal(|ui| {
              ui.colored_label(
                egui::Color32::from_rgb(255, 255, 255),
                format!("체인 이름: {}", self.chain_settings[i].chain_name),
              );
              ui.colored_label(
                egui::Color32::from_rgb(255, 255, 255),
                format!("체인 RPC URL: {}", self.chain_settings[i].rpc_url),
              );

              if ui.button("삭제").clicked() {
                self.chain_settings.remove(i);
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

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct ChainSettings {
  pub chain_name: String,
  pub rpc_url: String,
}

impl Default for ChainSettings {
  fn default() -> Self {
    Self {
      chain_name: "".to_string(),
      rpc_url: "".to_string(),
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
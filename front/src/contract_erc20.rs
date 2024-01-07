use crate::contract_abis::ierc20::IERC20;
use ethers::types::Address;
use poll_promise::Promise;

use crate::chain_settings::ChainSettingsWindow;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ERC20ContractWindow {
  pub is_open: bool,

  pub contract_name: String,
  pub contract_address: String,

  pub selected_chain: String,
  pub selected: usize,

  pub contract_name_color: egui::Color32,

  #[serde(skip)]
  contracts: Vec<ERC20Contract>,
}

impl Default for ERC20ContractWindow {
  fn default() -> Self {
    Self {
      is_open: false,
      contracts: vec![],
      contract_name: "".to_string(),
      contract_address: "".to_string(),

      contract_name_color: egui::Color32::from_rgb(140, 140, 140),

      selected_chain: "".to_string(),
      selected: 0,
    }
  }
}

impl ERC20ContractWindow {
  pub fn show(&mut self, ui: &mut egui::Ui) {
    ui.checkbox(&mut self.is_open, "ERC20");
  }

  pub fn update(
    &mut self,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
    chain_settings: &mut ChainSettingsWindow,
  ) {
    egui::Window::new("ERC20 컨트랙트")
      .open(&mut self.is_open)
      .vscroll(true)
      .show(ctx, |ui| {
        ui.collapsing("컨트랙트 추가", |ui| {
          ui.horizontal(|ui| {
            ui.label("컨트랙트 이름");
            ui.text_edit_singleline(&mut self.contract_name);
          });
          ui.horizontal(|ui| {
            ui.colored_label(self.contract_name_color, "컨트랙트 주소");
            ui.text_edit_singleline(&mut self.contract_address);
          });

          ui.horizontal(|ui| {
            egui::ComboBox::from_label("체인 선택")
              .selected_text(
                &chain_settings.chain_settings[self.selected].chain_name,
              )
              .show_ui(ui, |ui| {
                for i in 0..chain_settings.chain_settings.len() {
                  let chain_name =
                    chain_settings.chain_settings[i].chain_name.clone();
                  let value = ui.selectable_value(
                    &mut chain_settings.chain_settings[i].chain_name,
                    chain_name.clone(),
                    chain_name,
                  );

                  if value.clicked() {
                    self.selected = i;
                  }
                }
              });
          });

          if ui.button("컨트랙트 추가").clicked() {
            if let Err(_) = self.contract_address.parse::<Address>() {
              self.contract_name_color = egui::Color32::from_rgb(255, 0, 0);
              return;
            } else {
              self.contract_name_color = egui::Color32::from_rgb(140, 140, 140)
            }

            self.contracts.push(ERC20Contract {
              name: self.contract_name.clone(),
              address: self.contract_address.clone(),
            });

            self.selected = 0;
            self.contract_name = "".to_string();
            self.contract_address = "".to_string();
          }
        });

        for i in 0..self.contracts.len() {
          ui.horizontal(|ui| {
            ui.collapsing(&self.contracts[i].name, |ui| {
              ui.horizontal(|ui| {
                ui.label("totalSupply");
                if ui.button("호출").clicked() {}
              });
            });
            if ui.button("컨트랙트 삭제").clicked() {
              self.contracts.remove(i);
            }
          });
        }
      });
  }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ERC20Contract {
  pub name: String,
  pub address: String,
}

impl Default for ERC20Contract {
  fn default() -> Self {
    Self {
      name: "".to_string(),
      address: "".to_string(),
    }
  }
}

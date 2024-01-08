use std::collections::BTreeMap;

use ethers::{
  providers::{Http, Middleware, Provider},
  types::{Address, U256},
};
use poll_promise::Promise;

use crate::{
  chain_settings::{self, ChainSettings, ChainSettingsWindow},
  wallet_settings::{self, WalletSettingsWindow},
};

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
    wallet_settings: &WalletSettingsWindow,
  ) {
    egui::Window::new("지갑 잔액 조회")
      .open(&mut self.is_open)
      .vscroll(true)
      .show(ctx, |ui| {
        if ui.button("조회").clicked() {
          let chain_settings = chain_settings.chain_settings.clone();
          let wallet_settings = wallet_settings.wallet_addresses.clone();

          self.get_balance(
            chain_settings.chain_settings,
            wallet_settings.wallet_addresses,
          );
        }
        egui::Grid::new("coin_balance_grid")
          .striped(true)
          .min_col_width(50.0)
          .show(ui, |ui| {
            ui.label("지갑 이름");
            if let Some(s) = &self.balances {
              if let Some(r) = s.ready() {
                for (_, balance) in r.iter() {
                  for (chain_name, _) in balance.iter() {
                    ui.label(chain_name);
                  }
                  ui.end_row();
                  break;
                }
              } else {
                ui.add(egui::widgets::ProgressBar::new(0.5));
              }
            }

            if let Some(s) = &self.balances {
              if let Some(r) = s.ready() {
                for (address, balance) in r.iter() {
                  let address_name =
                    address.split(":").collect::<Vec<&str>>()[0];
                  ui.label(address_name);

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

impl WalletBalanceWindow {
  pub fn get_balance(
    &mut self,
    chain_settings: Vec<chain_settings::ChainSettings>,
    wallet_settings: Vec<wallet_settings::WalletInfo>,
  ) {
    // TODO: 올바른 URL이 아닐 때 처리
    let wallet_infos = wallet_settings.clone();
    let chain_settings = chain_settings.clone();
    let mut providers: Vec<(Provider<Http>, String)> = Vec::new();

    for chain_info in chain_settings.iter() {
      let provider =
        Provider::<Http>::try_from(chain_info.rpc_url.clone()).unwrap();
      providers.push((provider, chain_info.chain_name.clone()));
    }

    let promise = Promise::spawn_local(async move {
      let mut balances = BTreeMap::new();

      for wallet_info in wallet_infos.iter() {
        let wallet_key = format!(
          "{}:{}",
          wallet_info.name.clone(),
          wallet_info.address.clone(),
        );
        balances.insert(wallet_key, BTreeMap::new());

        for (provider, chain_name) in providers.iter() {
          let wallet_key = format!(
            "{}:{}",
            wallet_info.name.clone(),
            wallet_info.address.clone(),
          );

          let address = if let Ok(r) = wallet_info.address.parse::<Address>() {
            r
          } else {
            continue;
          };

          let balance = provider.get_balance(address, None).await;

          balances.get_mut(&wallet_key).unwrap().insert(
            chain_name.clone(),
            match balance {
              Ok(balance) => ethers::utils::format_ether(balance),
              Err(e) => {
                log::info!("get_balance error: {:?}", e);
                ethers::utils::format_ether(U256::zero())
              }
            },
          );
        }
      }

      balances
    });

    self.balances = Some(promise);
  }
}

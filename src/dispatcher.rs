use yansi::Paint;
use crate::wallet::Wallet;

const DEFAULT_PROMPT: &str = "➜ ";

#[derive(Debug)]
pub struct Dispatcher {
    pub wallet: Wallet,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            wallet: Wallet::new(),
        }
    }

    pub fn prompt(&self) -> String {
        DEFAULT_PROMPT.green().to_string()
    }

    pub fn set_wallet_seeds(&mut self, seeds: String) {
        self.wallet.set_seeds(seeds);
    }
}


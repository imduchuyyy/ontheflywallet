use crate::dispatcher::Dispatcher;
use yansi::Paint;

pub trait LoginTrait {
    fn login(&mut self, seeds: String) -> eyre::Result<bool, String>;
}

impl LoginTrait for Dispatcher {
    fn login(&mut self, seeds: String) -> eyre::Result<bool, String> {
        match self.wallet.login(seeds) {
            Ok(address) => {
                println!("{}: Wallet Address: {:?}", "Success".green(), address);
                Ok(false)
            }
            Err(e) => Err(format!("Failed to login: {}", e)),
        }
    }
}


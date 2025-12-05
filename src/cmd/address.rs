use crate::dispatcher::Dispatcher;
use yansi::Paint;

pub trait AddressTrait {
    fn print_wallet_address(&mut self) -> eyre::Result<bool, String>;
}

impl AddressTrait for Dispatcher {
    fn print_wallet_address(&mut self) -> eyre::Result<bool, String> {
        match self.wallet.get_address() {
            Ok(address) => {
                println!("{}: Wallet Address: {:?}", "Success".green(), address);
                Ok(false)
            }
            Err(e) => Err(format!("Failed to get wallet address: {}", e)),
        }
    }
}

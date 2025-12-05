use crate::dispatcher::Dispatcher;
use yansi::Paint;

pub trait TransferTrait {
    fn transfer(&mut self, token: String, to: String, amount: String) -> eyre::Result<bool, String>;
}

impl TransferTrait for Dispatcher {
    fn transfer(&mut self, token: String, to: String, amount: String) -> eyre::Result<bool, String> {
        Ok(false)
    }
}

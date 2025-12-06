use crate::dispatcher::Dispatcher;

pub trait ClearTrait {
    fn clear(&mut self) -> eyre::Result<bool, String>;
}

impl ClearTrait for Dispatcher {
    fn clear(&mut self) -> eyre::Result<bool, String> {
        clearscreen::clear().expect("failed to clear screen");
        Ok(false)
    }
}

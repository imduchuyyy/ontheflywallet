
#[derive(Debug, Clone)]
pub struct Wallet {
    seeds: Option<String>,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet { seeds: None }
    }

    pub fn set_seeds(&mut self, seeds: String) {
        self.seeds = Some(seeds);
    }
}

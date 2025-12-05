use alloy::{
    hex,
    primitives::{Address, eip191_hash_message},
    signers::local::PrivateKeySigner,
};
use yansi::Paint;

#[derive(Debug, Clone)]
pub struct Wallet {
    signer: Option<PrivateKeySigner>,
    // rpc_url: String,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            signer: None,
            // rpc_url: "https://reth-ethereum.ithaca.xyz/rpc".to_string(),
        }
    }

    pub fn login(&mut self, seeds: String) -> eyre::Result<Address> {
        if seeds.is_empty() {
            return Err(eyre::eyre!("Seeds cannot be empty"));
        }
        let hash = eip191_hash_message(seeds.as_bytes());

        let signer: PrivateKeySigner = format!("0x{}", hex::encode(hash)).parse()?;
        let address = signer.address();
        self.signer = Some(signer);

        Ok(address)
    }

    pub fn get_address(&self) -> eyre::Result<Address> {
        if let Some(signer) = &self.signer {
            return Ok(signer.address());
        }
        Err(eyre::eyre!(
            "Wallet not logged in, call `{}` first",
            "login".green()
        ))
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_set_seeds() {
        let mut wallet = Wallet::new();
        let seeds = "hello".to_string();
        wallet.login(seeds.clone()).unwrap();

        let address = wallet.get_address().unwrap();
        assert_eq!(
            address,
            address!("0x56d67386939607c11bd60bb009eb02f4dd29c318")
        );
    }
}

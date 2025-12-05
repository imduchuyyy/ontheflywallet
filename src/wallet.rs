use alloy::{
    primitives::{
        eip191_hash_message,
        keccak256,
        Address,
        address
    },
    providers::{
        Provider,
        ProviderBuilder
    },
    signers::{
        local::PrivateKeySigner,
    },
    hex
};
use yansi::Paint;


#[derive(Debug, Clone)]
pub struct Wallet {
    signer: Option<PrivateKeySigner>,
    provider: dyn Provider
}

impl Wallet {
    pub fn new() -> Self {
        let rpc_url = "https://reth-ethereum.ithaca.xyz/rpc".parse().unwrap();

        Wallet { 
            signer: None,
            provider: ProviderBuilder::new().connect_http(rpc_url)
        }
    }

    pub fn login(&mut self, seeds: String) -> eyre::Result<Address>{
        if seeds.is_empty() {
            return Err(eyre::eyre!("Seeds cannot be empty"));
        }
        let hash = eip191_hash_message(seeds.as_bytes());

        let signer: PrivateKeySigner = format!("0x{}", hex::encode(&hash)).parse()?;
        let address = signer.address();
        self.signer = Some(signer);

        Ok(address)
    }

    pub fn get_address(&self) -> eyre::Result<Address> {
        if let Some(signer) = &self.signer {
            return Ok(signer.address());
        }
        Err(eyre::eyre!("Wallet not logged in, call `{}` first", "login".green()))
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
        assert_eq!(address, address!("0x56d67386939607c11bd60bb009eb02f4dd29c318"));
    }
}

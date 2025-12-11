use alloy::{
    hex,
    network::TransactionBuilder,
    primitives::{Address, U256, eip191_hash_message, utils::parse_units},
    providers::{Provider, ProviderBuilder},
    rpc::types::{TransactionReceipt, TransactionRequest},
    signers::local::PrivateKeySigner,
    sol,
};
use rpassword::read_password;
use std::io::Write;
use yansi::Paint;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "abi/erc20.json"
);

#[derive(Debug, Clone)]
pub struct Wallet {
    signer: Option<PrivateKeySigner>,
    rpc_url: String,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            signer: None,
            rpc_url: "https://reth-ethereum.ithaca.xyz/rpc".to_string(),
        }
    }

    pub fn provider(&self) -> impl Provider {
        ProviderBuilder::new().connect_http(self.rpc_url.parse().unwrap())
    }

    pub fn provider_with_signer(&self, signer: PrivateKeySigner) -> impl Provider {
        ProviderBuilder::new()
            .wallet(signer)
            .connect_http(self.rpc_url.parse().unwrap())
    }

    pub fn is_logged_in(&self) -> bool {
        self.signer.is_some()
    }

    fn get_signer(&self, seeds: String) -> eyre::Result<PrivateKeySigner> {
        let hash = eip191_hash_message(seeds.as_bytes());
        let signer: PrivateKeySigner = format!("0x{}", hex::encode(hash)).parse()?;
        Ok(signer)
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

    pub fn set_rpc_url(&mut self, url: String) {
        self.rpc_url = url;
    }

    pub fn rpc_url(&self) -> &String {
        &self.rpc_url
    }

    pub async fn get_balance(&self, token: Address, wallet_address: Address) -> eyre::Result<U256> {
        if token != Address::ZERO {
            let erc20 = ERC20::new(token, self.provider());
            let balance = erc20.balanceOf(wallet_address).call().await?;
            return Ok(balance);
        }
        let balance = self.provider().get_balance(wallet_address).await?;
        Ok(balance)
    }

    pub async fn get_token_name(&self, token: Address) -> eyre::Result<String> {
        if token == Address::ZERO {
            return Ok("ETH".to_string());
        }
        let erc20 = ERC20::new(token, self.provider());
        let name = erc20.name().call().await?;
        Ok(name)
    }

    async fn get_token_decimals(&self, token: Address) -> eyre::Result<u8> {
        if token == Address::ZERO {
            return Ok(18);
        }
        let erc20 = ERC20::new(token, self.provider());
        let decimals = erc20.decimals().call().await?;
        Ok(decimals)
    }

    pub async fn transfer_token(
        &self,
        token: Address,
        to: Address,
        amount: String,
    ) -> eyre::Result<()> {
        let token_name = self.get_token_name(token).await?;
        let token_decimals = self.get_token_decimals(token).await?;
        let pu = parse_units(&amount, token_decimals)?;
        let amount: U256 = pu.into();

        println!(
            "Transferring:\n-Token: {}\n-To: {}\n-Amount: {}",
            token_name, to, amount
        );

        print!("Type your seed: ");
        std::io::stdout().flush().unwrap();
        let password = read_password().unwrap();

        let signer = self.get_signer(password)?;
        let address = signer.address();
        println!("From address: {}", address);
        let provider = self.provider_with_signer(signer);

        let balance = self.get_balance(token, address).await?;

        println!("Current balance: {}", balance);

        println!("Sending transaction...");
        if token == Address::ZERO {
            let tx = TransactionRequest::default().with_to(to).with_value(amount);
            let pending_tx = provider.send_transaction(tx).await?;
            let tx_hash = pending_tx.tx_hash();
            println!("Transaction sent with hash: {:?}", tx_hash);
            Ok(())
        } else {
            let erc20 = ERC20::new(token, provider);

            let tx = erc20.transfer(to, amount);
            let pending_tx = tx.send().await.unwrap();
            let tx_hash = pending_tx.tx_hash();
            println!("Transaction sent with hash: {:?}", tx_hash);
            Ok(())
        }
    }

    pub async fn sign_and_send_transaction(
        &self,
        tx: TransactionRequest,
    ) -> eyre::Result<TransactionReceipt> {
        if let Some(signer) = &self.signer {
            let provider = ProviderBuilder::new()
                .wallet(signer.clone())
                .connect_http(self.rpc_url.parse().unwrap());
            let pending_tx = provider.send_transaction(tx).await?;
            pending_tx.get_receipt().await.map_err(|e| eyre::eyre!(e))
        } else {
            Err(eyre::eyre!(
                "Wallet not logged in, call `{}` first",
                "login".green()
            ))
        }
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
    use alloy::network::TransactionBuilder;
    use alloy::primitives::{U256, address};

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

    #[tokio::test]
    async fn test_sign_and_send_transaction() {
        let mut wallet = Wallet::new();
        let seeds = "hello".to_string();
        wallet.login(seeds.clone()).unwrap();

        let bob = address!("0xab5801a7d398351b8be11c439e05c5b32532b593");

        let tx = TransactionRequest::default()
            .with_to(bob)
            .with_chain_id(1)
            .with_value(U256::from(100))
            .with_gas_limit(21_000)
            .with_max_priority_fee_per_gas(1_000_000_000)
            .with_max_fee_per_gas(20_000_000_000);
        let receipt = wallet.sign_and_send_transaction(tx).await;
        // println!("Transaction receipt: {:?}", receipt);
    }
}

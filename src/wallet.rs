use alloy::{
    hex,
    network::TransactionBuilder,
    primitives::{Address, U256, eip191_hash_message, utils::parse_units},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};
use rpassword::read_password;
use std::io::Write;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "abi/erc20.json"
);

#[derive(Debug, Clone)]
pub struct Wallet {
    rpc_url: String,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
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

    async fn request_signer(&self) -> eyre::Result<PrivateKeySigner> {
        print!("Type your seed: ");
        std::io::stdout().flush().unwrap();
        let seeds = read_password().unwrap();
        if seeds.is_empty() {
            return Err(eyre::eyre!("Seeds cannot be empty"));
        }
        let hash = eip191_hash_message(seeds.as_bytes());
        let signer: PrivateKeySigner = format!("0x{}", hex::encode(hash)).parse()?;
        Ok(signer)
    }

    pub async fn get_address(&self) -> eyre::Result<Address> {
        let signer = self.request_signer().await?;
        let address = signer.address();
        Ok(address)
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

        let signer = self.request_signer().await?;
        let address = signer.address();
        println!("Using Address: {}", address);
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
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

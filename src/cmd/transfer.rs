use crate::dispatcher::Dispatcher;
use alloy::{
    primitives::{Address, U256},
    sol,
};
use std::str::FromStr;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "abi/erc20.json"
);
pub trait TransferTrait {
    fn transfer(
        &mut self,
        token: String,
        to: String,
        amount: String,
    ) -> impl std::future::Future<Output = eyre::Result<bool, String>> + Send;
}

impl TransferTrait for Dispatcher {
    async fn transfer(
        &mut self,
        token: String,
        to: String,
        amount: String,
    ) -> eyre::Result<bool, String> {
        let token_address: Address = Address::from_str(&token).unwrap();
        let to_address: Address = Address::from_str(&to).unwrap();
        self.wallet.transfer_token(token_address, to_address, amount).await.map_err(|e| e.to_string())?;

        Ok(false)
    }
}

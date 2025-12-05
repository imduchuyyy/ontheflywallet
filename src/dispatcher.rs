use crate::{
    wallet::Wallet,
    cmd::{
        address::AddressTrait,
        login::LoginTrait,
    }
};
use alloy::primitives::Address;
use yansi::Paint;
use clap::Parser;

#[derive(Debug)]
pub struct Dispatcher {
    pub wallet: Wallet,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            wallet: Wallet::new(),
        }
    }

    pub fn prompt(&self) -> String {
        "➜ ".green().to_string()
    }

    pub async fn dispatch(&mut self, input: &str) -> Result<bool, String> {
        return match Command::parse(input) {
            Ok(cmd) => match cmd {
                Command::Quit => Ok(true),
                Command::Help => self.help(),
                Command::Login { seeds } => self.login(seeds),
                Command::Address => self.print_wallet_address(),
            },
            Err(e) => Err(format!("Failed to parse command: {}", e)),
        }
    }

    fn help (&self) -> eyre::Result<bool, String> {
        println!("Available commands:");
        println!("  {} - Show this help message", "help".green());
        println!("  {} - Quit the REPL", "quit".green());
        println!("  {} - Login to the wallet", "login".green());
        println!("  {} - Get wallet address", "address".green());
        Ok(false)
    }
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new()
    }
}

// Wallet REPL commands.
#[derive(Debug, Parser)]
#[command(disable_help_flag = true, disable_help_subcommand = true)]
pub enum Command {
    #[command(visible_alias = "h")]
    Help,

    /// Quit the REPL.
    #[command(visible_alias = "q")]
    Quit,

    /// Login to the wallet
    #[command(visible_alias = "lg")]
    Login {
        /// Wallet seeds
        seeds: String,
    },

    /// Get Address
    #[command(visible_alias = "add")]
    Address,
}

impl Command {
    pub fn parse(input: &str) -> eyre::Result<Self> {
        let args = input.split_whitespace();
        let args = std::iter::once("otfwallet").chain(args);
        Self::try_parse_from(args)
            .map_err(|e| eyre::eyre!("{}; for more information, see `help`", e.kind()))
    }
}



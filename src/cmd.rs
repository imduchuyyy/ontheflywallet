use clap::Parser;

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
    Login,

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

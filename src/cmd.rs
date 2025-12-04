use clap::Parser;

// Wallet REPL commands.
#[derive(Debug, Parser)]
pub enum Command {
    /// Quit the REPL.
    #[command(visible_alias = "q")]
    Quit,
}

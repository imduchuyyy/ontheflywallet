use crate::wallet::Wallet;
use crate::cmd::Command;
use yansi::Paint;

#[derive(Debug)]
pub enum DispatcherState {
    Idle,
    AwaitingSeeds,
}

#[derive(Debug)]
pub struct Dispatcher {
    state: DispatcherState,
    wallet: Wallet,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            state: DispatcherState::Idle,
            wallet: Wallet::new(),
        }
    }

    pub fn prompt(&self) -> String {
        match &self.state {
            DispatcherState::Idle => "➜ ".green().to_string(),
            DispatcherState::AwaitingSeeds => "Enter wallet seeds: ".yellow().to_string(),
        }
    }

    pub async fn dispatch(&mut self, input: &str) -> Result<bool, String> {
        // this could be main command or parameters for commands
        return match self.state {
            DispatcherState::Idle => self.handle_idle_input(input).await,
            DispatcherState::AwaitingSeeds => {
                self.login(input.to_string()).unwrap();
                self.state = DispatcherState::Idle;
                Ok(false)
            }
        }
    }

    async fn handle_idle_input(&mut self, input: &str) -> Result<bool, String> {
        return match Command::parse(input) {
            Ok(cmd) => match cmd {
                Command::Quit => Ok(true),
                Command::Help => self.help(),
                Command::Login => {
                    self.state = DispatcherState::AwaitingSeeds;
                    Ok(false)
                }
                Command::Address => self.get_wallet_address(),
            },
            Err(e) => Err(format!("Failed to parse command: {}", e)),
        }
    }
}

impl Dispatcher {
    fn help (&self) -> eyre::Result<bool, String> {
        println!("Available commands:");
        println!("  {} - Show this help message", "help".green());
        println!("  {} - Quit the REPL", "quit".green());
        println!("  {} - Login to the wallet", "login".green());
        println!("  {} - Get wallet address", "address".green());
        Ok(false)
    }

    fn login(&mut self, seeds: String) -> eyre::Result<bool, String> {
        match self.wallet.login(seeds) {
            Ok(address) => {
                println!("{}: Wallet Address: {:?}", "Success".green(), address);
            }
            Err(e) => {
                println!("{}: Failed to login: {}", "Err".red(), e);
            }
        }
        Ok(false)
    }

    fn get_wallet_address(&self) -> eyre::Result<bool, String> {
        match self.wallet.get_address() {
            Ok(address) => {
                println!("{}: Wallet Address: {:?}", "Success".green(), address);
            }
            Err(e) => {
                println!("{}: Failed to get wallet address: {}", "Err".red(), e);
            }
        }
        Ok(false)
    }
}

impl Default for Dispatcher {
    fn default() -> Self {
        Self::new()
    }
}

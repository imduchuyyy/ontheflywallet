use crate::{dispatcher::Dispatcher, opts::OnTheFlyWallet};
use clap::Parser;
use eyre::Result;
use rustyline::DefaultEditor as Editor;
use yansi::Paint;

pub fn run() -> Result<()> {
    // let args = OnTheFlyWallet::parse();
    let mut dispatcher = Dispatcher::new();

    let mut rl = Editor::new()?;
    println!(
        "Welcome to On-The-Fly Wallet! Type '{}' to exit.",
        "quit".green()
    );
    loop {
        let readline = rl.readline(&dispatcher.prompt());
        match readline {
            Ok(line) => {
                let args = OnTheFlyWallet::parse_from(line.split_whitespace());
                match args.cmd {
                    Some(cmd) => match cmd {
                        crate::cmd::Command::Quit => break,
                    },
                    None => continue,
                }
            }
            Err(_) => break,
        }
    }
    Ok(())
}

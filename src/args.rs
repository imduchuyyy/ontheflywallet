use crate::dispatcher::Dispatcher;
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
                println!("{}", line);
            }
            Err(_) => break,
        }
    }
    Ok(())
}

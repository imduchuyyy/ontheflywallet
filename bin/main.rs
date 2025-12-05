use eyre::Result;
use rustyline::DefaultEditor as Editor;
use ontheflywallet::dispatcher::Dispatcher;
use yansi::Paint;

#[tokio::main]
async fn main() -> Result<()> {
    let mut dispatcher = Dispatcher::new();

    let mut rl = Editor::new()?;
    println!(
        "Welcome to On-The-Fly Wallet! Type `{}` to show available commands.",
        "help".green()
    );
    loop {
        let readline = rl.readline(&dispatcher.prompt());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match dispatcher.dispatch(&line).await {
                    Ok(should_quit) => {
                        if should_quit {
                            println!("Exiting On-The-Fly Wallet. Goodbye!");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("{}: {}", "Error".red(), e);
                    }
                }
            }
            Err(_) => break,
        }
    }
    Ok(())
}

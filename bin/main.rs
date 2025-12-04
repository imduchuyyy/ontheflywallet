use ontheflywallet::args::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

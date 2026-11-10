mod core;
mod cli;

fn main() {
    if let Err(err) = cli::run() {
        eprintln!("Fehler: {err}");
        std::process::exit(1);
    }
}

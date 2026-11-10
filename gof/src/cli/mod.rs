use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "gof", version, about = "Prepare the data for the GOF extension.")]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct TestArgs {
    #[arg(long)]
    pub overwrite: bool,

    #[arg(short, long, default_value_t = 80)]
    pub quality: u8,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Test(TestArgs),
}

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();
    if cli.verbose {
        eprintln!("[DEBUG] Verbose active.")
    }
    match cli.command {
        Commands::Test(args) => {
            println!("Test command used with: ");
        }
    }
    Ok(())
}
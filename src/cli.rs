use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mcpkg")]
#[command(about = "A model context package manager for developers")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Login,
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login => {
            println!("Login command executed");
        }
    }
}

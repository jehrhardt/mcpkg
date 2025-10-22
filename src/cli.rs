use clap::{Parser, Subcommand};

use crate::mcp;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Start,
}

pub(crate) async fn run() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Start => mcp::run().await,
        }
    }
}

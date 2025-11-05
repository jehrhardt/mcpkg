mod cli;
mod mcp;
mod prompts;

#[tokio::main]
async fn main() {
    cli::run().await;
}

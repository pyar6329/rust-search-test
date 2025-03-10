use anyhow::{Error, Result};
use searchy::cli::run_cli;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_cli().await?;
    Ok(())
}

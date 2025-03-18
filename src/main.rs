use anyhow::{Error, Result};
use searchy::cli::run_cli;
// use searchy::search::foobar;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_cli().await?;
    // foobar();
    Ok(())
}

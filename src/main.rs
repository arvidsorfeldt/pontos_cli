//! PONTOS data hub CLI
#![warn(missing_docs)]
extern crate pontos_cli;
use pontos_cli::io::day_to_csv;
use pontos_cli::io::list_vessels;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    list_vessels().await?;
    day_to_csv("2023-11-07").await?;
    Ok(())
}

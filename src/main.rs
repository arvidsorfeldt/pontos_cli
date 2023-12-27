//! PONTOS data hub CLI
#![warn(missing_docs)]
extern crate pontos_cli;

use chrono::NaiveDate;
use pontos_cli::io::day_to_csv;
use pontos_cli::io::list_vessels;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    list_vessels().await?;
    let date = NaiveDate::parse_from_str("2023-11-07", "%Y-%m-%d").unwrap();
    day_to_csv(date).await?;
    Ok(())
}

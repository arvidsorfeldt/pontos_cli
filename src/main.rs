//! PONTOS data hub CLI
#![warn(missing_docs)]

use chrono::NaiveDate;
use clap::{Args, Parser};
use pontoslib::io::day_to_csv;
use pontoslib::io::list_vessels;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = PontosCli::parse();

    match args {
        PontosCli::List => {
            list_vessels().await?;
        }
        PontosCli::Data(args) => {
            day_to_csv(&args.vessel_id, args.date).await?;
        }
    }
    Ok(())
}

#[derive(Parser)]
#[command(name = "pontos")]
#[command(bin_name = "pontos")]
/// A CLI utility for downloading operational data from the PONTOS data hub.
enum PontosCli {
    /// List available vessel ids on the PONTOS data hub.
    List,
    /// Download daily data as csv files.
    Data(DataArgs),
}

#[derive(Args)]
struct DataArgs {
    #[arg(short, long, default_value_t = {"name_SD401Fredrika".to_string()})]
    vessel_id: String,
    #[arg(short, long, default_value_t = {NaiveDate::from_ymd_opt(2023, 11, 07).unwrap()})]
    date: NaiveDate,
}

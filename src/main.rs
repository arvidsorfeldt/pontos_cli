mod pontos;

use pontos::{get_vessel_data, get_vessel_ids};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_vessel_ids().await?;
    let data = get_vessel_data().await?;
    let mut writer = csv::Writer::from_path("records.csv").unwrap();
    for record in data {
        writer.serialize(record)?;
    }
    writer.flush()?;

    Ok(())
}

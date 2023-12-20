use csv::Writer;
use futures::join;

use crate::data::{get_other_data, get_vessel_ids, get_vessel_position_data};

fn output_csv<T: serde::Serialize>(
    date: &str,
    parameter: String,
    list: Vec<T>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = Writer::from_path(parameter + "_" + date + ".csv").unwrap();
    for record in list {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}

/// Saves all available data for a given day in separate csv files.
pub async fn day_to_csv(date: &str) -> Result<(), Box<dyn std::error::Error>> {
    let positions = get_vessel_position_data(date);
    let other_data = get_other_data(date);
    let (positions, other_data) = join!(positions, other_data);
    output_csv(date, "position".to_string(), positions?)?;

    for (p, data) in other_data? {
        output_csv(date, p.as_str_short().to_string(), data)?;
    }

    Ok(())
}
/// Lists the availavle vessels on the PONTOS data hub.
pub async fn list_vessels() -> Result<(), Box<dyn std::error::Error>> {
    let vessels = get_vessel_ids().await?;
    for vessel in vessels {
        println!("{}", vessel);
    }
    Ok(())
}
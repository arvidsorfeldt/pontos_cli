use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::env;

const PONTOS_URL: &str = "https://pontos.ri.se/api";

fn get_pontos_token() -> String {
    env::var("PONTOS_TOKEN").expect("The PONTOS_TOKEN environment variable has not been set.")
}

pub async fn get_vessel_ids() -> Result<(), Box<dyn std::error::Error>> {
    let pontos_token = get_pontos_token();
    let client = Postgrest::new(PONTOS_URL);
    let resp = client
        .from("vessel_ids")
        .auth(pontos_token)
        .execute()
        .await?;
    println!("{}", resp.text().await?);
    Ok(())
}

pub async fn get_vessel_data() -> Result<Vec<ShipData>, Box<dyn std::error::Error>> {
    let pontos_token = get_pontos_token();
    let client = Postgrest::new(PONTOS_URL);
    let resp = client
        .from("vessel_data")
        .auth(pontos_token)
        .eq("vessel_id", "name_SD401Fredrika")
        //.neq("parameter_id", "enginemain_fuelcons_lph_1")
        .neq("value", "0")
        .gte("time", "2023-11-7")
        .lt("time", "2023-11-15")
        .limit(1000)
        .select("time,parameter_id,value")
        .execute()
        .await?;
    Ok(serde_json::from_str(&resp.text().await?).unwrap())
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct ShipData {
    time: DateTime<Utc>,
    parameter_id: Parameter,
    #[serde_as(as = "DisplayFromStr")]
    value: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
enum Parameter {
    #[serde(rename(deserialize = "positioningsystem_latitude_deg_1"))]
    Latitude,
    #[serde(rename(deserialize = "positioningsystem_longitude_deg_1"))]
    Longitude,
    #[serde(rename(deserialize = "positioningsystem_sog_kn_1"))]
    Speed,
    #[serde(rename(deserialize = "steering_order_deg_1"))]
    SteeringOrder,
    #[serde(rename(deserialize = "steering_angle_deg_1"))]
    SteeringAngle,
    #[serde(rename(deserialize = "positioningsystem_heading_deg_1"))]
    Heading,
    #[serde(rename(deserialize = "positioningsystem_cog_deg_1"))]
    Course,
    #[serde(rename(deserialize = "enginemain_fuelcons_lph_1"))]
    FuelConsumption,
    #[serde(rename(deserialize = "rudder_order_deg_1"))]
    RudderOrder,
    #[serde(rename(deserialize = "rudder_angle_deg_1"))]
    RudderAngle,
}

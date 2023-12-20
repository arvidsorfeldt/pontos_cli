use chrono::{DateTime, Utc};
use futures::{future::join_all, join};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{env, fmt};

const PONTOS_URL: &str = "https://pontos.ri.se/api";
type ParameterAndData = (Parameter, Vec<ShipData>);

fn get_pontos_token() -> String {
    env::var("PONTOS_TOKEN").expect("The PONTOS_TOKEN environment variable has not been set.")
}

pub async fn get_vessel_ids() -> Result<Vec<Vessel>, Box<dyn std::error::Error>> {
    let pontos_token = get_pontos_token();
    let client = Postgrest::new(PONTOS_URL);
    let resp = client
        .from("vessel_ids")
        .auth(pontos_token)
        .execute()
        .await?;
    let data: Vec<Vessel> = serde_json::from_str(&resp.text().await?).unwrap();
    Ok(data)
}

pub async fn get_other_data(
    date: &str,
) -> Result<Vec<ParameterAndData>, Box<dyn std::error::Error>> {
    let futures = vec![
        get_vessel_data(date, Parameter::Speed),
        get_vessel_data(date, Parameter::SteeringOrder),
        get_vessel_data(date, Parameter::SteeringAngle),
        get_vessel_data(date, Parameter::Heading),
        get_vessel_data(date, Parameter::Course),
        get_vessel_data(date, Parameter::FuelConsumption),
        get_vessel_data(date, Parameter::RudderOrder),
        get_vessel_data(date, Parameter::RudderAngle),
    ];
    let results = join_all(futures).await;
    let data: Vec<ParameterAndData> = results.into_iter().map(Result::unwrap).collect();
    let (empty, non_empty): (Vec<ParameterAndData>, Vec<ParameterAndData>) =
        data.into_iter().partition(|(_, values)| values.is_empty());
    empty
        .iter()
        .for_each(|(p, _)| println!("{} was empty!", p.as_str_short()));
    Ok(non_empty)
}

pub async fn get_vessel_position_data(
    date: &str,
) -> Result<Vec<Position>, Box<dyn std::error::Error>> {
    let longitude = get_vessel_data(date, Parameter::Longitude);
    let latitude = get_vessel_data(date, Parameter::Latitude);
    let (longitude, latitude) = join!(longitude, latitude);
    let (_, longitude) = longitude?; // Throw away parameter info
    let (_, latitude) = latitude?; // Throw away parameter info
    let positions = pair_longitude_latitude(longitude, latitude);
    Ok(positions)
}

fn pair_longitude_latitude(longitude: Vec<ShipData>, latitude: Vec<ShipData>) -> Vec<Position> {
    let mut lng_iter = longitude.iter();
    let mut lat_iter = latitude.iter();
    let mut positions: Vec<Position> = Vec::new();
    let mut lng_opt = lng_iter.next();
    let mut lat_opt = lat_iter.next();

    while let (Some(lng), Some(lat)) = (lng_opt, lat_opt) {
        match lng.time.cmp(&lat.time) {
            std::cmp::Ordering::Less => {
                lng_opt = lng_iter.next();
            }
            std::cmp::Ordering::Equal => {
                positions.push(Position::new(lng.time, lng.value, lat.value));
                lng_opt = lng_iter.next();
                lat_opt = lat_iter.next();
            }
            std::cmp::Ordering::Greater => {
                lat_opt = lat_iter.next();
            }
        }
    }

    positions
}

pub async fn get_vessel_data(
    date: &str,
    parameter: Parameter,
) -> Result<(Parameter, Vec<ShipData>), Box<dyn std::error::Error>> {
    let pontos_token = get_pontos_token();
    let client = Postgrest::new(PONTOS_URL);
    let resp = client
        .from("vessel_data")
        .auth(pontos_token)
        .eq("vessel_id", "name_SD401Fredrika")
        .eq("parameter_id", parameter.as_str())
        .gte("time", date)
        .lt("time", "2023-11-8")
        //.limit(1000)
        .select("time,parameter_id,value")
        .execute()
        .await?;
    let mut data: Vec<ShipData> = serde_json::from_str(&resp.text().await?).unwrap();
    data.sort_by_key(|item| item.time);
    Ok((parameter, data))
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct ShipData {
    time: DateTime<Utc>,
    #[serde_as(as = "DisplayFromStr")]
    value: f32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum Parameter {
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

impl Parameter {
    fn as_str(&self) -> &str {
        match self {
            Parameter::Latitude => "positioningsystem_latitude_deg_1",
            Parameter::Longitude => "positioningsystem_longitude_deg_1",
            Parameter::Speed => "positioningsystem_sog_kn_1",
            Parameter::SteeringOrder => "steering_order_deg_1",
            Parameter::SteeringAngle => "steering_angle_deg_1",
            Parameter::Heading => "positioningsystem_heading_deg_1",
            Parameter::Course => "positioningsystem_cog_deg_1",
            Parameter::FuelConsumption => "enginemain_fuelcons_lph_1",
            Parameter::RudderOrder => "rudder_order_deg_1",
            Parameter::RudderAngle => "rudder_angle_deg_1",
        }
    }
    pub fn as_str_short(&self) -> &str {
        match self {
            Parameter::Latitude => "latitude",
            Parameter::Longitude => "longitude",
            Parameter::Speed => "sog",
            Parameter::SteeringOrder => "steering_order",
            Parameter::SteeringAngle => "steering_angle",
            Parameter::Heading => "heading",
            Parameter::Course => "cog",
            Parameter::FuelConsumption => "enginemain_fuelcons",
            Parameter::RudderOrder => "rudder_order",
            Parameter::RudderAngle => "rudder_angle",
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    time: DateTime<Utc>,
    longitude: f32,
    latitude: f32,
}

impl Position {
    fn new(time: DateTime<Utc>, longitude: f32, latitude: f32) -> Position {
        Position {
            time,
            longitude,
            latitude,
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Vessel {
    vessel_id: String,
}

impl fmt::Display for Vessel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.vessel_id)
    }
}

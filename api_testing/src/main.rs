// api_test in rust
pub mod data_struct;

use clap::Parser;
use dotenv::dotenv;
use reqwest::Error;
use std::env;
#[allow(unused_imports)]
use serde_json::to_string_pretty;
#[allow(unused_imports)]
use serde_json::Value;

use crate::data_struct::OpenWeatherMapResponse;

#[derive(Debug, Parser)]
struct Args {
    // Latitude to use for the request
    #[clap(short = 'a', long = "lat", default_value = "40.7128", allow_hyphen_values = true)]
    lat: f64,
    // Longitude to use for the request
    #[clap(short = 'o', long = "lon", default_value = "-74.0060", allow_hyphen_values = true)]
    lon: f64
}

#[tokio::main] // used for async calls
async fn main() -> Result<(), Error> {
    dotenv().expect(".env file not found");

    let api_key = env::var("API_KEY").expect("API_KEY not found");
    let args = Args::parse();
    let lat = args.lat.to_string();
    let lon = args.lon.to_string();

    let request_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?\
            lat={lat}&lon={lon}&appid={API_KEY}&units={units}",
        lat = lat,
        lon = lon,
        API_KEY = api_key,
        units = "imperial"
    );

    // println!("request url: {}", request_url);

    let response = reqwest::get(&request_url).await?;
    println!("reponse status: {}", &response.status());

    // let response2 = reqwest::get(&request_url).await?;

    // let data = &response2.json::<Value>().await?;
    // println!("{}", to_string_pretty(&data).unwrap());

    let weather_data: OpenWeatherMapResponse = response.json().await?;
    // println!("{:?}", &weather_data);

    println!(
        "The temperature at ({:?}, {:?}) is: {:?} degrees F", 
        weather_data.coord.lat.unwrap(),
        weather_data.coord.lon.unwrap(),
        weather_data.main.temp.unwrap()
    );
    
    Ok(())
}

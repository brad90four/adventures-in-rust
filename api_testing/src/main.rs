// api_test in rust
pub mod data_struct;

use dotenv::dotenv;
use reqwest::Error;
use std::env;
use serde_json::to_string_pretty;
use serde_json::Value;

use crate::data_struct::OpenWeatherMapResponse;

#[tokio::main] // used for async calls
async fn main() -> Result<(), Error> {
    dotenv().expect(".env file not found");

    let api_key = env::var("API_KEY").expect("API_KEY not found");

    let request_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?\
            lat={lat}&lon={lon}&appid={API_KEY}&units={units}",
        lat = "0",
        lon = "0",
        API_KEY = api_key,
        units = "imperial"
    );

    println!("request url: {}", request_url);

    let response = reqwest::get(&request_url).await?;
    println!("reponse status: {}", &response.status());

    let response2 = reqwest::get(&request_url).await?;

    let data = &response2.json::<Value>().await?;
    println!("{}", to_string_pretty(&data).unwrap());

    let weather_data: OpenWeatherMapResponse = response.json().await?;
    println!("{:?}", &weather_data);

    // error is currently: 
    // Error: reqwest::Error { kind: Decode, source: Error("missing field `feelsLike`", line: 1, column: 229) }
    
    Ok(())
}

// api_test in rust
extern crate serde;
extern crate serde_json; // 1.0.79;
extern crate reqwest; // 0.11.10


use serde::Deserialize;
use serde_json::Value;
use reqwest::{ ClientBuilder, Error };


#[tokio::main] // used for async calls
async fn main() -> Result<(), Error> {
    let api_key = ""; // implement reading from .env file
    let request_url = format!("https://api.openweathermap.org/data/2.5/weather?\
        lat={lat}&lon={lon}&appid={API_KEY}&units={units}",
        lat = "0",
        lon = "0",
        API_KEY = api_key,
        units = "imperial");
    println!("request url: {}", request_url);

    let timeout = std::time::Duration::from_secs(15);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.get(&request_url).send().await?;
    println!("reponse status: {}", &response.status());
    let json: serde_json::Value = response.json().await?;
    println!("{:?}", &json);
    Ok(())
}
// api_test in rust
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;

use dotenv::dotenv;
use reqwest::Error;
#[allow(unused_imports)]
use serde::Deserialize;
#[allow(unused_imports)]
use serde_json::Value;
use std::env;

pub mod data_struct;

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

    let data = &response.json::<Value>().await?;

    let weather_data = data_struct::OpenWeatherMapResponse {
        base: data["base"].as_str().unwrap().to_string(),
        clouds: data_struct::Clouds {
            all: data["clouds"].get("all").unwrap().as_i64().unwrap(),
        },
        cod: data["cod"].as_i64().unwrap(),
        coord: data_struct::Coord {
            lat: data["coord"].get("lat").unwrap().as_f64().unwrap(),
            lon: data["coord"].get("lon").unwrap().as_f64().unwrap(),
        },
        dt: data["dt"].as_i64().unwrap(),
        id: data["id"].as_i64().unwrap(),
        main: data_struct::Main {
            feels_like: data["main"].get("feels_like").unwrap().as_f64().unwrap(),
            grnd_level: data["main"].get("grnd_level").unwrap().as_i64().unwrap(),
            humidity: data["main"].get("humidity").unwrap().as_i64().unwrap(),
            pressure: data["main"].get("pressure").unwrap().as_i64().unwrap(),
            sea_level: data["main"].get("sea_level").unwrap().as_i64().unwrap(),
            temp: data["main"].get("temp").unwrap().as_f64().unwrap(),
            temp_max: data["main"].get("temp_max").unwrap().as_f64().unwrap(),
            temp_min: data["main"].get("temp_min").unwrap().as_f64().unwrap(),
        },
        name: data["name"].as_str().unwrap().to_string(),
        sys: data_struct::Sys {
            sunrise: data["sys"].get("sunrise").unwrap().as_i64().unwrap(),
            sunset: data["sys"].get("sunset").unwrap().as_i64().unwrap(),
        },
        timezone: data["timezone"].as_i64().unwrap(),
        visibility: data["visibility"].as_i64().unwrap(),
        weather: data_struct::Weather {
            description: data["weather"].get("description").unwrap().to_string(), // broken here
            icon: data["weather"].get("icon").unwrap().to_string(),
            id: data["weather"].get("id").unwrap().as_i64().unwrap(),
            main: data["weather"].get("main").unwrap().to_string(),
        },
        wind: data_struct::Wind {
            deg: data["wind"].get("deg").unwrap().as_i64().unwrap(),
            gust: data["wind"].get("gust").unwrap().as_f64().unwrap(),
            speed: data["wind"].get("speed").unwrap().as_f64().unwrap(),
        }
    };


    let json = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", &json);

    println!("{:?}", &weather_data);
    Ok(())
}

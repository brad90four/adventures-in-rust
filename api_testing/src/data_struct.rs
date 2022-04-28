// openweathermap.org API JSON response struct
extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenWeatherMapResponse {
    pub base: String,
    pub clouds: Clouds,
    pub cod: i64,
    pub coord: Coord,
    pub dt: i64,
    pub id: i64,
    pub main: Main,
    pub name: String,
    pub sys: Sys,
    pub timezone: i64,
    pub visibility: i64,
    pub weather: Weather,
    pub wind: Wind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Clouds {
    pub all: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub feels_like: f64,
    pub grnd_level: i64,
    pub humidity: i64,
    pub pressure: i64,
    pub sea_level: i64,
    pub temp: f64,
    pub temp_max: f64,
    pub temp_min: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub description: String,
    pub icon: String,
    pub id: i64,
    pub main: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub deg: i64,
    pub gust: f64,
    pub speed: f64,
}
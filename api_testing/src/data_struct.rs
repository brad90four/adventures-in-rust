// openweathermap.org API JSON response struct
extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenWeatherMapResponse {
    pub base: String,
    pub clouds: Clouds,
    pub cod: Option<f64>,
    pub coord: Coord,
    pub dt: Option<f64>,
    pub id: Option<f64>,
    pub main: Main,
    pub name: String,
    pub sys: Sys,
    pub timezone: Option<f64>,
    pub visibility: Option<f64>,
    pub weather: Vec<Weather>,
    pub wind: Wind,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Clouds {
    pub all: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub feels_like: Option<f64>,
    pub grnd_level: Option<f64>,
    pub humidity: Option<f64>,
    pub pressure: Option<f64>,
    pub sea_level: Option<f64>,
    pub temp: Option<f64>,
    pub temp_max: Option<f64>,
    pub temp_min: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    pub sunrise: Option<f64>,
    pub sunset: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub description: String,
    pub icon: String,
    pub id: Option<f64>,
    pub main: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wind {
    pub deg: Option<f64>,
    pub gust: Option<f64>,
    pub speed: Option<f64>,
}
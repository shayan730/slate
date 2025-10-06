use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub timezone_offset: i64,
    pub current: CurrentWeather,
    pub hourly: Vec<HourlyWeather>,
    pub daily: Vec<DailyWeather>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub dt: u64,
    pub sunrise: u64,
    pub sunset: u64,
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: u64,
    pub humidity: u64,
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: u64,
    pub visibility: u64,
    pub wind_speed: f64,
    pub wind_deg: u64,
    pub weather: Vec<WeatherCondition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyWeather {
    pub dt: u64,
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: u64,
    pub humidity: u64,
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: u64,
    pub visibility: u64,
    pub wind_speed: f64,
    pub wind_deg: u64,
    pub wind_gust: Option<f64>,
    pub weather: Vec<WeatherCondition>,
    pub pop: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyWeather {
    pub dt: u64,
    pub sunrise: u64,
    pub sunset: u64,
    pub moonrise: u64,
    pub moonset: u64,
    pub moon_phase: f64,
    pub summary: String,
    pub temp: DailyTemp,
    pub feels_like: DailyFeelsLike,
    pub pressure: u64,
    pub humidity: u64,
    pub dew_point: f64,
    pub wind_speed: f64,
    pub wind_deg: u64,
    pub wind_gust: Option<f64>,
    pub weather: Vec<WeatherCondition>,
    pub clouds: u64,
    pub pop: f64,
    pub rain: Option<f64>,
    pub uvi: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyTemp {
    pub day: f64,
    pub min: f64,
    pub max: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyFeelsLike {
    pub day: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherCondition {
    pub id: u64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

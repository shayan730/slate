use chrono::{Local, TimeZone};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OWM_API_KEY")?;

    // Athens, GA coordinates
    let lat = 33.9519;
    let lon = -83.3576;

    let url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&units=imperial&appid={}",
        lat, lon, api_key
    );

    let res = reqwest::get(&url).await?;
    let weather: WeatherResponse = res.json().await?;

    println!("📍 Athens, GA");
    println!("Current Temp: {}°F", weather.current.temp);
    println!("Daily Forecasts:");

    for day in weather.daily.iter().take(7) {
        if let chrono::LocalResult::Single(datetime) = Local.timestamp_opt(day.dt as i64, 0) {
            println!(
                "{}: {}°F - {}°F, {}",
                datetime.format("%a, %b %e"),
                day.temp.min,
                day.temp.max,
                day.weather
                    .first()
                    .map(|w| w.description.as_str())
                    .unwrap_or("No description")
            );
        }
    }

    println!();

    for hour in weather.hourly.iter().take(12) {
        if let chrono::LocalResult::Single(datetime) = Local.timestamp_opt(hour.dt as i64, 0) {
            println!(
                "{}: {}°F, {}",
                datetime.format("%a %l %p").to_string().trim(),
                hour.temp,
                hour.weather
                    .first()
                    .map(|w| w.description.as_str())
                    .unwrap_or("No description")
            );
        }
    }

    Ok(())
}

// ------------------------ Struct Definitions ------------------------

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

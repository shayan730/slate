use crate::models::WeatherResponse;
use std::env;

pub async fn fetch_weather() -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let api_key = env::var("OWM_API_KEY")?;

    let lat = 33.9519;
    let lon = -83.3576;

    let url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&units=imperial&appid={}",
        lat, lon, api_key
    );

    let res = reqwest::get(&url).await?;
    let weather: WeatherResponse = res.json().await?;
    Ok(weather)
}


mod api;
mod models;

use chrono::{Local, TimeZone};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let weather = api::fetch_weather().await?;

    println!("📍 Athens, GA");
    println!("Current Temp: {}°F", weather.current.temp);

    if let chrono::LocalResult::Single(datetime) =
        Local.timestamp_opt(weather.current.sunrise as i64, 0)
    {
        println!("Sunrise: {}", datetime.format("%l:%M %p"))
    };
    if let chrono::LocalResult::Single(datetime) =
        Local.timestamp_opt(weather.current.sunset as i64, 0)
    {
        println!("Sunset: {}", datetime.format("%l:%M %p"))
    };

    println!();
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

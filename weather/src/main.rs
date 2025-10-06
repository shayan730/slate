mod api;
mod models;

use chrono::{Local, TimeZone};
use dotenv::dotenv;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use tera::{Context, Tera};

fn map_icon_to_class(icon: &str) -> &str {
    match icon {
        "01d" => "wi-day-sunny",
        "01n" => "wi-night-clear",
        "02d" => "wi-day-cloudy",
        "02n" => "wi-night-alt-cloudy",
        "03d" | "03n" => "wi-cloud",
        "04d" | "04n" => "wi-cloudy",
        "09d" | "09n" => "wi-showers",
        "10d" => "wi-day-rain",
        "10n" => "wi-night-alt-rain",
        "11d" | "11n" => "wi-thunderstorm",
        "13d" | "13n" => "wi-snow",
        "50d" | "50n" => "wi-fog",
        _ => "wi-na", // fallback icon
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let weather = api::fetch_weather().await?;

    // Setup Tera
    let tera = Tera::new("templates/*.tera")?;

    // Prepare Tera context
    let mut context = Context::new();

    context.insert("current_temp", &weather.current.temp);

    if let chrono::LocalResult::Single(datetime) =
        Local.timestamp_opt(weather.current.sunrise as i64, 0)
    {
        context.insert("sunrise", &datetime.format("%l:%M %p").to_string());
    }

    if let chrono::LocalResult::Single(datetime) =
        Local.timestamp_opt(weather.current.sunset as i64, 0)
    {
        context.insert("sunset", &datetime.format("%l:%M %p").to_string());
    }

    let daily: Vec<HashMap<&str, String>> = weather
        .daily
        .iter()
        .take(7)
        .filter_map(|day| {
            if let chrono::LocalResult::Single(datetime) = Local.timestamp_opt(day.dt as i64, 0) {
                let mut map = HashMap::new();
                map.insert("date", datetime.format("%a, %b %e").to_string());
                map.insert("min", format!("{:.1}", day.temp.min));
                map.insert("max", format!("{:.1}", day.temp.max));

                let description = day
                    .weather
                    .first()
                    .map(|w| w.description.clone())
                    .unwrap_or_else(|| "No description".to_string());
                map.insert("description", description);

                let icon = day.weather.first().map(|w| w.icon.as_str()).unwrap_or("na");
                map.insert("icon", map_icon_to_class(icon).to_string());

                Some(map)
            } else {
                None
            }
        })
        .collect();

    context.insert("daily", &daily);

    let hourly: Vec<HashMap<&str, String>> = weather
        .hourly
        .iter()
        .take(12)
        .filter_map(|hour| {
            if let chrono::LocalResult::Single(datetime) = Local.timestamp_opt(hour.dt as i64, 0) {
                let mut map = HashMap::new();
                map.insert(
                    "time",
                    datetime.format("%a %l %p").to_string().trim().to_string(),
                );
                map.insert("temp", format!("{:.1}", hour.temp));

                let description = hour
                    .weather
                    .first()
                    .map(|w| w.description.clone())
                    .unwrap_or_else(|| "No description".to_string());
                map.insert("description", description);

                let icon = hour
                    .weather
                    .first()
                    .map(|w| w.icon.as_str())
                    .unwrap_or("na");
                map.insert("icon", map_icon_to_class(icon).to_string());

                Some(map)
            } else {
                None
            }
        })
        .collect();

    context.insert("hourly", &hourly);

    // Render template
    let rendered = tera.render("weather.html.tera", &context)?;

    // Write to file
    let mut file = File::create("weather.html")?;
    file.write_all(rendered.as_bytes())?;

    println!("✅ Weather report written to 'weather.html'");

    Ok(())
}

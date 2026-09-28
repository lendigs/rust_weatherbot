use std::io::{self, Write};
fn convert_pressure(pressure: u64) -> u64 {
  (pressure as f64 * 0.7500616827).round() as u64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key: &str = "c60f128b5a2b005a11f4727e83852285";
    let mut city: String = String::new();
    print!("Enter City: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut city).expect("Error reading line!");
    print!("[DEBUG] CITY: {city}");
    io::stdout().flush()?;
    // API CALLS
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units=metric&lang=en");
    let data: serde_json::Value = reqwest::get(&url).await?.json().await?;

    println!("City, Country: {}, {}", data["name"].as_str().unwrap_or(""), data["sys"]["country"].as_str().unwrap_or(""));
    println!("Temperature(feels like): {}°C ({}°C)", data["main"]["temp"], data["main"]["feels_like"]);
    println!("Humidity: {}%", data["main"]["humidity"]);
    println!("Wind: {}m/sec", data["wind"]["speed"]);
    println!("Pressure: {}mmHg", convert_pressure(data["main"]["pressure"].as_u64().unwrap_or(0)));
    println!("Description: {}", data["weather"][0]["description"].as_str().unwrap_or(""));
    Ok(())
}

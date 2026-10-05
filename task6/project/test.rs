use serde::Serialize;
use serde::Deserialize;

enum Testovie{
    First, Second
}
#[derive(Serialize)]
struct Params <'a>{
    latitude: f64,
    longitude: f64,
    current: &'a str,
    timezone: &'a str
}
#[derive(Deserialize,Debug)]
struct Current<'a>{
    time: &'a str,
    temperature_2m: f32,
    apparent_temperature: f32,
}
#[derive(Deserialize,Debug)]
struct Weather <'a>{
    latitude: f32,
    longitude: f32,
    #[serde(borrow)]
    current: Current<'a>,
    timezone: &'a str
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let query_params = Params {
        latitude: 56.129057,
        longitude: 40.406635,
        current:  "temperature_2m,relative_humidity_2m,apparent_temperature,weather_code,wind_speed_10m",
        timezone: "auto"
    };
    // Запрос блокирует поток до получения ответа
    let json_buffer = client.get("https://api.open-meteo.com/v1/forecast?").query(&query_params)
    .send()?
    .text()?;
    let obj: Weather = serde_json::from_str(&json_buffer)?;
    println!("Время: {} Температура: {} Ощущается: {}", obj.current.time, obj.current.temperature_2m, obj.current.apparent_temperature);
    Ok(())
}


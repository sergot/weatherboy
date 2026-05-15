pub mod provider;

#[derive(Debug)]
pub struct WeatherSnapshot {
    time: String,
    temperature: f32,
    is_day: bool,
    wind_speed: f32,
    wind_direction: u16,
    precipitation: f32,
    weather_code: u8,
}

pub struct WeatherLocation {
    pub latitude: f64,
    pub longitude: f64,
}

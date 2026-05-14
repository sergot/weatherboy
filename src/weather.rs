mod provider;

pub struct WeatherSnapshot {
    time: String,
    temperature: f32,
    is_day: bool,
    wind_speed: f32,
    wind_direction: f32,
    precipitation: f32,
    weather_code: u8,
}

pub struct WeatherLocation {}

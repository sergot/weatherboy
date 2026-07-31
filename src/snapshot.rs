#[derive(Debug)]
pub struct WeatherSnapshot {
    pub time: String,
    pub temperature: f32,
    pub is_day: bool,
    pub wind_speed: f32,
    pub wind_direction: u16,
    pub precipitation: f32,
    pub cloud_cover: f32,
    pub weather_code: u8,
}

pub struct Weather {
    pub condition: WeatherCondition,
    pub wind: Wind,
}

pub enum WeatherCondition {
    Sunny,
    Cloudy,
    PartiallyCloudy,
    Rainy(f32), // XXX: worth changing f32 to "struct Milimeters(f32)" or something similar
}

// XXX: do we need direction here?
pub enum Wind {
    Windy { speed: f32 }, // XXX: same for f32 as for Rainy(f32)
    Calm,
}

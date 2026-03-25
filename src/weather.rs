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

pub enum Direction {
    N,
    NW,
    W,
    SW,
    S,
    SE,
    E,
    NE,
}

pub enum Wind {
    Windy { speed: f32, direction: Direction }, // XXX: same for f32 as for Rainy(f32)
    Calm,
}

impl Wind {
    pub fn horizontal_speed(&self) -> f32 {
        match self {
            Wind::Windy { speed, direction } => match direction {
                Direction::N | Direction::S => 0.0,
                Direction::E | Direction::NE | Direction::SE => -*speed,
                Direction::W | Direction::NW | Direction::SW => *speed,
            },
            Wind::Calm => 0.0,
        }
    }
}

pub struct Weather {
    pub cloud_cover: f32,
    precipitation: Option<Precipitation>,
    wind: Option<Wind>,
}

impl Weather {
    pub fn new(cloud_cover: f32, precipitation: Option<Precipitation>, wind: Option<Wind>) -> Self {
        Self {
            cloud_cover,
            precipitation,
            wind,
        }
    }

    pub fn wind_speed(&self) -> f32 {
        self.wind.as_ref().map_or(0.0, |w| w.horizontal_speed())
    }

    pub fn rainfall_intensity(&self) -> Option<f32> {
        let p = self.precipitation.as_ref()?;
        matches!(p.kind, PrecipitationKind::Rain).then(|| p.intensity())
    }
}

#[allow(dead_code)]
pub enum PrecipitationKind {
    Rain,
    Snow,
    Sleet,
    Hail,
}

pub struct Precipitation {
    pub kind: PrecipitationKind,
    pub rate: f32,
}

impl Precipitation {
    pub fn intensity(&self) -> f32 {
        (self.rate / 50.0).clamp(0.0, 1.0) // TODO: return a proper value within range[0.0-1.0]
    }
}

#[allow(dead_code)]
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

pub struct Wind {
    speed: f32,
    direction: Direction,
}

impl Wind {
    pub fn new(speed: f32, direction: Direction) -> Self {
        Self { speed, direction }
    }

    // follows meteorological convention: wind direction indicates where the wind blows *from*
    pub fn horizontal_speed(&self) -> f32 {
        match self.direction {
            Direction::N | Direction::S => 0.0,
            Direction::E | Direction::NE | Direction::SE => -self.speed,
            Direction::W | Direction::NW | Direction::SW => self.speed,
        }
    }
}

use crate::snapshot::WeatherSnapshot;

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

    pub fn from_snapshot(snapshot: WeatherSnapshot) -> Self {
        let cloud_cover = (snapshot.cloud_cover / 100.0).clamp(0.0, 1.0);

        let precipitation = (snapshot.precipitation > 0.0).then(|| Precipitation {
            kind: PrecipitationKind::from_weather_code(snapshot.weather_code),
            rate: snapshot.precipitation,
        });

        let wind = (snapshot.wind_speed > 0.0).then(|| {
            Wind::new(
                snapshot.wind_speed,
                Direction::from_degrees(snapshot.wind_direction),
            )
        });

        Self::new(cloud_cover, precipitation, wind)
    }
}

pub enum PrecipitationKind {
    Rain,
    Snow,
    Sleet,
    Hail,
}

impl PrecipitationKind {
    // WMO codes
    pub fn from_weather_code(code: u8) -> Self {
        match code {
            71 | 73 | 75 | 77 | 85 | 86 => PrecipitationKind::Snow,
            56 | 57 | 66 | 67 => PrecipitationKind::Sleet,
            96 | 99 => PrecipitationKind::Hail,
            _ => PrecipitationKind::Rain,
        }
    }
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

impl Direction {
    pub fn from_degrees(degrees: u16) -> Self {
        match ((degrees as f32 / 45.0).round() as u16) % 8 {
            0 => Direction::N,
            1 => Direction::NE,
            2 => Direction::E,
            3 => Direction::SE,
            4 => Direction::S,
            5 => Direction::SW,
            6 => Direction::W,
            7 => Direction::NW,
            _ => unreachable!(),
        }
    }
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

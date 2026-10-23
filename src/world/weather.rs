use crate::snapshot::WeatherSnapshot;

pub struct Weather {
    pub cloud_cover: f32,
    precipitation: Option<Precipitation>,
    wind: Wind,
}

impl Weather {
    pub fn new(cloud_cover: f32, precipitation: Option<Precipitation>, wind: Wind) -> Self {
        Self {
            cloud_cover,
            precipitation,
            wind,
        }
    }

    pub fn wind_speed(&self) -> f32 {
        self.wind.horizontal_speed()
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

        let wind = Wind::new(snapshot.wind_speed, snapshot.wind_direction as f32);

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

#[derive(Default)]
pub struct Wind {
    speed: f32,
    degrees: f32,
}

impl Wind {
    pub fn new(speed: f32, degrees: f32) -> Self {
        Self { speed, degrees }
    }

    // follows meteorological convention: wind direction indicates where the wind blows *from*
    pub fn horizontal_speed(&self) -> f32 {
        -self.speed * self.degrees.to_radians().sin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(270.0 => is almost_equal_to 10.0 precision 1e-4; "from west blows right")]
    #[test_case(90.0 => is almost_equal_to -10.0 precision 1e-4; "from east blows left")]
    #[test_case(0.0 => is almost_equal_to 0.0 precision 1e-4; "from north no sideways")]
    #[test_case(180.0 => is almost_equal_to 0.0 precision 1e-4; "from south no sideways")]
    #[test_case(199.0 => is almost_equal_to 3.2557 precision 1e-4; "slightly west of south drifts right")]
    fn horizontal_speed(degrees: f32) -> f32 {
        Wind::new(10.0, degrees).horizontal_speed()
    }
}

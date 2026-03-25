use rand::{rngs::ThreadRng, RngExt};

use crate::{
    cloud::{Cloud, CloudParams},
    point::Point,
    weather::{Weather, WeatherCondition, Wind},
};

pub struct World {
    pub weather: Weather,
    pub width: f32,
    pub height: f32,
    pub clouds: Vec<Cloud>,
    rng: ThreadRng,
}

impl World {
    pub fn new(width: f32, height: f32, weather: Weather) -> Self {
        let mut rng = rand::rng();
        let clouds = match &weather.condition {
            WeatherCondition::PartiallyCloudy => (0..5)
                .map(|i| {
                    let y = rng.random_range(5.0..height * 0.33);
                    let params = CloudParams::default();
                    Cloud::new(
                        &mut rng,
                        Point {
                            // TODO: 5.0 should be most probably computed based on WeatherCondition:
                            // maybe RenderParams or smt like that?
                            // meaning: how many clouds we should spawn...
                            x: -width / 5.0 * i as f32 - params.width,
                            y,
                        },
                        params,
                    )
                })
                .collect(),
            _ => {
                vec![]
            }
        };
        Self {
            width,
            height,
            weather,
            clouds,
            rng,
        }
    }

    pub fn tick(&mut self) {
        let wind_speed = self.weather.wind.horizontal_speed();
        if wind_speed != 0.0 {
            self.clouds.iter_mut().for_each(|cloud| {
                cloud.tick(wind_speed);
                if cloud.position.x > self.width + cloud.params.width {
                    cloud.regenerate(&mut self.rng);
                }
            });
        }
    }
}

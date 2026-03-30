use rand::{rngs::SmallRng, Rng, RngExt, SeedableRng};

use crate::{
    cloud::{Cloud, CloudParams},
    point::Point,
    weather::Weather,
};

pub struct World {
    weather: Weather,
    width: f32,
    height: f32,
    clouds: Vec<Cloud>,
    rng: SmallRng,
}

impl World {
    pub fn new(width: f32, height: f32, weather: Weather, seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let clouds = Self::spawn_clouds(&mut rng, width, height, weather.cloud_cover);
        Self {
            width,
            height,
            weather,
            clouds,
            rng,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.clouds = Self::spawn_clouds(&mut self.rng, width, height, self.weather.cloud_cover);
    }

    pub fn tick(&mut self) {
        let speed = self.weather.wind_speed();
        if speed == 0.0 {
            return;
        }

        self.move_clouds(speed);
    }

    pub fn clouds(&self) -> &[Cloud] {
        &self.clouds
    }

    pub fn weather(&self) -> &Weather {
        &self.weather
    }

    fn move_clouds(&mut self, speed: f32) {
        for cloud in self.clouds.iter_mut() {
            cloud.advance(speed);
            if cloud.left_edge() > self.width {
                let y = self.rng.random_range(0.0..self.height * 0.33);
                cloud.respawn(&mut self.rng, -cloud.width() / 2.0, y);
            } else if cloud.right_edge() < 0.0 {
                let y = self.rng.random_range(0.0..self.height * 0.33);
                cloud.respawn(&mut self.rng, self.width + cloud.width() / 2.0, y);
            }
        }
    }

    fn spawn_clouds(rng: &mut impl Rng, width: f32, height: f32, cloud_cover: f32) -> Vec<Cloud> {
        if cloud_cover == 0.0 {
            return vec![];
        }

        let params = CloudParams::default();
        let spacing = params.width * (1.0 - cloud_cover * 0.5).max(0.4);
        let x_start = -params.width / 2.0;
        let x_end = width + params.width / 2.0;
        let count = ((x_end - x_start) / spacing).ceil() as usize;

        (0..count)
            .map(|i| {
                let x = x_start + spacing * i as f32;
                let y = rng.random_range(0.0..height * 0.33);
                Cloud::new(rng, Point { x, y }, params)
            })
            .collect()
    }
}

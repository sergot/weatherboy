mod cloud;
mod scene;
mod sky;
mod weather;

use rand::{Rng, RngExt, SeedableRng, rngs::SmallRng};

pub use scene::Material;
use scene::Scene;
pub use weather::Weather;

use crate::{point::Point, snapshot::WeatherSnapshot};

use cloud::{Cloud, CloudParams};

const CLOUD_BAND: f32 = 0.33; // 1/3rd of view
const DRIFT_PER_KMH: f32 = 0.0025;

pub struct World {
    weather: Weather,
    aspect: f32,
    clouds: Vec<Cloud>,
    rng: SmallRng,
    scene: Scene,
}

impl World {
    pub fn new(aspect: f32, weather: Weather, seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let clouds = Self::spawn_clouds(&mut rng, aspect, weather.cloud_cover);
        Self {
            aspect,
            weather,
            clouds,
            rng,
            scene: Scene::hills(),
        }
    }

    pub fn from_snapshot(snapshot: WeatherSnapshot, aspect: f32, seed: u64) -> Self {
        Self::new(aspect, Weather::from_snapshot(snapshot), seed)
    }

    pub fn resize(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.clouds = Self::spawn_clouds(&mut self.rng, aspect, self.weather.cloud_cover);
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn tick(&mut self, dt: f32) {
        let velocity = cloud_drift(self.weather.wind_speed());
        if velocity == 0.0 {
            return;
        }

        self.move_clouds(velocity * dt);
    }

    pub fn weather(&self) -> &Weather {
        &self.weather
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    fn move_clouds(&mut self, dx: f32) {
        for cloud in self.clouds.iter_mut() {
            cloud.advance(dx);
            if cloud.left_edge() > self.aspect {
                let y = self.rng.random_range(0.0..CLOUD_BAND);
                cloud.respawn(&mut self.rng, -cloud.width() / 2.0, y);
            } else if cloud.right_edge() < 0.0 {
                let y = self.rng.random_range(0.0..CLOUD_BAND);
                cloud.respawn(&mut self.rng, self.aspect + cloud.width() / 2.0, y);
            }
        }
    }

    fn spawn_clouds(rng: &mut impl Rng, aspect: f32, cloud_cover: f32) -> Vec<Cloud> {
        if cloud_cover == 0.0 {
            return vec![];
        }

        let params = CloudParams::default();
        let spacing = params.width * (1.0 - cloud_cover * 0.5).max(0.4);
        let x_start = -params.width / 2.0;
        let x_end = aspect + params.width / 2.0;
        let count = ((x_end - x_start) / spacing).ceil() as usize;

        (0..count)
            .map(|i| {
                let x = x_start + spacing * i as f32;
                let y = rng.random_range(0.0..CLOUD_BAND);
                Cloud::new(rng, Point { x, y }, params)
            })
            .collect()
    }

    pub fn cloud_density_at(&self, p: Point) -> f32 {
        self.clouds
            .iter()
            .map(|cloud| cloud.density_at(p))
            .fold(0.0, f32::max) // XXX: is it worth using smin here?
    }
}

fn cloud_drift(wind_kmh: f32) -> f32 {
    wind_kmh * DRIFT_PER_KMH
}

#[cfg(test)]
mod tests {
    use super::*;
    use weather::Wind;

    #[test]
    fn drift_is_proportional_to_wind() {
        assert_eq!(cloud_drift(0.0), 0.0);
        assert_eq!(cloud_drift(20.0), 20.0 * DRIFT_PER_KMH);
        assert_eq!(cloud_drift(-20.0), -cloud_drift(20.0));
    }

    #[test]
    fn tick_moves_clouds_by_velocity_times_dt() {
        let wind = Wind::new(20.0, 270.0); // from the west: blows to the right
        let mut world = World::new(16.0 / 9.0, Weather::new(0.5, None, wind), 42);
        let before = world.clouds[0].left_edge();

        world.tick(0.5);

        let moved = world.clouds[0].left_edge() - before;
        assert!((moved - cloud_drift(20.0) * 0.5).abs() < 1e-6);
    }
}

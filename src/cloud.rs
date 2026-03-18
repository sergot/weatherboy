use rand::{Rng, RngExt};

use crate::{braille::sdf_density, circle::Circle, point::Point};

pub struct Cloud {
    pub(crate) position: Point,
    pub(crate) params: CloudParams,
    circles: Vec<Circle>,
}

impl Cloud {
    pub fn new(rng: &mut impl Rng, position: Point, params: CloudParams) -> Self {
        Self {
            circles: Self::generate_lobes(rng, &params),
            position,
            params,
        }
    }

    fn generate_lobes(rng: &mut impl Rng, params: &CloudParams) -> Vec<Circle> {
        (0..params.lobes)
            .map(|_| {
                let x = rng.random_range(-params.width..=params.width);
                let y = rng
                    .random_range(-params.height..=params.height)
                    .min(rng.random_range(-params.height..=params.height));
                let radius = rng.random_range(1.0..=params.height);
                Circle {
                    center: Point { x, y },
                    radius,
                }
            })
            .collect()
    }

    pub fn tick(&mut self, speed: f32) {
        self.position.x += speed;
    }

    pub fn density_at(&self, p: Point) -> f32 {
        let local_p = Point {
            x: p.x - self.position.x,
            y: p.y - self.position.y,
        };
        sdf_density(local_p, &self.circles, self.params.smoothness)
    }

    pub(crate) fn regenerate(&mut self, rng: &mut impl Rng) {
        self.position.x = -self.params.width;
        self.circles = Cloud::generate_lobes(rng, &self.params);
    }
}

#[derive(Clone, Copy)]
pub struct CloudParams {
    pub width: f32,
    height: f32,
    lobes: u16,
    smoothness: f32,
}

impl Default for CloudParams {
    fn default() -> Self {
        Self {
            width: 40.0,
            height: 10.0,
            lobes: 10,
            smoothness: 3.0,
        }
    }
}

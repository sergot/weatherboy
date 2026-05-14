use rand::{Rng, RngExt};

use crate::{braille::sdf_density, circle::Circle, point::Point};

pub struct Cloud {
    position: Point,
    params: CloudParams,
    circles: Vec<Circle>,
}

impl Cloud {
    pub fn new(rng: &mut impl Rng, position: Point, params: CloudParams) -> Self {
        Self {
            position,
            params,
            circles: Self::generate_lobes(rng, &params),
        }
    }

    pub fn width(&self) -> f32 {
        self.params.width
    }

    fn generate_lobes(rng: &mut impl Rng, params: &CloudParams) -> Vec<Circle> {
        (0..params.lobes)
            .map(|_| {
                let center: f32 = rng.random::<f32>() - rng.random::<f32>();
                let x = center * params.width / 2.0;

                let t: f32 = rng.random::<f32>().powi(2);
                let y = -t * params.height;

                let max_r = params.height * (1.0 - center.powi(2) * 0.6);
                let radius = rng.random_range((max_r * 0.3)..=max_r);

                Circle {
                    center: Point { x, y },
                    radius,
                }
            })
            .collect()
    }

    pub fn density_at(&self, p: Point) -> f32 {
        let local_p = Point {
            x: p.x - self.position.x,
            y: p.y - self.position.y,
        };
        sdf_density(local_p, &self.circles, self.params.smoothness)
    }

    pub fn advance(&mut self, speed: f32) {
        self.position.x += speed;
    }

    pub fn left_edge(&self) -> f32 {
        self.position.x - self.params.width / 2.0
    }

    pub fn right_edge(&self) -> f32 {
        self.position.x + self.params.width / 2.0
    }

    pub fn respawn(&mut self, rng: &mut impl Rng, x: f32, y: f32) {
        self.position = Point { x, y };
        self.circles = Self::generate_lobes(rng, &self.params);
    }
}

#[derive(Clone, Copy)]
pub struct CloudParams {
    pub width: f32,
    pub height: f32,
    pub lobes: u16,
    pub smoothness: f32,
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

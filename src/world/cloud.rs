use rand::{Rng, RngExt};

use crate::{circle::Circle, point::Point};

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
        sdf_density(
            local_p,
            &self.circles,
            self.params.smoothness,
            self.params.edge_softness,
        )
    }

    pub fn advance(&mut self, dx: f32) {
        self.position.x += dx;
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
    pub edge_softness: f32,
}

impl Default for CloudParams {
    fn default() -> Self {
        Self {
            width: 0.2,
            height: 0.05,
            lobes: 10,
            smoothness: 0.015,
            edge_softness: 0.005,
        }
    }
}

fn sdf_circle(p: Point, circle: &Circle) -> f32 {
    ((p.x - circle.center.x).powi(2) + (p.y - circle.center.y).powi(2)).sqrt() - circle.radius
}

fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = (k - (a - b).abs()).max(0.0) / k;
    a.min(b) - h * h * k * 0.25
}

// 1.0 deep inside, 0.5 on the edge, 0.0 far outside
fn sdf_density(p: Point, circles: &[Circle], k: f32, softness: f32) -> f32 {
    let final_sdf = circles
        .iter()
        .map(|c| sdf_circle(p, c))
        .fold(f32::INFINITY, |acc, sdf| smin(acc, sdf, k));

    1.0 / (1.0 + (final_sdf / softness).exp())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn single_circle_cloud(radius: f32) -> Cloud {
        Cloud {
            position: Point { x: 0.0, y: 0.0 },
            params: CloudParams::default(),
            circles: vec![Circle {
                center: Point { x: 0.0, y: 0.0 },
                radius,
            }],
        }
    }

    #[test_case(0.0 => is greater_than 0.99; "center is solid")]
    #[test_case(0.05 => is almost_equal_to 0.5 precision 1e-6; "edge is half")]
    #[test_case(0.1 => is less_than 0.01; "far outside is empty")]
    fn density_profile(x: f32) -> f32 {
        single_circle_cloud(0.05).density_at(Point { x, y: 0.0 })
    }
}

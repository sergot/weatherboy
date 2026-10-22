use crate::{point::Point, world::World};

const SKY_ZENITH: Rgb = [0.24, 0.45, 0.80];
const SKY_HORIZON: Rgb = [0.68, 0.80, 0.94];
const CLOUD_LIT: Rgb = [0.97, 0.97, 0.98];
const CLOUD_OVERCAST: Rgb = [0.55, 0.57, 0.62];

pub type Rgb = [f32; 3];

pub struct Sample {
    pub color: Rgb,
}

pub fn sample(world: &World, p: Point) -> Sample {
    let sky = lerp(SKY_ZENITH, SKY_HORIZON, p.y);
    let cloud = cloud_color(world.weather().cloud_cover);
    Sample {
        color: lerp(sky, cloud, world.cloud_density_at(p)),
    }
}

fn lerp(a: Rgb, b: Rgb, t: f32) -> Rgb {
    [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * t)
}

fn cloud_color(cover: f32) -> Rgb {
    lerp(CLOUD_LIT, CLOUD_OVERCAST, cover * 0.6)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::World;

    fn clear_sky() -> World {
        World::new(1.0, crate::world::Weather::new(0.0, None, None), 0)
    }

    #[test]
    fn sky_goes_from_zenith_to_horizon() {
        let world = clear_sky();
        assert_eq!(sample(&world, Point { x: 0.5, y: 0.0 }).color, SKY_ZENITH);
        assert_eq!(sample(&world, Point { x: 0.5, y: 1.0 }).color, SKY_HORIZON);
    }

    #[test]
    fn overcast_clouds_are_darker() {
        let brightness = |c: Rgb| c.iter().sum::<f32>();
        assert!(brightness(cloud_color(1.0)) < brightness(cloud_color(0.0)));
    }
}

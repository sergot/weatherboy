use crate::{
    point::Point,
    world::{Material, World},
};

const HAZE_DENSITY: f32 = 0.25;

const SKY_ZENITH: Rgb = [0.24, 0.45, 0.80];
const SKY_HORIZON: Rgb = [0.68, 0.80, 0.94];
const CLOUD_LIT: Rgb = [0.97, 0.97, 0.98];
const CLOUD_OVERCAST: Rgb = [0.55, 0.57, 0.62];

const MEADOW: Rgb = [0.18, 0.35, 0.12];
const HILL: Rgb = [0.30, 0.50, 0.28];

pub type Rgb = [f32; 3];

pub struct Sample {
    pub color: Rgb,
}

pub fn sample(world: &World, p: Point) -> Sample {
    if let Some(layer) = world.scene().layer_at(p) {
        return Sample {
            color: haze(material_color(layer.material), layer.depth),
        };
    }

    let sky = sky_color(p.y);
    let cloud = cloud_color(world.weather().cloud_cover);
    Sample {
        color: lerp(sky, cloud, world.cloud_density_at(p)),
    }
}

fn haze(color: Rgb, depth: f32) -> Rgb {
    lerp(color, SKY_HORIZON, 1.0 - (-HAZE_DENSITY * depth).exp())
}

fn sky_color(y: f32) -> Rgb {
    lerp(SKY_ZENITH, SKY_HORIZON, y)
}

pub fn lerp(a: Rgb, b: Rgb, t: f32) -> Rgb {
    [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * t)
}

fn cloud_color(cover: f32) -> Rgb {
    lerp(CLOUD_LIT, CLOUD_OVERCAST, cover * 0.6)
}

fn material_color(material: Material) -> Rgb {
    match material {
        Material::Meadow => MEADOW,
        Material::Hill => HILL,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{Weather, World};

    #[test]
    fn sky_goes_from_zenith_to_horizon() {
        assert_eq!(sky_color(0.0), SKY_ZENITH);
        assert_eq!(sky_color(1.0), SKY_HORIZON);
    }

    #[test]
    fn bottom_of_world_is_meadow() {
        let world = World::new(1.0, Weather::new(0.0, None, Default::default()), 0);
        assert_eq!(sample(&world, Point { x: 0.5, y: 0.99 }).color, MEADOW);
    }

    #[test]
    fn overcast_clouds_are_darker() {
        let brightness = |c: Rgb| c.iter().sum::<f32>();
        assert!(brightness(cloud_color(1.0)) < brightness(cloud_color(0.0)));
    }

    #[test]
    fn no_haze_close_up() {
        assert_eq!(haze(HILL, 0.0), HILL);
    }

    #[test]
    fn farther_is_closer_to_horizon() {
        let gap = |c: Rgb| (0..3).map(|i| (c[i] - SKY_HORIZON[i]).abs()).sum::<f32>();
        assert!(gap(haze(HILL, 3.0)) < gap(haze(HILL, 1.0)));
    }
}

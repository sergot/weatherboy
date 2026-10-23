use crate::{
    appearance::{self, Rgb, lerp},
    render::{
        PixelStyle, SampleGrid,
        braille::bayer,
        pixel::{PixelImage, View, to_bytes},
    },
    world::World,
};

const BACKGROUND: Rgb = [0.04, 0.05, 0.08];
const PITCH: usize = 8;
const LEVELS: u8 = 3;

// < 0.5 leaves a gap between dots
const MAX_RADIUS: f32 = 0.45;

fn radius(level: u8) -> f32 {
    MAX_RADIUS * PITCH as f32 * (level as f32 / LEVELS as f32).sqrt()
}

fn disc_alpha(dist: f32, radius: f32) -> f32 {
    (radius + 0.5 - dist).clamp(0.0, 1.0)
}

pub struct DotsStyle {
    image: PixelImage,
    dots: Vec<Dot>,
}

impl DotsStyle {
    pub fn new() -> Self {
        Self {
            image: PixelImage {
                width: 0,
                height: 0,
                rgb: Vec::new(),
            },
            dots: Vec::new(),
        }
    }
}

impl PixelStyle for DotsStyle {
    fn draw(&mut self, world: &World, view: &View) -> &PixelImage {
        // XXX: leftover pixels (< PITCH) at the edge might stay background, review later
        let (dots_w, dots_h) = (view.px_width / PITCH, view.px_height / PITCH);
        let grid = SampleGrid {
            width: dots_w,
            height: dots_h,
            aspect: world.aspect(),
        };

        self.dots.clear();
        for y in 0..dots_h {
            for x in 0..dots_w {
                let color = appearance::sample(world, grid.to_world(x, y)).color;
                self.dots.push(dot(color, x, y));
            }
        }

        let image = &mut self.image;
        (image.width, image.height) = (view.px_width, view.px_height);
        image.rgb.clear();
        for py in 0..image.height {
            for px in 0..image.width {
                let (x, y) = (px / PITCH, py / PITCH);
                let color = if x < dots_w && y < dots_h {
                    let d = self.dots[y * dots_w + x];
                    let centre = |i: usize| (i as f32 + 0.5) * PITCH as f32;
                    let dist = (px as f32 + 0.5 - centre(x)).hypot(py as f32 + 0.5 - centre(y));
                    lerp(BACKGROUND, d.ink, disc_alpha(dist, radius(d.level)))
                } else {
                    BACKGROUND
                };
                image.rgb.extend(to_bytes(color));
            }
        }

        image
    }
}

#[derive(Clone, Copy)]
struct Dot {
    level: u8,
    ink: Rgb,
}

fn dot(color: Rgb, x: usize, y: usize) -> Dot {
    let v = color[0].max(color[1]).max(color[2]).min(1.0);
    if v <= 0.0 {
        return Dot {
            level: 0,
            ink: BACKGROUND,
        };
    }

    let t = v * LEVELS as f32;
    let lower = t.floor();
    let up = t - lower > bayer(x % 4, y % 4);

    Dot {
        level: lower as u8 + up as u8,
        ink: color.map(|c| c / v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ink_is_hue_at_full_brightness() {
        assert_eq!(dot([0.4, 0.2, 0.0], 0, 0).ink, [1.0, 0.5, 0.0]);
    }

    #[test]
    fn black_is_no_dot() {
        assert_eq!(dot([0.0, 0.0, 0.0], 0, 0).level, 0);
    }

    #[test]
    fn full_brightness_is_largest_dot_everywhere() {
        for y in 0..4 {
            for x in 0..4 {
                assert_eq!(dot([1.0, 1.0, 1.0], x, y).level, LEVELS);
            }
        }
    }

    #[test]
    fn dithering_averages_to_brightness() {
        let total: u32 = (0..4)
            .flat_map(|y| (0..4).map(move |x| dot([0.5, 0.5, 0.5], x, y).level as u32))
            .sum();
        assert_eq!(total as f32 / 16.0, 1.5);
    }

    #[test]
    fn radius_grows_by_area() {
        assert_eq!(radius(0), 0.0);
        assert_eq!(radius(LEVELS), MAX_RADIUS * PITCH as f32);

        let area = |l| radius(l).powi(2);
        assert!((area(1) / area(LEVELS) - 1.0 / LEVELS as f32).abs() < 1e-6);
    }

    #[test]
    fn disc_edge_is_soft() {
        assert_eq!(disc_alpha(0.0, 3.0), 1.0); // centre
        assert_eq!(disc_alpha(3.0, 3.0), 0.5); // edge
        assert_eq!(disc_alpha(5.0, 3.0), 0.0); // outside
    }
}

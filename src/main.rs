#![allow(unused)]
mod app;
mod world;

use std::{array, f32};

use color_eyre::eyre::Result;
use rand::{Rng, RngExt};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use terminal_size::{terminal_size, Height, Width};

use crate::{app::App, world::World};

#[derive(Debug)]
struct Weather {
    condition: WeatherCondition,
    wind: Wind,
}

#[derive(Debug)]
enum WeatherCondition {
    Sunny,
    Cloudy,
    // PartiallyCloudy,
    Rainy(f32), // XXX: worth changing f32 to "struct Milimeters(f32)" or something similar
}

#[derive(Debug)]
enum Direction {
    N,
    // NE,
    // E,
    // SE,
    // S,
    // SW,
    // W,
    // NW,
}

#[derive(Debug)]
enum Wind {
    Windy { speed: f32, direction: Direction }, // XXX: same for f32 as for Rainy(f32)
    Calm,
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f32,
}
#[derive(Debug)]
struct Cloud {
    circles: Vec<Circle>,
    smoothness: f32,
}

struct WeatherView<'a> {
    world: &'a World,
}

impl Widget for WeatherView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!();
    }
}

impl Cloud {
    fn new(rng: &mut impl Rng, position: &Point, params: CloudParams) -> Self {
        let circles = (0..params.lobes)
            .map(move |_| {
                let x = rng.random_range(position.x - params.width..=position.x + params.width);
                let y = rng
                    .random_range(position.y - params.height..=position.y + params.height)
                    .min(rng.random_range(position.y - params.height..=position.y + params.height));
                let radius = rng.random_range(1.0..params.height);
                Circle {
                    center: Point { x, y },
                    radius,
                }
            })
            .collect();

        Self {
            circles,
            smoothness: params.smoothness,
        }
    }
}

struct CloudParams {
    width: f32,
    height: f32,
    lobes: u16,
    smoothness: f32,
}

impl Default for CloudParams {
    fn default() -> Self {
        Self {
            width: 20.0,
            height: 20.0,
            lobes: 4,
            smoothness: 4.0,
        }
    }
}

const BAYER: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];
fn bayer(x: usize, y: usize) -> f32 {
    assert!(x < BAYER.len());
    assert!(y < BAYER.len());
    BAYER[y][x] as f32 / 16.0
}

fn sdf_circle(p: &Point, circle: &Circle) -> f32 {
    ((p.x - circle.center.x).powi(2) + (p.y - circle.center.y).powi(2)).sqrt() - circle.radius
}

fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = (k - (a - b).abs()).max(0.0) / k;
    a.min(b) - h * h * k * 0.25
}

fn sdf_density(p: &Point, circles: &[Circle], k: f32) -> f32 {
    let final_sdf = circles
        .iter()
        .map(|c| sdf_circle(p, c))
        .fold(f32::INFINITY, |acc, sdf| smin(acc, sdf, k));

    1.0 / (1.0 + final_sdf.exp())
}

fn render_cloud(term_width: u16, term_height: u16) {
    println!("Drawing cloud on {term_width} x {term_height} terminal");

    let mut rng = rand::rng();
    let cloud = Cloud::new(
        &mut rng,
        &Point { x: 50.0, y: 50.0 },
        CloudParams::default(),
    );

    for row in 0..term_height {
        for col in 0..term_width {
            let base_wx = col * 2;
            let base_wy = row * 4;

            let dots: [[bool; 2]; 4] = array::from_fn(|i| {
                array::from_fn(|j| {
                    let dot_x = (base_wx as usize) + j;
                    let dot_y = (base_wy as usize) + i;
                    sdf_density(
                        &Point {
                            x: dot_x as f32,
                            y: dot_y as f32,
                        },
                        &cloud.circles,
                        cloud.smoothness,
                    ) > bayer(dot_x % 4, dot_y % 4).max(0.1)
                })
            });

            let braille = dots_to_braille(dots);
            print!("{braille}");
        }
        println!();
    }
}

fn render_sun(term_width: u16, term_height: u16) {
    let center_x = 1.0;
    let center_y = 1.0;
    let radius = 5.0;

    println!(
        "Drawing sun ({center_x}, {center_y}) r = {radius} on {term_width} x {term_height} terminal"
    );

    for row in 0..term_height {
        for col in 0..term_width {
            let dx = (col as f32 - center_x) * 1.0;
            let dy = (row as f32 - center_y) * 1.0;
            let distance = (dx * dx + dy * dy).sqrt();
            if (distance - radius).abs() < 0.5 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn dots_to_braille(dots: [[bool; 2]; 4]) -> char {
    const BITS: [[u32; 2]; 4] = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];

    let braille_base: u32 = 0x2800;

    let mask = dots
        .into_iter()
        .zip(BITS)
        .flat_map(|(a, b)| a.into_iter().zip(b))
        .fold(
            0,
            |bitmask, (dot, bit)| {
                if dot {
                    bitmask | bit
                } else {
                    bitmask
                }
            },
        );

    char::from_u32(braille_base | mask).unwrap()
}

fn main() -> Result<()> {
    let terminal = ratatui::init();
    color_eyre::install()?;

    let size = terminal.size()?;
    let mut app = App::new(size.width as f32, size.height as f32);
    app.run(terminal);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::array;
    use test_case::test_case;

    fn single_dot(row: usize, col: usize) -> [[bool; 2]; 4] {
        array::from_fn(|x| array::from_fn(|y| x == row && y == col))
    }

    #[test_case(single_dot(0,0) => 0x2800 | 0x01; "top left")]
    #[test_case(single_dot(0,1) => 0x2800 | 0x08; "top right")]
    #[test_case(single_dot(1,0) => 0x2800 | 0x02; "2nd row left")]
    #[test_case(single_dot(1,1) => 0x2800 | 0x10; "2nd row right")]
    #[test_case(single_dot(2,0) => 0x2800 | 0x04; "3rd row left")]
    #[test_case(single_dot(2,1) => 0x2800 | 0x20; "3rd row right")]
    #[test_case(single_dot(3,0) => 0x2800 | 0x40; "bottom left")]
    #[test_case(single_dot(3,1) => 0x2800 | 0x80; "bottom right")]
    #[test_case(array::from_fn(|_| array::from_fn(|_| true)) => 0x28ff; "all")]
    fn dot_to_braille_mapping(dots: [[bool; 2]; 4]) -> u32 {
        dots_to_braille(dots) as u32
    }
}

use crate::{circle::Circle, point::Point};

pub const BRAILLE_COLS_PER_CELL: f32 = 2.0;
pub const BRAILLE_ROWS_PER_CELL: f32 = 4.0;

const BAYER: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];
pub fn bayer(x: usize, y: usize) -> f32 {
    assert!(x < BAYER.len());
    assert!(y < BAYER.len());
    BAYER[y][x] as f32 / 16.0
}

fn sdf_circle(p: Point, circle: &Circle) -> f32 {
    ((p.x - circle.center.x).powi(2) + (p.y - circle.center.y).powi(2)).sqrt() - circle.radius
}

fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = (k - (a - b).abs()).max(0.0) / k;
    a.min(b) - h * h * k * 0.25
}

pub fn sdf_density(p: Point, circles: &[Circle], k: f32) -> f32 {
    let final_sdf = circles
        .iter()
        .map(|c| sdf_circle(p, c))
        .fold(f32::INFINITY, |acc, sdf| smin(acc, sdf, k));

    1.0 / (1.0 + final_sdf.exp())
}

pub fn dots_to_braille(dots: [[bool; 2]; 4]) -> char {
    const BITS: [[u32; 2]; 4] = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];

    let braille_base: u32 = 0x2800;

    let mask = dots
        .into_iter()
        .zip(BITS)
        .flat_map(|(a, b)| a.into_iter().zip(b))
        .fold(
            0,
            |bitmask, (dot, bit)| {
                if dot { bitmask | bit } else { bitmask }
            },
        );

    char::from_u32(braille_base | mask).unwrap()
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

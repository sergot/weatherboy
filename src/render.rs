mod braille;
mod kitty;
mod pixel;
mod smooth;
mod terminal;

use crate::{point::Point, world::World};
use color_eyre::eyre::Result;

pub use kitty::KittyRenderer;
pub use pixel::PixelStyle;
pub use smooth::SmoothStyle;
pub use terminal::TerminalRenderer;

pub trait Renderer {
    // physical width / height of the drawing area; the world is that wide and 1.0 tall
    fn aspect(&self) -> Result<f32>;

    fn render(&mut self, world: &World) -> Result<()>;

    fn tick(&mut self);

    fn resize(&mut self, cols: u16, rows: u16) -> Result<()>;
}

pub enum InputEvent {
    Quit,
    Resize { cols: u16, rows: u16 },
}

const FALLBACK_CELL_SIZE: (f32, f32) = (8.0, 16.0);

pub fn cell_size() -> Result<(f32, f32)> {
    let size = crossterm::terminal::window_size()?;
    let reported = size.width > 0 && size.height > 0 && size.columns > 0 && size.rows > 0;
    Ok(if reported {
        (
            size.width as f32 / size.columns as f32,
            size.height as f32 / size.rows as f32,
        )
    } else {
        FALLBACK_CELL_SIZE
    })
}

pub fn terminal_aspect(cols: u16, rows: u16) -> Result<f32> {
    Ok(aspect_of(cols, rows, cell_size()?))
}

fn aspect_of(cols: u16, rows: u16, (cell_w, cell_h): (f32, f32)) -> f32 {
    (cols as f32 * cell_w) / (rows.max(1) as f32 * cell_h)
}

pub struct SampleGrid {
    pub width: usize,
    pub height: usize,
    pub aspect: f32,
}

impl SampleGrid {
    pub fn to_world(&self, x: usize, y: usize) -> Point {
        Point {
            x: (x as f32 + 0.5) / self.width as f32 * self.aspect,
            y: (y as f32 + 0.5) / self.height as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(80, 24, (10.0, 20.0) => is almost_equal_to 80.0 * 10.0 / (24.0 * 20.0) precision 1e-6; "reported cell size")]
    #[test_case(80, 40, FALLBACK_CELL_SIZE => is almost_equal_to 1.0 precision 1e-6; "fallback cells are 1:2")]
    #[test_case(80, 0, FALLBACK_CELL_SIZE => is almost_equal_to 40.0 precision 1e-6; "zero rows don't divide by zero")]
    fn aspect(cols: u16, rows: u16, cell_size: (f32, f32)) -> f32 {
        aspect_of(cols, rows, cell_size)
    }

    #[test_case(0, 0 => (0.25, 0.25); "first sample")]
    #[test_case(3, 1 => (1.75, 0.75); "last sample")]
    fn sample_to_world(x: usize, y: usize) -> (f32, f32) {
        let grid = SampleGrid {
            width: 4,
            height: 2,
            aspect: 2.0,
        };
        let p = grid.to_world(x, y);
        (p.x, p.y)
    }

    #[test]
    fn non_square_samples_keep_proportions() {
        let grid = SampleGrid {
            width: 8,
            height: 4,
            aspect: 1.0,
        };
        let (a, b, c) = (
            grid.to_world(0, 0),
            grid.to_world(2, 0),
            grid.to_world(0, 1),
        );
        assert!(((b.x - a.x) - (c.y - a.y)).abs() < 1e-6);
    }
}

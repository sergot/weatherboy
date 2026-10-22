use crate::world::World;

pub struct PixelImage {
    pub width: usize,
    pub height: usize,
    pub rgb: Vec<u8>,
}

pub trait PixelStyle {
    fn draw(&mut self, world: &World, view: &View) -> &PixelImage;
}

pub struct View {
    pub cols: u16,
    pub rows: u16,
    pub px_width: usize,
    pub px_height: usize,
}

impl View {
    pub fn new(cols: u16, rows: u16, (cell_w, cell_h): (f32, f32)) -> Self {
        Self {
            cols,
            rows,
            px_width: (cols as f32 * cell_w).round() as usize,
            px_height: (rows as f32 * cell_h).round() as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::render::pixel::View;

    #[test]
    fn view_size_in_pixels() {
        let view = View::new(10, 5, (8.0, 16.0));
        assert_eq!((view.px_width, view.px_height), (80, 80));
    }
}

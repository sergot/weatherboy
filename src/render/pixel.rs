use crate::world::World;

pub struct PixelImage {
    pub width: usize,
    pub height: usize,
    pub rgb: Vec<u8>,
}

pub trait PixelStyle {
    fn draw(&mut self, world: &World, cols: u16, rows: u16) -> &PixelImage;
}

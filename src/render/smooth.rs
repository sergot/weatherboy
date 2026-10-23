use crate::{
    appearance,
    render::{
        SampleGrid,
        pixel::{PixelImage, PixelStyle, View, to_bytes},
    },
    world::World,
};

const PIXELS_PER_COL: usize = 2;
const PIXELS_PER_ROW: usize = 4;

pub struct SmoothStyle {
    image: PixelImage,
}

impl SmoothStyle {
    pub fn new() -> Self {
        Self {
            image: PixelImage {
                width: 0,
                height: 0,
                rgb: Vec::new(),
            },
        }
    }
}

impl PixelStyle for SmoothStyle {
    fn draw(&mut self, world: &World, view: &View) -> &PixelImage {
        let (width, height) = (
            view.cols as usize * PIXELS_PER_COL,
            view.rows as usize * PIXELS_PER_ROW,
        );
        let grid = SampleGrid {
            width,
            height,
            aspect: world.aspect(),
        };

        let image = &mut self.image;
        (image.width, image.height) = (width, height);
        image.rgb.clear();
        image.rgb.reserve(width * height * 3);

        for y in 0..height {
            for x in 0..width {
                let color = appearance::sample(world, grid.to_world(x, y)).color;
                image.rgb.extend(to_bytes(color));
            }
        }

        image
    }
}

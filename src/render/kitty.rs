mod protocol;

use std::io::{stdout, Stdout, Write};

use color_eyre::eyre::Result;
use crossterm::{cursor, execute, queue, terminal};

use crate::{
    render::{Renderer, SampleGrid},
    world::World,
};

use protocol::Placement;

const PIXELS_PER_COL: usize = 2;
const PIXELS_PER_ROW: usize = 4;

const IMAGE_ID: u32 = 1;
const PLACEMENT_ID: u32 = 1;

type Rgb = [f32; 3];

const SKY_ZENITH: Rgb = [0.24, 0.45, 0.80];
const SKY_HORIZON: Rgb = [0.68, 0.80, 0.94];
const CLOUD_LIT: Rgb = [0.97, 0.97, 0.98];
const CLOUD_OVERCAST: Rgb = [0.55, 0.57, 0.62];

pub struct KittyRenderer {
    out: Stdout,
    cols: u16,
    rows: u16,
    rgb: Vec<u8>,
}

impl KittyRenderer {
    pub fn new() -> Result<Self> {
        let (cols, rows) = terminal::size()?;
        let mut out = stdout();
        terminal::enable_raw_mode()?;
        execute!(
            out,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All)
        )?;
        Ok(Self {
            out,
            cols,
            rows,
            rgb: Vec::new(),
        })
    }

    fn image_size(&self) -> (usize, usize) {
        (
            self.cols as usize * PIXELS_PER_COL,
            self.rows as usize * PIXELS_PER_ROW,
        )
    }
}

impl Drop for KittyRenderer {
    fn drop(&mut self) {
        let _ = self.out.write_all(&protocol::delete_all());
        let _ = execute!(self.out, cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

impl Renderer for KittyRenderer {
    fn aspect(&self) -> Result<f32> {
        super::terminal_aspect(self.cols, self.rows)
    }

    fn render(&mut self, world: &World) -> Result<()> {
        let (width, height) = self.image_size();
        let grid = SampleGrid {
            width,
            height,
            aspect: world.aspect(),
        };
        self.rgb.clear();
        self.rgb.reserve(width * height * 3);

        let overcast = world.weather().cloud_cover * 0.6;
        let cloud_color = lerp(CLOUD_LIT, CLOUD_OVERCAST, overcast);

        for y in 0..height {
            let sky = lerp(SKY_ZENITH, SKY_HORIZON, y as f32 / height as f32);
            for x in 0..width {
                let p = grid.to_world(x, y);
                let density = world
                    .clouds()
                    .iter()
                    .map(|cloud| cloud.density_at(p))
                    .fold(0.0, f32::max);
                let color = lerp(sky, cloud_color, density);
                self.rgb
                    .extend(color.map(|c| (c.clamp(0.0, 1.0) * 255.0).round() as u8));
            }
        }

        let placement = Placement {
            image_id: IMAGE_ID,
            placement_id: PLACEMENT_ID,
            cols: self.cols,
            rows: self.rows,
        };
        queue!(self.out, cursor::MoveTo(0, 0))?;
        self.out.write_all(&protocol::transmit_and_display(
            &self.rgb, width, height, &placement,
        ))?;
        self.out.flush()?;
        Ok(())
    }

    fn tick(&mut self) {}

    fn resize(&mut self, cols: u16, rows: u16) -> Result<()> {
        (self.cols, self.rows) = (cols, rows);
        self.out.write_all(&protocol::delete_all())?;
        execute!(self.out, terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }
}

fn lerp(a: Rgb, b: Rgb, t: f32) -> Rgb {
    [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * t)
}

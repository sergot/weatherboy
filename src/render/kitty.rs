mod protocol;

use std::io::{Stdout, Write, stdout};

use color_eyre::eyre::Result;
use crossterm::{cursor, execute, queue, terminal};

use crate::{
    render::{Renderer, pixel::PixelStyle},
    world::World,
};

use protocol::Placement;

const IMAGE_ID: u32 = 1;
const PLACEMENT_ID: u32 = 1;

pub struct KittyRenderer {
    out: Stdout,
    cols: u16,
    rows: u16,
    style: Box<dyn PixelStyle>,
}

impl KittyRenderer {
    pub fn new(style: Box<dyn PixelStyle>) -> Result<Self> {
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
            style,
        })
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
        let image = self.style.draw(world, self.cols, self.rows);

        let placement = Placement {
            image_id: IMAGE_ID,
            placement_id: PLACEMENT_ID,
            cols: self.cols,
            rows: self.rows,
        };
        queue!(self.out, cursor::MoveTo(0, 0))?;
        self.out.write_all(&protocol::transmit_and_display(
            &image.rgb,
            image.width,
            image.height,
            &placement,
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

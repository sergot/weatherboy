mod protocol;

use std::io::{Stdout, Write, stdout};

use color_eyre::eyre::Result;
use crossterm::{cursor, execute, queue, terminal};

use crate::{
    render::{
        Renderer,
        pixel::{PixelStyle, View},
    },
    world::World,
};

use protocol::Placement;

const IMAGE_ID: u32 = 1;
const PLACEMENT_ID: u32 = 1;

pub struct KittyRenderer {
    out: Stdout,
    view: View,
    style: Box<dyn PixelStyle>,
}

impl KittyRenderer {
    pub fn new(style: Box<dyn PixelStyle>) -> Result<Self> {
        let (cols, rows) = terminal::size()?;
        let view = View::new(cols, rows, super::cell_size()?);
        let mut out = stdout();
        terminal::enable_raw_mode()?;
        execute!(
            out,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All)
        )?;
        Ok(Self { out, view, style })
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
        super::terminal_aspect(self.view.cols, self.view.rows)
    }

    fn render(&mut self, world: &World) -> Result<()> {
        let image = self.style.draw(world, &self.view);

        let placement = Placement {
            image_id: IMAGE_ID,
            placement_id: PLACEMENT_ID,
            cols: self.view.cols,
            rows: self.view.rows,
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
        self.view = View::new(cols, rows, super::cell_size()?);
        self.out.write_all(&protocol::delete_all())?;
        execute!(self.out, terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }
}

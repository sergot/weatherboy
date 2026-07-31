mod braille;
mod terminal;

use crate::world::World;
use color_eyre::eyre::Result;

pub use terminal::TerminalRenderer;

pub trait Renderer {
    fn render(&mut self, world: &World) -> Result<()>;

    fn tick(&mut self);

    // return true to quit
    fn poll_input(&mut self, world: &mut World) -> Result<bool>;
}

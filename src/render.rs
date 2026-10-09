mod braille;
mod kitty;
mod terminal;

use std::time::Duration;

use crate::world::World;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub use kitty::KittyRenderer;
pub use terminal::TerminalRenderer;

pub trait Renderer {
    // world size in world units
    fn viewport_size(&self) -> Result<(f32, f32)>;

    fn render(&mut self, world: &World) -> Result<()>;

    fn tick(&mut self);

    // return true to quit
    fn poll_input(&mut self, world: &mut World) -> Result<bool>;
}

pub enum InputEvent {
    Quit,
    Resize { cols: u16, rows: u16 },
}

// XXX: input belongs to App, not to renderers; shared here until it moves there
pub fn next_input_event(timeout: Duration) -> Result<Option<InputEvent>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }

    Ok(match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(InputEvent::Quit),
            _ => None,
        },
        Event::Resize(cols, rows) => Some(InputEvent::Resize { cols, rows }),
        _ => None,
    })
}

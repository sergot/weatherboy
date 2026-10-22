use std::time::{Duration, Instant};

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::{
    render::{InputEvent, Renderer},
    world::World,
};

// prevents sleeping process to teleport everything on resume
const MAX_DT: Duration = Duration::from_millis(100);
const FPS: f64 = 30.0;

pub struct App {
    world: World,
    renderer: Box<dyn Renderer>,
}

impl App {
    pub fn new(world: World, renderer: Box<dyn Renderer>) -> Self {
        Self { world, renderer }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut last = Instant::now();
        loop {
            let frame_start = Instant::now();
            self.renderer.render(&self.world)?;
            let budget = Duration::from_secs_f64(1.0 / FPS);
            match next_input_event(budget.saturating_sub(frame_start.elapsed()))? {
                Some(InputEvent::Resize { cols, rows }) => {
                    self.renderer.resize(cols, rows)?;
                    self.world.resize(self.renderer.aspect()?);
                }
                Some(InputEvent::Quit) => break,
                None => {}
            }

            let now = Instant::now();
            let dt = now.duration_since(last).min(MAX_DT);
            last = now;

            self.renderer.tick();
            self.world.tick(dt.as_secs_f32());
        }
        Ok(())
    }
}

fn next_input_event(timeout: Duration) -> Result<Option<InputEvent>> {
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

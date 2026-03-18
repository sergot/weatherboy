use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{widgets::Widget, DefaultTerminal, Frame};

use crate::{world::World, WeatherView};

pub struct App {
    world: World,
    should_quit: bool,
}

impl App {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            world: World::new(width, height),
            should_quit: false,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            if self.should_quit {
                break;
            }

            self.world.tick();
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let widget = WeatherView { world: &self.world };
        frame.render_widget(widget, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_secs_f64(1.0 / 30.0))? {
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

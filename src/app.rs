use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    braille::{BRAILLE_COLS_PER_CELL, BRAILLE_ROWS_PER_CELL},
    weather::{Weather, WeatherCondition, Wind},
    weather_view::{WeatherView, WeatherViewState},
    world::World,
};

pub struct App {
    world: World,
    weather_view_state: WeatherViewState,
    should_quit: bool,
}

impl App {
    pub fn new(width: f32, height: f32) -> Self {
        let weather = Weather {
            condition: WeatherCondition::PartiallyCloudy,
            wind: Wind::Windy { speed: 1.0 },
        };
        Self {
            world: World::new(
                width * BRAILLE_COLS_PER_CELL,
                height * BRAILLE_ROWS_PER_CELL,
                weather,
            ),
            should_quit: false,
            weather_view_state: WeatherViewState::default(),
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            if self.should_quit {
                break;
            }

            self.weather_view_state.tick();
            self.world.tick();
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let widget = WeatherView {
            world: &self.world,
        };
        frame.render_stateful_widget(widget, frame.area(), &mut self.weather_view_state);
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_secs_f64(1.0 / 30.0))?
            && let Some(key) = event::read()?.as_key_press_event()
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                _ => {}
            }
        }

        Ok(())
    }
}

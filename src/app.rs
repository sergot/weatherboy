use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    braille::{BRAILLE_COLS_PER_CELL, BRAILLE_ROWS_PER_CELL},
    weather::{Direction, Precipitation, PrecipitationKind, Weather, Wind},
    weather_view::{WeatherView, WeatherViewState},
    world::World,
};

pub struct App {
    world: World,
    weather_view_state: WeatherViewState,
    should_quit: bool,
}

impl App {
    pub fn new(width: f32, height: f32, seed: u64) -> Self {
        let weather = Weather::new(
            0.1,
            Some(Precipitation {
                kind: PrecipitationKind::Rain,
                rate: 1.0,
            }),
            Some(Wind::new(1.0, Direction::E)),
        );
        Self {
            world: World::new(
                width * BRAILLE_COLS_PER_CELL,
                height * BRAILLE_ROWS_PER_CELL,
                weather,
                seed,
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
        let widget = WeatherView { world: &self.world };
        frame.render_stateful_widget(widget, frame.area(), &mut self.weather_view_state);
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_secs_f64(1.0 / 30.0))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    _ => {}
                },
                Event::Resize(w, h) => {
                    self.world.resize(
                        w as f32 * BRAILLE_COLS_PER_CELL,
                        h as f32 * BRAILLE_ROWS_PER_CELL,
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }
}

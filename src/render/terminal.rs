use std::{array, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::{
    point::Point,
    render::{
        Renderer,
        braille::{BRAILLE_COLS_PER_CELL, BRAILLE_ROWS_PER_CELL, bayer, dots_to_braille},
    },
    world::World,
};
use color_eyre::eyre::Result;

pub struct TerminalRenderer {
    terminal: DefaultTerminal,
    state: TerminalRendererState,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self {
            terminal: ratatui::init(),
            state: TerminalRendererState::default(),
        }
    }

    pub fn viewport_size(&self) -> Result<(f32, f32)> {
        let size = self.terminal.size()?;
        Ok((
            size.width as f32 * BRAILLE_COLS_PER_CELL,
            size.height as f32 * BRAILLE_ROWS_PER_CELL,
        ))
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

impl Renderer for TerminalRenderer {
    fn render(&mut self, world: &World) -> Result<()> {
        let state = &mut self.state;
        self.terminal.draw(|frame| {
            let widget = WeatherWidget { world };
            frame.render_stateful_widget(widget, frame.area(), state);
        })?;
        Ok(())
    }

    fn tick(&mut self) {
        self.state.tick_counter = self.state.tick_counter.wrapping_add(1);
    }

    fn poll_input(&mut self, world: &mut World) -> Result<bool> {
        if event::poll(Duration::from_secs_f64(1.0 / 30.0))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
                    _ => {}
                },
                Event::Resize(w, h) => {
                    world.resize(
                        w as f32 * BRAILLE_COLS_PER_CELL,
                        h as f32 * BRAILLE_ROWS_PER_CELL,
                    );
                }
                _ => {}
            }
        }

        Ok(false)
    }
}

#[derive(Default)]
struct TerminalRendererState {
    tick_counter: u16,
}

struct WeatherWidget<'a> {
    world: &'a World,
}

impl StatefulWidget for WeatherWidget<'_> {
    type State = TerminalRendererState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut TerminalRendererState) {
        self.render_sun(area, buf);
        self.render_rain(area, buf, state);
        self.render_clouds(area, buf);
    }
}

impl WeatherWidget<'_> {
    // fn render_sun(term_width: u16, term_height: u16) {
    //     let center_x = 1.0;
    //     let center_y = 1.0;
    //     let radius = 5.0;

    //     println!(
    //         "Drawing sun ({center_x}, {center_y}) r = {radius} on {term_width} x {term_height} terminal"
    //     );

    //     for row in 0..term_height {
    //         for col in 0..term_width {
    //             let dx = (col as f32 - center_x) * 1.0;
    //             let dy = (row as f32 - center_y) * 1.0;
    //             let distance = (dx * dx + dy * dy).sqrt();
    //             if (distance - radius).abs() < 0.5 {
    //                 print!("*");
    //             } else {
    //                 print!(" ");
    //             }
    //         }
    //         println!();
    //     }
    // }
    fn render_sun(&self, _area: Rect, buf: &mut Buffer) {
        buf[(1, 1)].set_char('S'); // TODO: render a proper sun
    }

    fn render_rain(&self, area: Rect, buf: &mut Buffer, state: &mut TerminalRendererState) {
        let Some(rain_intensity) = self.world.weather().rainfall_intensity() else {
            return;
        };

        let wind_speed = self.world.weather().wind_speed();
        let rain_char = match wind_speed {
            x if x < 0.0 => '/',
            x if x > 0.0 => '\\',
            _ => '\'',
        };
        for row in 0..area.height {
            for col in 0..area.width {
                let visible = Self::rain_hash(
                    col.wrapping_sub_signed((state.tick_counter as f32 * wind_speed) as i16),
                    row.wrapping_sub(state.tick_counter),
                );
                if visible > 1.0 - rain_intensity {
                    buf[(col, row)].set_char(rain_char);
                }
            }
        }
    }

    fn render_clouds(&self, area: Rect, buf: &mut Buffer) {
        for row in 0..(area.height as i32) {
            for col in 0..(area.width as i32) {
                let base_wx = col * 2;
                let base_wy = row * 4;

                let dots: [[bool; 2]; 4] = array::from_fn(|i| {
                    array::from_fn(|j| {
                        let dot_x = (base_wx as usize) + j;
                        let dot_y = (base_wy as usize) + i;
                        let density = self
                            .world
                            .clouds()
                            .iter()
                            .map(|cloud| {
                                cloud.density_at(Point {
                                    x: dot_x as f32,
                                    y: dot_y as f32,
                                })
                            })
                            .fold(0.0, f32::max); // XXX: is it worth using smin here?
                        density > bayer(dot_x % 4, dot_y % 4).max(0.1)
                    })
                });

                if !dots.into_iter().flatten().any(|b| b) {
                    continue;
                }

                let braille = dots_to_braille(dots);
                buf[(col as u16, row as u16)].set_char(braille);
            }
        }
    }

    fn rain_hash(col: u16, row: u16) -> f32 {
        let mut n = (col as u32)
            .wrapping_mul(1619)
            .wrapping_add((row as u32).wrapping_mul(31337));
        n = (n ^ (n >> 16)).wrapping_mul(0x45d9f3b);
        n = (n ^ (n >> 16)).wrapping_mul(0x45d9f3b);
        n = n ^ (n >> 16);
        n as f32 / u32::MAX as f32
    }
}

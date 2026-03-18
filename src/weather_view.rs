use std::array;

use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::{
    braille::{bayer, dots_to_braille},
    point::Point,
    world::World,
};

pub struct WeatherView<'a> {
    pub world: &'a World,
}

#[derive(Default)]
pub struct WeatherViewState {
    tick_counter: u16,
}

impl WeatherViewState {
    pub fn tick(&mut self) {
        self.tick_counter = self.tick_counter.wrapping_add(1);
    }
}

impl StatefulWidget for WeatherView<'_> {
    type State = WeatherViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut WeatherViewState) {
        self.render_sun(area, buf);
        self.render_rain(area, buf, state);
        self.render_clouds(area, buf);
    }
}

impl WeatherView<'_> {
    fn render_sun(&self, _area: Rect, buf: &mut Buffer) {
        buf[(1, 1)].set_char('S'); // TODO: render a proper sun
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
                            .clouds
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

    fn render_rain(&self, area: Rect, buf: &mut Buffer, state: &mut WeatherViewState) {
        for row in 0..area.height {
            for col in 0..area.width {
                // XXX: should tick counter be u16?
                let visible = Self::rain_hash(col, row.wrapping_sub(state.tick_counter));
                if visible > 0.98 {
                    buf[(col, row)].set_char('|'); // TODO: change to .,/\ etc based on how heavy is the rain and wind
                }
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

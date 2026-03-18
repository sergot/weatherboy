#![allow(unused)] // TODO: remove
mod app;
mod braille;
mod circle;
mod cloud;
mod point;
mod weather;
mod weather_view;
mod world;

use color_eyre::eyre::Result;

use crate::app::App;

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

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let size = terminal.size()?;
    let mut app = App::new(size.width as f32, size.height as f32);
    app.run(terminal)?;
    ratatui::restore();
    Ok(())
}

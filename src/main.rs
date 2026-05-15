mod app;
mod braille;
mod circle;
mod point;
mod weather;
mod weather_view;
mod world;

use clap::Parser;
use color_eyre::eyre::Result;

use crate::{app::App, weather::WeatherLocation};
use crate::weather::provider::{OpenMeteoProvider, WeatherProvider};

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

#[derive(Debug, Parser)]
#[command()]
struct Args {
    #[arg(long)]
    lat: f64,

    #[arg(long)]
    lon: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let location = WeatherLocation{
        latitude: args.lat,
        longitude: args.lon,
    };

    let provider = OpenMeteoProvider::new();
    let snapshot = provider.fetch_snapshot(location).await?;

    println!("{snapshot:#?}");
    // color_eyre::install()?;
    // let terminal = ratatui::init();

    // let size = terminal.size()?;
    // let mut app = App::new(size.width as f32, size.height as f32, rand::random());
    // app.run(terminal)?;
    // ratatui::restore();
    Ok(())
}

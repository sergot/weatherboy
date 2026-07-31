mod app;
mod circle;
mod point;
mod provider;
mod render;
mod snapshot;
mod world;

use clap::Parser;
use color_eyre::eyre::Result;

use crate::app::App;
use crate::provider::{OpenMeteoProvider, WeatherLocation, WeatherProvider};
use crate::render::TerminalRenderer;
use crate::world::World;

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
    color_eyre::install()?;

    let args = Args::parse();
    let location = WeatherLocation {
        latitude: args.lat,
        longitude: args.lon,
    };

    let provider = OpenMeteoProvider::new();
    let snapshot = provider.fetch_snapshot(location).await?;

    let renderer = TerminalRenderer::new();
    let (width, height) = renderer.viewport_size()?;
    let world = World::from_snapshot(snapshot, width, height, rand::random());

    let mut app = App::new(world, Box::new(renderer));
    app.run()
}

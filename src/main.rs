mod app;
mod circle;
mod point;
mod provider;
mod render;
mod snapshot;
mod world;

use clap::{Parser, ValueEnum};
use color_eyre::eyre::Result;

use crate::app::App;
use crate::provider::{OpenMeteoProvider, WeatherLocation, WeatherProvider};
use crate::render::{KittyRenderer, Renderer, TerminalRenderer};
use crate::world::World;

#[derive(Debug, Parser)]
#[command()]
struct Args {
    #[arg(long)]
    lat: f64,

    #[arg(long)]
    lon: f64,

    #[arg(long, value_enum, default_value_t = Backend::Braille)]
    backend: Backend,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Backend {
    Braille,
    Kitty,
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

    let renderer: Box<dyn Renderer> = match args.backend {
        Backend::Braille => Box::new(TerminalRenderer::new()),
        Backend::Kitty => Box::new(KittyRenderer::new()?),
    };
    let world = World::from_snapshot(snapshot, renderer.aspect()?, rand::random());

    let mut app = App::new(world, renderer);
    app.run()
}

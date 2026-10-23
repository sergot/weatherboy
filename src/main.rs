mod app;
mod appearance;
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
use crate::render::{
    DotsStyle, KittyRenderer, PixelStyle, Renderer, SmoothStyle, TerminalRenderer,
};
use crate::world::World;

#[derive(Debug, Parser)]
#[command()]
struct Args {
    #[arg(long)]
    lat: f64,

    #[arg(long)]
    lon: f64,

    #[arg(long, value_enum, default_value_t = Backend::Kitty)]
    backend: Backend,

    #[arg(long, value_enum, default_value_t = Style::Dots)]
    style: Style,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Backend {
    Braille,
    Kitty,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Style {
    Dots,
    Smooth,
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
        Backend::Kitty => {
            let style: Box<dyn PixelStyle> = match args.style {
                Style::Dots => Box::new(DotsStyle::new()),
                Style::Smooth => Box::new(SmoothStyle::new()),
            };
            Box::new(KittyRenderer::new(style)?)
        }
    };
    let world = World::from_snapshot(snapshot, renderer.aspect()?, rand::random());

    let mut app = App::new(world, renderer);
    app.run()
}

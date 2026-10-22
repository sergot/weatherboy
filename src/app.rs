use std::time::{Duration, Instant};

use color_eyre::eyre::Result;

use crate::{render::Renderer, world::World};

// prevents sleeping process to teleport everything on resume
const MAX_DT: Duration = Duration::from_millis(100);

pub struct App {
    world: World,
    renderer: Box<dyn Renderer>,
}

impl App {
    pub fn new(world: World, renderer: Box<dyn Renderer>) -> Self {
        Self { world, renderer }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut last = Instant::now();
        loop {
            self.renderer.render(&self.world)?;
            if self.renderer.poll_input(&mut self.world)? {
                break;
            }

            let now = Instant::now();
            let dt = now.duration_since(last).min(MAX_DT);
            last = now;

            self.renderer.tick();
            self.world.tick(dt.as_secs_f32());
        }
        Ok(())
    }
}

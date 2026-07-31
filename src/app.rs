use color_eyre::eyre::Result;

use crate::{render::Renderer, world::World};

pub struct App {
    world: World,
    renderer: Box<dyn Renderer>,
}

impl App {
    pub fn new(world: World, renderer: Box<dyn Renderer>) -> Self {
        Self { world, renderer }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.renderer.render(&self.world)?;
            if self.renderer.poll_input(&mut self.world)? {
                break;
            }
            self.renderer.tick();
            self.world.tick();
        }
        Ok(())
    }
}

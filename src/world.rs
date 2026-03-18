pub struct World {
    width: f32,
    height: f32,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn tick(&self) {
        todo!();
    }
}

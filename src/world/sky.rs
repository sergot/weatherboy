pub struct Sky {
    pub time: f32,
    pub sun: Sun,
    pub moon: Moon,
}

pub struct Sun {
    azimuth: f32,
    altitude: f32,
}
pub struct Moon {
    azimuth: f32,
    altitude: f32,
    phase: f32,
}

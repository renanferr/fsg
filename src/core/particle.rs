use super::vec2d::Vec2D;

pub struct Particle {
    pos: Vec2D,
    dir: Vec2D,
    speed: f32,
}

impl Particle {
    pub fn new(pos: Vec2D, dir: Vec2D, speed: f32) -> Particle {
        return Particle {
            pos: pos,
            dir: dir,
            speed: speed
        };
    }
}
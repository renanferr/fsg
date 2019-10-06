#[derive(Copy, Clone)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

// #[derive(Copy, Clone)]
impl Vec2D {
    pub fn new(x: f32, y: f32) -> Vec2D {
        return Vec2D { x: x, y: y }
    }
}
use macroquad::Vec2;

pub struct Commander {}

struct Turret {
    location: Vec2,
}

struct AntiMissile {
    location: Vec2,
    speed: Vec2,
    explosion_radius: f32,
}

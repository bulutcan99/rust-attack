use macroquad::math::Vec2;

/// Represents an anti-missile launched by a turret.
#[derive(Debug)]
pub struct AntiMissile {
    location: Vec2,
    /// Movement speed per frame, represented as a 2D vector.
    speed: Vec2,
    /// The radius within which the missile can cause an explosion.
    explosion_radius: u32,
}

impl AntiMissile {
    pub fn new(location: Vec2, speed: Vec2, explosion_radius: u32) -> Self {
        Self {
            location,
            speed,
            explosion_radius,
        }
    }
    pub fn explosion(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    pub fn draw(&self) {}
}

use macroquad::math::{bool, Vec2};

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

    //TODO: belirtilen hiz ve mouse ciktisina gore bulunacagi koordinata gore
    //target'i vurup vurmamasina gore true, false doncek
    pub fn hit(&self, target: Vec2, mouse: Vec2) -> Result<bool, anyhow::Error> {
        Ok(true)
    }

    fn explosion(&self) -> Result<(), anyhow::Error> {
        Ok(())
    }

    //TODO: her framede yeri guncellenecek
    pub fn draw(&self) {}
}

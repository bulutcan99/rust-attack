use arrayvec::ArrayVec;
use macroquad::math::{bool, Vec2};

use super::anti_missile::AntiMissile;

// The commander will command the turrets
#[derive(Debug)]
pub struct Turret {
    pub location: Vec2,
    muzzle_point: Vec2,
    anti_missiles: ArrayVec<AntiMissile, 1>,
    /// The maximum number of anti-missiles this turret can have in flight at a time.
    max_anti_missiles: u8,
    angle: f32,
}

impl Turret {
    pub fn fire(&self, target: Vec2) -> Result<bool, anyhow::Error> {
        Ok(true)
    }
}

use arrayvec::ArrayVec;
use macroquad::math::{bool, Vec2};

use super::anti_missile::AntiMissile;
//TODO: sirada turret var bundan sonrasinda anti_misisle'daki methotlara gore turreti
//sekillendirecegiz

// The commander will command the turrets
#[derive(Debug)]
pub struct Turret {
    pub location: Vec2,
    // We will have fire more than 1 anti-missiles simultaneously.
    anti_missiles: ArrayVec<AntiMissile, 1>,
    /// The maximum number of anti-missiles this turret can have in flight at a time.
    max_anti_missiles: u8,
}

impl Turret {
    pub fn new(
        location: Vec2,
        anti_missiles: ArrayVec<AntiMissile, 1>,
        max_anti_missiles: u8,
    ) -> Self {
        Self {
            location,
            anti_missiles,
            max_anti_missiles,
        }
    }

    fn is_angle_within_limits(
        location: Vec2,
        target: Vec2,
        min_angle: f32,
        max_angle: f32,
    ) -> bool {
        let vx = target.x - location.x;
        let vy = target.y - location.y;
        let theta = vy.atan2(vx).to_degrees();
        let theta_normalized = (theta + 360.0) % 360.0;
        (min_angle..=max_angle).contains(&theta_normalized)
    }

    // pub fn fire(&self, target: Vec2, mouse: Vec2) -> Result<bool, anyhow::Error> {
    //     // Check if we can fire another anti-missile
    //     if self.anti_missiles.len() as u8 >= self.max_anti_missiles {
    //         return Err(anyhow::anyhow!(
    //             "Cannot fire: Maximum anti-missiles in flight reached"
    //         ));
    //     }
    //
    //     //TODO: speedi verilen mouse ve target ve belirtilen sureye gore bir hesaplama
    //     //yapilip buna gore bir speed verecegiz (ya da speed default bir deger mi vermeliyiz?)
    //     let speed = Vec2::new(x, y);
    //     let anti_missile = AntiMissile::new(self.location);
    //     self.anti_missiles.push(anti_missile);
    //
    //     // Simulate a successful hit (this should be replaced with actual logic)
    //     Ok(true)
    // }

    pub fn draw(&self) {
        // Placeholder for drawing the turret
        // In a real implementation, you would use macroquad or another library to render the turret and anti-missiles
        println!(
            "Drawing turret at location ({}, {})",
            self.location.x, self.location.y
        );

        for missile in &self.anti_missiles {
            missile.draw();
        }
    }
}

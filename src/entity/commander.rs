use super::turret::Turret;
use arrayvec::ArrayVec;
use macroquad::math::Vec2;

//TODO: commander, turret ve anti-missile draw
//anti missilelara mouse ve target verilecek ona gore patlama yapacak
// First version will have one turret per commander
#[derive(Debug)]
pub struct Commander {
    // We will have more than 1 turret simultaneously.
    turrets: ArrayVec<Turret, 1>,
}

impl Commander {
    pub fn new(turrets: ArrayVec<Turret, 1>) -> Self {
        return Self { turrets };
    }
    pub fn draw(&self) {}

    // pub fn fire(&self, target: Vec2) -> Result<bool, anyhow::Error> {
    //     let mut reachable_found = false;
    //     let mouse = macroquad::prelude::mouse_position();
    //     let mouse_vec = Vec2::new(mouse.0, mouse.1);
    //     for turret in self.turrets.iter() {
    //         match self.target_reachable(turret.location, mouse_vec) {
    //             Ok(()) => {
    //                 reachable_found = true;
    //                 if turret.fire(target)? {
    //                     //TODO: Will calculate after if one turret can hit return true
    //                     return Ok(true); // Successfully hit
    //                 }
    //             }
    //             Err(_) => continue, // Skip unreachable
    //         }
    //     }
    //
    //     if reachable_found {
    //         Ok(false) // No turret could hit, but some were reachable
    //     } else {
    //         Err(anyhow::anyhow!("No turret can reach the target")) // No turret reachable
    //     }
    // }

    // Max 45 and 135 degrees are reachable
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

    fn target_reachable(&self, location: Vec2, mouse: Vec2) -> Result<(), anyhow::Error> {
        if Self::is_angle_within_limits(location, mouse, 45.0, 135.0) {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Target is not reachable"))
        }
    }
}

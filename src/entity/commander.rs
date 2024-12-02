use super::turret::Turret;
use arrayvec::ArrayVec;
use macroquad::math::Vec2;

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

    pub fn fire(&self, target: Vec2) -> Result<bool, anyhow::Error> {
        let mut reachable_found = false;
        for turret in self.turrets.iter() {
            match self.target_reachable(turret.location, target) {
                Ok(()) => {
                    reachable_found = true;
                    if turret.fire(target)? {
                        //TODO: Will calculate after if one turret can hit return true
                        return Ok(true); // Successfully hit
                    }
                }
                Err(_) => continue, // Skip unreachable
            }
        }

        if reachable_found {
            Ok(false) // No turret could hit, but some were reachable
        } else {
            Err(anyhow::anyhow!("No turret can reach the target")) // No turret reachable
        }
    }

    // Max 45 and 135 degrees are reachable
    fn target_reachable(&self, location: Vec2, target: Vec2) -> Result<(), anyhow::Error> {
        let vx = target.x - location.x;
        let vy = target.y - location.y;
        let theta = vy.atan2(vx);
        let theta_degree = theta.to_degrees();
        let theta_normalized = (theta_degree + 360.0) % 360.0;
        if (45.0..=135.0).contains(&theta_normalized) {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Target is not reachable"))
        }
    }
}

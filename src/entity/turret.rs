use arrayvec::ArrayVec;
use macroquad::math::Vec2;

use super::anti_missile::AntiMissile;

/// The turret will fire anti-missiles to defend.
#[derive(Debug)]
pub struct Turret {
    pub location: Vec2,
    // The turret can fire multiple anti-missiles simultaneously.
    anti_missiles: ArrayVec<AntiMissile, 1>,
    /// The maximum number of anti-missiles this turret can have in flight at a time.
    max_anti_missiles: u8,
    // Health of the turret (0-100, where 100 is full).
    health: u8,
}

impl Turret {
    /// Creates a new turret.
    pub fn new(location: Vec2, max_anti_missiles: u8) -> Self {
        Self {
            location,
            anti_missiles: ArrayVec::new(),
            max_anti_missiles,
            health: 100,
        }
    }

    /// Checks if a target is within the turret's firing range and angle.
    fn is_target_in_range(&self, target: Vec2, min_angle: f32, max_angle: f32) -> bool {
        let vx = target.x - self.location.x;
        let vy = target.y - self.location.y;
        let theta = vy.atan2(vx).to_degrees();
        let theta_normalized = (theta + 360.0) % 360.0;
        (min_angle..=max_angle).contains(&theta_normalized)
    }

    /// Fires an anti-missile towards the target if possible.
    pub fn aim_and_fire(&mut self, target: Vec2) -> bool {
        if self.anti_missiles.len() as u8 >= self.max_anti_missiles {
            return false; // Cannot fire more missiles than the limit.
        }

        let missile = AntiMissile::new(self.location, target);
        self.anti_missiles.push(missile);
        true
    }

    /// Updates the turret and its anti-missiles.
    pub fn update(&mut self, delta_time: f32, targets: &[Vec2]) {
        let mut alive_missiles = ArrayVec::<AntiMissile, 1>::new();

        for mut missile in self.anti_missiles.drain(..) {
            missile.update_position(delta_time);

            if missile.check_collision_general(targets) || !missile.report_alive() {
                continue;
            }

            // Retain missiles that are still alive.
            alive_missiles.push(missile);
        }

        self.anti_missiles = alive_missiles;
    }

    /// Reports the current health of the turret.
    pub fn report_health(&self) -> u8 {
        self.health
    }

    /// Reduces the turret's health by the specified amount.
    pub fn take_damage(&mut self, amount: u8) {
        self.health = self.health.saturating_sub(amount);
    }

    /// Draws the turret and its anti-missiles.
    pub fn draw(&self) {
        // Placeholder for drawing the turret.
        println!(
            "Drawing turret at location ({}, {}) with health {}",
            self.location.x, self.location.y, self.health
        );

        // Draw each anti-missile.
        for missile in &self.anti_missiles {
            missile.draw();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrayvec::ArrayVec;

    #[test]
    fn test_turret_new() {
        let turret = Turret::new(Vec2::new(0.0, 0.0), 3);
        assert_eq!(turret.location, Vec2::new(0.0, 0.0));
        assert_eq!(turret.max_anti_missiles, 3);
        assert_eq!(turret.health, 100);
        assert!(turret.anti_missiles.is_empty());
    }

    #[test]
    fn test_turret_fire() {
        let mut turret = Turret::new(Vec2::new(0.0, 0.0), 1);
        assert!(turret.aim_and_fire(Vec2::new(10.0, 10.0)));
        assert_eq!(turret.anti_missiles.len(), 1);

        // Exceeding the max anti-missile count.
        assert!(!turret.aim_and_fire(Vec2::new(30.0, 30.0)));
        assert_ne!(turret.anti_missiles.len(), 2);
    }

    #[test]
    fn test_turret_update() {
        let mut turret = Turret::new(Vec2::new(0.0, 0.0), 2);
        turret.aim_and_fire(Vec2::new(10.0, 0.0));

        let targets = vec![Vec2::new(10.0, 0.0)];
        turret.update(1.0, &targets);

        // The missile should explode and be removed after hitting a target.
        assert!(turret.anti_missiles.is_empty());
    }

    #[test]
    fn test_turret_take_damage() {
        let mut turret = Turret::new(Vec2::new(0.0, 0.0), 2);
        turret.take_damage(30);
        assert_eq!(turret.health, 70);

        turret.take_damage(80);
        assert_eq!(turret.health, 0);
    }
}

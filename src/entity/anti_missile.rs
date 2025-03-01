use macroquad::{
    prelude::{draw_rectangle, Vec2, SKYBLUE},
    shapes::draw_circle,
};

const MISSILE_WIDTH: f32 = 10.0;
const MISSILE_HEIGHT: f32 = 10.0;
const MISSILE_SPEED: f32 = 10.0;

/// Represents an anti-missile launched by a turret.
#[derive(Debug)]
pub struct AntiMissile {
    location: Vec2,
    velocity: Vec2,
    explosion_radius: f32,
    life_time: f32,
    is_alive: bool,
}

impl AntiMissile {
    pub fn new(location: Vec2, mouse: Vec2) -> Self {
        let explosion_radius = 50.0;
        let direction = (mouse - location).normalize();
        let velocity = direction * MISSILE_SPEED;
        let lt = 20.0;

        Self {
            location,
            velocity,
            explosion_radius,
            life_time: lt,
            is_alive: true,
        }
    }

    /// Checks for collision with a list of targets.
    pub fn check_collision_general(&mut self, targets: &[Vec2]) -> bool {
        for target in targets {
            if self.location.distance(*target) <= self.explosion_radius {
                self.explode();
                return true;
            }
        }
        false
    }

    /// Combines collision checks for both mouse and multiple targets.
    pub fn check_collision_combined(&mut self, mouse: Vec2, targets: &[Vec2]) -> bool {
        if self.check_collision_general(&[mouse]) {
            return true;
        }
        if self.check_collision_general(targets) {
            return true;
        }
        false
    }

    /// Simulates an explosion.
    fn explode(&mut self) {
        self.is_alive = false;
        self.life_time = 0.0;
        println!("Explosion at {:?}", self.location);
    }

    /// Updates the missile's position.
    pub fn update_position(&mut self, delta_time: f32) {
        if self.is_alive {
            self.life_time -= delta_time;
            if self.life_time <= 0.0 {
                self.explode();
            }
            self.location += self.velocity * delta_time;
        }
    }

    /// Is still alive?
    pub fn report_alive(&self) -> bool {
        self.is_alive
    }

    /// Draws the missile on the screen.
    pub fn draw(&self) {
        if self.is_alive {
            draw_rectangle(
                self.location.x - MISSILE_WIDTH / 2.0,
                self.location.y - MISSILE_HEIGHT / 2.0,
                MISSILE_WIDTH,
                MISSILE_HEIGHT,
                SKYBLUE,
            );
        } else {
            draw_circle(
                self.location.x,
                self.location.y,
                self.explosion_radius,
                macroquad::prelude::ORANGE,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOCATION: Vec2 = Vec2::new(10.0, 5.0);
    const MOUSE: Vec2 = Vec2::new(10.0, 0.0);

    #[test]
    fn test_anti_missile_new() {
        let missile = AntiMissile::new(LOCATION, MOUSE);
        let velocity = (MOUSE - LOCATION).normalize() * MISSILE_SPEED;

        assert_eq!(missile.location, LOCATION);
        assert_eq!(missile.velocity, velocity);
        assert!(missile.is_alive);
    }

    #[test]
    fn test_anti_missile_update_position() {
        let mut missile = AntiMissile::new(LOCATION, MOUSE);
        missile.update_position(1.0);

        assert!(missile.life_time < 20.0);
        assert!(missile.is_alive);
        assert_ne!(missile.location, LOCATION);
    }

    #[test]
    fn test_anti_missile_explode() {
        let mut missile = AntiMissile::new(Vec2::new(10.0, 10.0), MOUSE);
        missile.explode();

        assert!(!missile.is_alive);
        assert_eq!(missile.life_time, 0.0);
    }

    #[test]
    fn test_check_collision_general() {
        let mut missile = AntiMissile::new(LOCATION, MOUSE);
        let targets = vec![
            Vec2::new(10.0, 5.0),
            Vec2::new(10.0, 4.0),
            Vec2::new(10.0, 3.0),
        ];

        // Target is within the explosion radius
        assert!(missile.check_collision_general(&targets));
        assert!(!missile.is_alive);

        let mut missile = AntiMissile::new(LOCATION, MOUSE);
        let targets_out_of_range = vec![Vec2::new(1000.0, 1000.0)];

        // Targets are out of range
        assert!(!missile.check_collision_general(&targets_out_of_range));
        assert!(missile.is_alive);
    }

    #[test]
    fn test_check_collision_combined() {
        let mut missile = AntiMissile::new(LOCATION, MOUSE);
        let targets = vec![Vec2::new(15.0, 5.0), Vec2::new(20.0, 5.0)];

        // Mouse is within the explosion radius
        assert!(missile.check_collision_combined(MOUSE, &targets));
        assert!(!missile.is_alive);

        let mut missile = AntiMissile::new(LOCATION, MOUSE);
        let targets_out_of_range = vec![Vec2::new(1000.0, 1000.0)];

        // Both mouse and targets are out of range
        assert!(!missile.check_collision_combined(Vec2::new(1000.0, 1000.0), &targets_out_of_range));
        assert!(missile.is_alive);
    }
}

use macroquad::prelude::{draw_rectangle, mouse_position, Vec2, SKYBLUE};

const BULLET_WIDTH: f32 = 10.0;
const BULLET_HEIGHT: f32 = 10.0;

/// Represents an anti-missile launched by a turret.
#[derive(Debug)]
pub struct AntiMissile {
    location: Vec2,
    velocity: Vec2,
    explosion_radius: f32,
    is_alive: bool,
}

impl AntiMissile {
    pub fn new(location: Vec2, target: Vec2) -> Self {
        let explosion_radius = 50.0;
        let direction = (target - location).normalize();
        let velocity = direction * 10.0;

        Self {
            location,
            velocity,
            explosion_radius,
            is_alive: true,
        }
    }

    /// Check if the missile hit the target.
    pub fn hit(&self, target: Vec2) -> bool {
        if self.location.distance(target) <= self.explosion_radius {
            return true;
        }
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        if self.location.distance(mouse_pos) <= self.explosion_radius {
            return true;
        }

        false
    }

    /// Simulates an explosion.
    pub fn explode(&mut self) {
        self.is_alive = false;
        println!("Explosion at {:?}", self.location);
    }

    /// Updates the missile's position.
    pub fn update(&mut self) {
        if self.is_alive {
            self.location += self.velocity;
        }
    }

    /// Draws the missile on the screen.
    pub fn draw(&self) {
        // Draw rectangle needs first point of bullet
        draw_rectangle(
            self.location.x - BULLET_WIDTH / 2.0,
            self.location.y - BULLET_HEIGHT / 2.0,
            BULLET_WIDTH,
            BULLET_HEIGHT,
            SKYBLUE,
        );
    }
}

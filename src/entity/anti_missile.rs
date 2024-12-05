use macroquad::{
    prelude::{draw_rectangle, mouse_position, Vec2, SKYBLUE},
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
    pub fn new(location: Vec2, target: Vec2) -> Self {
        let explosion_radius = 50.0;
        let direction = (target - location).normalize();
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

    /// Check if the missile hit the target.
    /// if hits explode return true
    pub fn hit(&mut self, targets: &[Vec2]) -> bool {
        for target in targets {
            if self.location.distance(*target) <= self.explosion_radius {
                self.explode();
                return true;
            }
        }
        false
    }

    /// Simulates an explosion.
    pub fn explode(&mut self) {
        self.is_alive = false;
        self.life_time = 0.0;
        println!("Explosion at {:?}", self.location);
    }

    /// Updates the missile's position.
    pub fn update(&mut self, delta_time: f32) {
        if self.is_alive {
            self.life_time -= delta_time;
            if self.life_time <= 0.0 {
                self.explode();
            }
            self.location += self.velocity * delta_time;
        }
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

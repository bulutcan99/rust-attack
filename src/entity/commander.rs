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
}

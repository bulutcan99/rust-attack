use super::turret::Turret;
use arrayvec::ArrayVec;

// First version will have one turret per commander
pub struct Commander {
    turrets: ArrayVec<Turret, 1>,
}

use macroquad::math::Quat;

use crate::xyz::XYZ;

pub struct Transform {
    // Describes an object's location
    pub position: XYZ,
    pub scale: XYZ,
    pub rotation: Quat,
}

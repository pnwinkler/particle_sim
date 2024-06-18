use crate::colliders::ColliderType;
use crate::transform::Transform;
use crate::xyz::XYZ;

pub struct Object {
    // signed velocity in m/s^2 in the X and Y axes
    pub velocity: XYZ,
    // signed force in Newtons in the X and Y axes
    pub force: XYZ,
    // mass in kilograms
    pub mass: f32,

    pub transform: Transform,
    pub collider: ColliderType,
}

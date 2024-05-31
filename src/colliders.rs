// TODO: put the 2 structs below into transform.rs
use crate::objects::*;
use crate::quaternion::*;

struct CollisionPoints {
    a: XYZ,      // Furthest point of A into B
    b: XYZ,      // Furthest point of B into A
    normal: XYZ, // B – A normalized
    depth: f32,  // Length of B – A
    has_collision: bool,
}

struct Transform {
    // Describes an object's location
    position: XYZ,
    scale: XYZ,
    rotation: Quaternion,
}

pub enum ColliderType {
    SPHERE { center: XYZ, radius: f32 },
    PLANE { normal: XYZ, distance: f32 },
}

pub trait TestCollision {
    fn test_collision(&self, collider: ColliderType) -> bool;
}

impl TestCollision for ColliderType {
    fn test_collision(&self, collider: ColliderType) -> bool {
        use ColliderType::*;
        match collider {
            // TODO: implement collision functions
            PLANE => false,
            SPHERE => false,
        }
    }
}

use core::fmt;

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

impl fmt::Display for CollisionPoints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CollisionPoints(a={},b={},normal={},depth={},has_collision={})",
            self.a, self.b, self.normal, self.depth, self.has_collision
        )
    }
}

struct Transform {
    // Describes an object's location
    position: XYZ,
    scale: XYZ,
    rotation: Quaternion,
}

pub enum ColliderType {
    SPHERE { center: XYZ, radius: f32 },
    // PLANE { normal: XYZ, distance: f32 },
}

pub trait TestCollision {
    fn test_collision(&self, collider: ColliderType) -> bool;
}

impl TestCollision for ColliderType {
    fn test_collision(&self, collider: ColliderType) -> bool {
        use ColliderType::*;
        match (self, collider) {
            // TODO: implement collision functions
            // (PLANE { .. }, PLANE { .. }) => false,
            // (PLANE { .. }, SPHERE { .. }) => false,
            (SPHERE { .. }, SPHERE { .. }) => false,
            // (SPHERE { .. }, PLANE { .. }) => false,
        }
    }
}

// TODO
// fn plane_plane_collision_points() -> CollisionPoints {
//     return false;
// }

// fn plane_sphere_collision_points() -> CollisionPoints {
//     return false;
// }

/// Returns the CollisionPoints for the intersection of two spheres
/// The following formula is used: a^2 + b^2 = c^2. If c <= radius, then the point is considered to be within the circle
fn sphere_sphere_collision_points(
    sphere_1_center: &XYZ,
    sphere_1_radius: f32,
    sphere_2_center: &XYZ,
    sphere_2_radius: f32,
) -> CollisionPoints {
    // make the code easier to read
    let zero_xyz = XYZ {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    // We can choose any axis, because we expect a uniform radius in all directions
    if ((sphere_1_center.x - sphere_1_radius) >= (sphere_2_center.x + sphere_2_radius))
        || ((sphere_1_center.x + sphere_1_radius) <= (sphere_2_center.x - sphere_2_radius))
    {
        return CollisionPoints {
            a: zero_xyz,
            b: zero_xyz,
            normal: zero_xyz, // TODO
            depth: 0.0,       // TODO
            has_collision: false,
        };
    }

    let cx_diff = sphere_1_center.x - sphere_2_center.x;
    let cy_diff = sphere_1_center.y - sphere_2_center.y;
    let cz_diff = sphere_1_center.z - sphere_2_center.z;

    // todo: decide what to do when one sphere fully consumes another
    let deepest_x = if cx_diff < 0.0 {
        (sphere_2_center.x + sphere_2_radius) - sphere_1_center.x
    } else {
        // sphere 1 is to the right of sphere 2
        sphere_1_center.x - (sphere_2_center.x + sphere_2_radius)
    };

    let deepest_y = if cy_diff < 0.0 {
        (sphere_2_center.y + sphere_2_radius) - sphere_1_center.y
    } else {
        sphere_1_center.y - (sphere_2_center.y + sphere_2_radius)
    };

    let deepest_z = if cz_diff < 0.0 {
        (sphere_2_center.z + sphere_2_radius) - sphere_1_center.z
    } else {
        sphere_1_center.z - (sphere_2_center.z + sphere_2_radius)
    };

    let res = CollisionPoints {
        a: XYZ {
            x: deepest_x,
            y: deepest_y,
            z: deepest_z,
        },
        b: XYZ {
            x: deepest_x,
            y: deepest_y,
            z: deepest_z,
        },
        normal: zero_xyz, // TODO
        depth: 0.0,       // TODO
        has_collision: true,
    };
    println!("{}", res);
    return res;

    // return (sphere_1_radius.powf(2.0) + tolerance) >= (a_2 + b_2);
}

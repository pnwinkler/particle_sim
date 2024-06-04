use core::fmt;

// TODO: put the 2 structs below into transform.rs
use crate::objects::*;
use crate::quaternion::*;

pub struct CollisionPoints {
    pub a: XYZ,      // Furthest point of A into B
    pub b: XYZ,      // Furthest point of B into A
    pub normal: XYZ, // B – A normalized
    pub depth: f32,  // Length of B – A
    pub has_collision: bool,
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
    fn test_collision(&self, collider: &ColliderType) -> CollisionPoints;
}

impl TestCollision for ColliderType {
    fn test_collision(&self, collider: &ColliderType) -> CollisionPoints {
        use ColliderType::*;
        match (self, collider) {
            // TODO: implement collision functions
            // (PLANE { .. }, PLANE { .. }) => false,
            // (PLANE { .. }, SPHERE { .. }) => false,
            (
                SPHERE {
                    center: c1,
                    radius: r1,
                },
                SPHERE {
                    center: c2,
                    radius: r2,
                },
            ) => sphere_sphere_collision_points(c1, r1, c2, r2),
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
    sphere_1_radius: &f32,
    sphere_2_center: &XYZ,
    sphere_2_radius: &f32,
) -> CollisionPoints {
    // alias this to make the code easier to read
    let zero_xyz = XYZ {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let cx_diff = sphere_1_center.x - sphere_2_center.x;
    let cy_diff = sphere_1_center.y - sphere_2_center.y;
    let cz_diff = sphere_1_center.z - sphere_2_center.z;

    // todo: tidy up
    let larger_radius = if sphere_1_radius > sphere_2_radius {
        sphere_1_radius
    } else {
        sphere_2_radius
    };

    let smaller_radius = if sphere_1_radius < sphere_2_radius {
        sphere_1_radius
    } else {
        sphere_2_radius
    };

    let x_collision = cx_diff.abs() < *larger_radius;
    let y_collision = cy_diff.abs() < *larger_radius;
    let z_collision = cz_diff.abs() < *larger_radius;

    // TODO: add min and maxes here
    let s1_furthest_x_into_s2 = if x_collision && sphere_1_center.x <= sphere_2_center.x {
        // The furthest collision point of sphere A into sphere B is sphere B's center
        // Hence the min and max functions
        f32::min(sphere_1_center.x + smaller_radius, sphere_2_center.x)
    } else if x_collision && sphere_1_center.x > sphere_2_center.x {
        f32::max(sphere_1_center.x - smaller_radius, sphere_2_center.x)
    } else {
        0.0
    };

    let s1_furthest_y_into_s2 = if y_collision && sphere_1_center.y <= sphere_2_center.y {
        // The furthest collision point of sphere A into sphere B is sphere B's center
        // Hence the min and may functions
        f32::min(sphere_1_center.y + smaller_radius, sphere_2_center.y)
    } else if y_collision && sphere_1_center.y > sphere_2_center.y {
        f32::max(sphere_1_center.y - smaller_radius, sphere_2_center.y)
    } else {
        0.0
    };

    let s1_furthest_z_into_s2 = if z_collision && sphere_1_center.z <= sphere_2_center.z {
        f32::min(sphere_1_center.z + smaller_radius, sphere_2_center.z)
    } else if z_collision && sphere_1_center.z > sphere_2_center.z {
        f32::max(sphere_1_center.z - smaller_radius, sphere_2_center.z)
    } else {
        0.0
    };

    let s2_furthest_x_into_s1 = if x_collision && sphere_2_center.x <= sphere_1_center.x {
        f32::min(sphere_2_center.x + smaller_radius, sphere_1_center.x)
    } else if x_collision && sphere_2_center.x > sphere_1_center.x {
        f32::max(sphere_2_center.x - smaller_radius, sphere_1_center.x)
    } else {
        0.0
    };

    let s2_furthest_y_into_s1 = if y_collision && sphere_2_center.y <= sphere_1_center.y {
        f32::min(sphere_2_center.y + smaller_radius, sphere_1_center.y)
    } else if y_collision && sphere_2_center.y > sphere_1_center.y {
        f32::max(sphere_2_center.y - smaller_radius, sphere_1_center.y)
    } else {
        0.0
    };

    let s2_furthest_z_into_s1 = if z_collision && sphere_2_center.z <= sphere_1_center.z {
        f32::min(sphere_2_center.z + smaller_radius, sphere_1_center.z)
    } else if z_collision && sphere_2_center.z > sphere_1_center.z {
        f32::max(sphere_2_center.z - smaller_radius, sphere_1_center.z)
    } else {
        0.0
    };

    let has_collision = x_collision || y_collision || z_collision;
    // println!("xc={},yc={},zc={}", x_collision, y_collision, z_collision);
    let res = CollisionPoints {
        a: XYZ {
            x: s1_furthest_x_into_s2,
            y: s1_furthest_y_into_s2,
            z: s1_furthest_z_into_s2,
        },
        b: XYZ {
            x: s2_furthest_x_into_s1,
            y: s2_furthest_y_into_s1,
            z: s2_furthest_z_into_s1,
        },
        normal: zero_xyz, // TODO
        depth: 0.0,       // TODO
        has_collision,
    };
    println!("{}", res);
    return res;

    // return (sphere_1_radius.powf(2.0) + tolerance) >= (a_2 + b_2);
}

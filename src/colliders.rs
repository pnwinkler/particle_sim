use crate::objects::NormalizeXyz;
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
    /// Remember to normalize this. We don't do it automatically, in order to avoid accidentally normalizing twice
    /// which would magnify potential floating point imprecision.
    PLANE { normal: XYZ, distance: f32 },
}

pub trait TestCollision {
    fn test_collision(&self, collider: &ColliderType) -> CollisionPoints;
}

impl TestCollision for ColliderType {
    fn test_collision(&self, collider: &ColliderType) -> CollisionPoints {
        use ColliderType::*;
        // todo: make this more legible
        match (self, collider) {
            (
                PLANE {
                    normal: n1,
                    distance: d1,
                },
                PLANE {
                    normal: n2,
                    distance: d2,
                },
            ) => plane_plane_collision_points(n1, d1, n2, d2),

            (
                PLANE {
                    normal: n1,
                    distance: d1,
                },
                SPHERE {
                    center: c1,
                    radius: r1,
                },
            )
            | (
                SPHERE {
                    center: c1,
                    radius: r1,
                },
                PLANE {
                    normal: n1,
                    distance: d1,
                },
            ) => plane_sphere_collision_points(*n1, *d1, *c1, *r1),

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
        }
    }
}

// todo?
fn plane_plane_collision_points(
    _normal_1: &XYZ,
    _distance_1: &f32,
    _normal_2: &XYZ,
    _distance_2: &f32,
) -> CollisionPoints {
    return CollisionPoints {
        a: XYZ {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        b: XYZ {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: XYZ {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, // TODO
        depth: 0.0,
        has_collision: false,
    };
}

fn dot_product(a: XYZ, b: XYZ) -> f32 {
    // todo: make this more generic and move it elsewhere
    let dot_a_b = a.x * b.x + a.y * b.y + a.z * b.z;
    return dot_a_b;
}

fn plane_sphere_collision_points(
    plane_normal: XYZ,
    plane_distance: f32,
    sphere_center: XYZ,
    sphere_radius: f32,
) -> CollisionPoints {
    let p_normal = plane_normal.normalize();
    println!("normal {}", p_normal);
    println!("plane_distance {}", plane_distance);
    let on_plane = p_normal * plane_distance;
    println!("on_plane {}", on_plane);

    // distance from center of sphere to plane surface
    // https://github.com/IainWinter/IwEngine/blob/3e2052855fea85718b7a499a7b1a3befd49d812b/IwEngine/include/iw/physics/impl/TestCollision.h#L53
    let distance = dot_product(sphere_center - on_plane, p_normal);
    println!("distance {}", distance);

    let has_collision = distance < sphere_radius;
    println!("has_collision {}", has_collision);

    let a = sphere_center - p_normal * sphere_radius;
    let b = sphere_center - p_normal * distance;

    // println!("xc={},yc={},zc={}", x_collision, y_collision, z_collision);
    let res = CollisionPoints {
        a,
        b,
        normal: (b - a).normalize(),
        depth: sphere_radius - distance,
        has_collision,
    };
    println!("{}", res);
    return res;
}

/// Returns the CollisionPoints for the intersection of two spheres
/// The furthest collision point of a sphere A into another sphere B, is B's center
fn sphere_sphere_collision_points(
    sphere_1_center: &XYZ,
    sphere_1_radius: &f32,
    sphere_2_center: &XYZ,
    sphere_2_radius: &f32,
) -> CollisionPoints {
    let larger_radius = f32::max(*sphere_1_radius, *sphere_2_radius);
    let smaller_radius = f32::min(*sphere_1_radius, *sphere_2_radius);

    let x_collision = (sphere_1_center.x - sphere_2_center.x) < larger_radius;
    let y_collision = (sphere_1_center.y - sphere_2_center.y) < larger_radius;
    let z_collision = (sphere_1_center.z - sphere_2_center.z) < larger_radius;

    let s1_furthest_x_into_s2 = if x_collision && sphere_1_center.x <= sphere_2_center.x {
        f32::min(sphere_1_center.x + smaller_radius, sphere_2_center.x)
    } else if x_collision && sphere_1_center.x > sphere_2_center.x {
        f32::max(sphere_1_center.x - smaller_radius, sphere_2_center.x)
    } else {
        0.0
    };

    let s1_furthest_y_into_s2 = if y_collision && sphere_1_center.y <= sphere_2_center.y {
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

    let a = XYZ {
        x: s1_furthest_x_into_s2,
        y: s1_furthest_y_into_s2,
        z: s1_furthest_z_into_s2,
    };
    let b = XYZ {
        x: s2_furthest_x_into_s1,
        y: s2_furthest_y_into_s1,
        z: s2_furthest_z_into_s1,
    };
    let diff = b - a;
    let normal = diff.normalize();
    let depth = diff.magnitude();
    let has_collision = x_collision || y_collision || z_collision;

    // println!("xc={},yc={},zc={}", x_collision, y_collision, z_collision);
    let res = CollisionPoints {
        a,
        b,
        normal,
        depth,
        has_collision,
    };
    println!("{}", res);
    return res;
}

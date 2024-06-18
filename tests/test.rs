use macroquad::math::Quat;
use objects::*;
use particle_sim::colliders::ColliderType;
use particle_sim::transform::Transform;
use particle_sim::xyz::XYZ;
use particle_sim::*;

/// Return a sphere with relatively standard arguments. The idea is to use this to reduce
/// boilerplate, then overwrite specified parameters for a given test
fn return_centered_sphere() -> Object {
    let sphere = Object {
        transform: Transform {
            position: XYZ {
                x: 0.5 * SCREEN_WIDTH,
                y: 0.5 * SCREEN_HEIGHT,
                z: 0.0, // TODO: 0.5 * SCREEN_DEPTH,
            },
            scale: XYZ {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            rotation: Quat {
                w: 0.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        velocity: XYZ {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        force: XYZ {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        mass: 1.0,
        collider: ColliderType::SPHERE {
            center: XYZ {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        },
    };
    return sphere;
}

#[cfg(test)]
mod tests {
    use colliders;
    use macroquad::math::Quat;
    use objects::*;
    use particle_sim::colliders::ColliderType;
    use particle_sim::transform::Transform;
    use particle_sim::xyz::NormalizeXyz;
    use particle_sim::xyz::{MagnitudeXyz, XYZ};
    use particle_sim::{colliders::TestCollision, *};

    use crate::return_centered_sphere;

    // After normalizing a vector of 3 equal dimensions, the normalized vector should have
    // this value for all 3 dimensions.
    const BALANCED_NORMAL: f32 = 0.57735026;

    #[test]
    fn test_xyz_magnitude_on_nonzero_input() {
        let loc_1 = XYZ {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        };
        let result = loc_1.magnitude();
        assert_eq!(result, f32::sqrt(75.0));

        let loc_1 = XYZ {
            x: -5.0,
            y: -5.0,
            z: -5.0,
        };
        let result = loc_1.magnitude();
        assert_eq!(result, f32::sqrt(75.0));
    }

    #[test]
    fn test_xyz_magnitude_does_not_return_nan() {
        // We prefer 0.0 to nan
        let loc_1 = XYZ {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let result = loc_1.magnitude();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_xyz_normalize_on_nonzero_input() {
        // Assert that unequal proportions are preserved
        let loc_1 = XYZ {
            x: 1000.0,
            y: 0.0,
            z: 0.0,
        };
        let result = loc_1.normalize();
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);

        // Assert that equal proportions are preserved
        let loc_1 = XYZ {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        };
        let result = loc_1.normalize();
        assert_eq!(result.x, BALANCED_NORMAL);
        assert_eq!(result.y, BALANCED_NORMAL);
        assert_eq!(result.z, BALANCED_NORMAL);

        let loc_1 = XYZ {
            x: 1.0,
            y: 10.0,
            z: 0.0,
        };
        let result = loc_1.normalize();
        // todo: determine why there are more significant digits returned for x than for y?
        let x_expected = 0.099503726;
        let y_expected = 0.9950372;
        assert_eq!(result.x, x_expected);
        assert_eq!(result.y, y_expected);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_xyz_normalize_raises_on_all_zeroes_vector() {
        // It's not possible to normalize a vector where all 3 elements are 0 (can't divide by 0)
        // so we'll assume the user meant something like (1,1,1), where all elements are equal and non-zero
        let loc_1 = XYZ {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let result = loc_1.normalize();
        assert_eq!(result.x, BALANCED_NORMAL);
        assert_eq!(result.y, BALANCED_NORMAL);
        assert_eq!(result.z, BALANCED_NORMAL);
    }

    #[test]
    fn test_sphere_sphere_intersection_sphere_zero_radius() {
        // Test with 0 radius. We expect this to count as a collision
        let sphere_1 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 20.0,
                y: 20.0,
                z: 20.0,
            },
            radius: 5.0,
        };
        let sphere_2 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 20.0,
                y: 20.0,
                z: 20.0,
            },
            radius: 0.0,
        };
        let result = sphere_1.test_collision(&sphere_2);
        // Both spheres should collide at sphere 2's spawn, with the furthest point in each sphere being that spawn
        // (because sphere_2's radius is 0.0)
        assert!(result.has_collision == true);
        assert!(result.a.x == 20.0);
        assert!(result.a.y == 20.0);
        assert!(result.a.z == 20.0);
        assert!(result.b.x == 20.0);
        assert!(result.b.y == 20.0);
        assert!(result.b.z == 20.0);
        // if we have a radius of 0, we can't have a penetration depth > 0.0
        assert_eq!(result.depth, 0.0);
        assert_eq!(result.normal.x, BALANCED_NORMAL);
        assert_eq!(result.normal.y, BALANCED_NORMAL);
        assert_eq!(result.normal.z, BALANCED_NORMAL);
    }

    #[test]
    fn test_sphere_sphere_intersection_no_collision() {
        // Assert that non-colliding spheres are correctly identified as not colliding
        let sphere_3 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 20.0,
                y: 20.0,
                z: 20.0,
            },
            radius: 5.0,
        };
        let sphere_4 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 10.0,
                y: 10.0,
                z: 10.0,
            },
            radius: 4.0,
        };
        let result = sphere_3.test_collision(&sphere_4);
        assert!(result.has_collision == false);
        // There may be more values in our result object, but for non-colliding spheres,
        // we don't care about them
    }

    #[test]
    fn test_sphere_sphere_intersection_single_axis() {
        // Assert that a collision in only one axis is still registered as a collision,
        // and the A and B values are as expected
        let sphere_5 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 20.0,
                y: 20.0,
                z: 20.0,
            },
            radius: 1.0,
        };
        let sphere_6 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 17.0,
                y: 17.0,
                z: 19.0,
            },
            radius: 2.0,
        };
        let result = sphere_5.test_collision(&sphere_6);
        assert!(result.has_collision == true);
        assert!(result.a.z == 19.0);
        // The center is the innermost point that a sphere (A) can intersect into another sphere (B).
        // Therefore, if A intersects past that point, we still expect the point furthest into B to be
        // B's center
        assert!(result.b.z == 20.0);
    }

    #[test]
    fn test_sphere_sphere_intersection_mini_sphere_fully_contained() {
        // Assert that, when a sphere fully contains another sphere:
        // 1) it's registered as a collision
        // 2) the furthest point of the larger sphere into the smaller sphere is the smaller
        // sphere's center
        let sphere_7 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 20.0,
                y: 20.0,
                z: 20.0,
            },
            radius: 10.0,
        };
        let sphere_8 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 21.0,
                y: 21.0,
                z: 21.0,
            },
            radius: 2.0,
        };
        let result = sphere_7.test_collision(&sphere_8);
        assert!(result.has_collision == true);
        // the sphere A, containing the other sphere B, should have B's center as A's furthermost
        // incursion into B's space
        assert!(result.a.z == 21.0);
        // the smaller sphere should choose the point closest to the center of the other sphere, that
        // it can still reach
        assert!(result.b.z == 20.0);
    }

    #[test]
    fn test_sphere_plane_intersection_example() {
        // Roughly emulate the example in the screenshot in the "Collision detection"
        // section of this article https://blog.winter.dev/articles/physics-engine
        let sphere_1 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 0.0,
                y: 4.0,
                z: 0.0,
            },
            radius: 10.0,
        };
        let plane_1 = colliders::ColliderType::PLANE {
            normal: XYZ {
                x: 0.0,
                y: 1.0, // lying flat on the x axis, extending also in z axis, normal pointing up
                z: 0.0,
            }
            .normalize(),
            distance: 0.0,
        };
        let result = sphere_1.test_collision(&plane_1);
        assert!(result.has_collision == true);
        assert_eq!(result.a.x, 0.0);
        assert_eq!(result.b.x, 0.0);

        // sphere's center minus its radius
        assert_eq!(result.a.y, -6.0);
        assert_eq!(result.b.y, 0.0);

        assert_eq!(result.a.z, 0.0);
        assert_eq!(result.b.z, 0.0);

        assert_eq!(result.depth, 6.0);
        assert_eq!(result.normal.x, 0.0);
        // same as the plane's normal
        assert_eq!(result.normal.y, 1.0);
        assert_eq!(result.normal.z, 0.0);
    }

    #[test]
    fn test_sphere_plane_intersection_sphere_not_at_origin() {
        // Test that it can handle a sphere which is not at the origin (0,0,0)
        let sphere_2 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            },
            radius: 25.0,
        };
        let plane_2 = colliders::ColliderType::PLANE {
            normal: XYZ {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
            .normalize(),
            distance: 0.0,
        };
        let result = sphere_2.test_collision(&plane_2);
        assert!(result.has_collision == true);
        // todo: determine what this (the deepest intrusion into a plane) is supposed to represent, given that a
        // plane is of infinite size.
        // For now, I think we ignore it? Given that it maybe doesn't make sense anyway
        assert_eq!(result.a.x, -20.0);
        assert_eq!(result.a.y, 5.0);
        assert_eq!(result.a.z, 5.0);

        // given that the sphere is not centered on the origin 0,0,0, the deepest intrusion of plane
        // into the sphere should therefore not be at the sphere's center
        assert_eq!(result.b.x, 0.0);
        assert_eq!(result.b.y, 5.0);
        assert_eq!(result.b.z, 5.0);
        // this should not equal the sphere's radius, because the sphere isn't centered on 0,0,0
        assert_eq!(result.depth, 20.0);
        assert_eq!(result.normal.x, 1.0);
        assert_eq!(result.normal.y, 0.0);
        assert_eq!(result.normal.z, 0.0);
    }

    #[test]
    fn test_sphere_plane_intersection_handles_plane_not_at_origin() {
        // Test that we successfully handle a collision where the plane has a non-zero distance to origin
        let sphere_3 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 0.0,
                y: 15.0,
                z: 0.0,
            },
            radius: 14.00,
        };
        let plane_3 = colliders::ColliderType::PLANE {
            normal: XYZ {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
            .normalize(),
            // This should make the edge of the plane exactly touch the sphere
            distance: 1.0,
        };
        let result = sphere_3.test_collision(&plane_3);
        assert!(result.has_collision);
        assert_eq!(result.a.x, 0.0);
        assert_eq!(result.b.x, 0.0);

        // This is the lowest point of the sphere
        assert_eq!(result.a.y, 1.0);
        assert_eq!(result.b.y, 1.0);

        assert_eq!(result.a.z, 0.0);
        assert_eq!(result.b.z, 0.0);
        // The two objects exactly touch, so we expect 0 depth
        assert_eq!(result.depth, 0.0);

        // We expect a normal that's equal in each dimension, because the contact patch is equal
        // in each dimension
        assert_eq!(result.normal.x, BALANCED_NORMAL);
        assert_eq!(result.normal.y, BALANCED_NORMAL);
        assert_eq!(result.normal.z, BALANCED_NORMAL);
    }

    #[test]
    fn test_sphere_plane_intersection_handles_no_collision_scenario() {
        // Test that we gracefully handle a no-collision scenario
        let sphere_3 = colliders::ColliderType::SPHERE {
            center: XYZ {
                x: 0.0,
                y: 15.0,
                z: 0.0,
            },
            radius: 14.99,
        };
        let plane_3 = colliders::ColliderType::PLANE {
            normal: XYZ {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
            .normalize(),
            distance: 0.0,
        };
        let result = sphere_3.test_collision(&plane_3);
        assert!(!result.has_collision);
    }

    #[test]
    fn test_convert_meters_to_pixels() {
        let result = particle_sim::convert_meters_to_pixels(10.0, 100.0);
        assert_eq!(result, 1000.0);

        let result = particle_sim::convert_meters_to_pixels(0.001, 0.01);
        assert!(result - 0.000000000001 <= 0.00001);
    }

    #[test]
    fn test_convert_pixels_to_meters() {
        let result = particle_sim::convert_pixels_to_meters(10.0, 100.0);
        assert_eq!(result, 10.0 / 100.0);

        let result = particle_sim::convert_pixels_to_meters(100.0, 0.1);
        assert_eq!(result, 1000.0);
    }

    #[test]
    fn test_calculate_particle_acceleration() {
        let result = particle_sim::calculate_particle_acceleration(2.0, 1.0, 1.0);
        assert_eq!(result, 1.0);

        let result = particle_sim::calculate_particle_acceleration(1.0, 3.0, 2.0);
        assert_eq!(result, -1.0);
    }

    // Maybe worth testing once I have other shapes. Then, I could for example pass a shape, orientation
    // and function in, which would determine the distance to the ground
    // #[test]
    // fn test_particle_touching_ground() {
    //     let result = particle_sim::particle_touching_ground(1.0,2.0,1.0);
    //     assert_eq!(result, 1.0);
    // }

    #[test]
    fn test_calulate_friction() {
        // This formula may break. The point is that we want any y location at which the particle touches the ground.
        let touching_ground_y_pos = SCREEN_HEIGHT - 1.0;

        // Objects not in contact should have 0 friction
        let mut sphere_airborne = return_centered_sphere();
        sphere_airborne.transform.position.x = 5.0;
        sphere_airborne.transform.position.y = 5.0;
        sphere_airborne.transform.position.z = 5.0;
        sphere_airborne.velocity.x = 5.0;
        let result = particle_sim::calculate_friction_deceleration(&sphere_airborne, 0.2);
        assert_eq!(result, 0.0);

        // Objects in contact should have friction
        let mut sphere_grounded_1 = return_centered_sphere();
        sphere_grounded_1.transform.position.y = touching_ground_y_pos;
        sphere_grounded_1.velocity.x = 5.0;
        let result = particle_sim::calculate_friction_deceleration(&sphere_grounded_1, 0.2);
        assert_eq!(result, -1.96);

        // Fast moving objects should have more friction than slow moving objects
        let mut sphere_slow = return_centered_sphere();
        sphere_slow.transform.position.y = touching_ground_y_pos;
        sphere_slow.velocity.x = 1.0;
        let result_slow = particle_sim::calculate_friction_deceleration(&sphere_slow, 0.2);

        let mut sphere_fast = return_centered_sphere();
        sphere_fast.transform.position.y = touching_ground_y_pos;
        sphere_fast.velocity.x = 10.0;
        let result_fast = particle_sim::calculate_friction_deceleration(&sphere_fast, 0.2);
        assert!(result_fast.abs() > result_slow.abs());

        // Objects in contact should have friction regardless of direction
        let mut sphere_grounded_2 = return_centered_sphere();
        sphere_grounded_2.transform.position.y = touching_ground_y_pos;
        sphere_grounded_2.velocity.x = -1.0;
        let result = particle_sim::calculate_friction_deceleration(&sphere_grounded_2, 0.2);
        assert_eq!(result, 1.0);

        // TODO: test the relationship between friction coefficient magnitude and velocity magnitude.
        // The idea is to codify a logical relationship between the two units, instead of the current
        // slightly magical numbers (e.g. why does 0.01 friction coefficient almost instantly stop +5 velocity)?
        // what exactly is the relationship between the two numbers?

        // Test negative friction. It's not required, or how physics works, but it's fun.
        let mut sphere_magical = return_centered_sphere();
        sphere_magical.transform.position.y = touching_ground_y_pos;
        sphere_magical.velocity.x = -5.0;
        let result = particle_sim::calculate_friction_deceleration(&sphere_magical, -0.2);
        assert_eq!(result, -1.96);
    }

    #[test]
    fn test_bounce_y_basic() {
        // Test bounces in both Y directions

        // Choose a large velocity, to stress the calculation logic, and have the object
        // bouncing within 1 second
        let initial_y_velocity =
            1.0 + convert_pixels_to_meters(-0.5 * SCREEN_HEIGHT - 1.0, PIXELS_PER_METER);

        // We want the particle not already colliding on spawn
        let mut sphere = return_centered_sphere();
        sphere.transform.position.x = 0.5 * SCREEN_WIDTH;
        sphere.transform.position.y = 0.5 * SCREEN_HEIGHT;
        sphere.velocity.y = initial_y_velocity;
        let result = particle_sim::calculate_bounce(&sphere, 0.9, 1.0).unwrap();

        // There should be no errors or nans with the parameters we're using here
        assert!(!result.position.y.is_nan());
        assert!(!result.velocity.y.is_nan());

        // The particle should remain within Y bounds
        assert!(result.position.y >= 0.0);
        assert!(result.position.y <= SCREEN_HEIGHT + 0.0001);

        // The particle's X location should be unchanged
        assert_eq!(result.position.x, 0.5 * SCREEN_WIDTH);

        // The particle's Y velocity should be reversed, and reduced in magnitude
        assert!(result.velocity.y >= 0.0);
        assert!(result.velocity.y.abs() < initial_y_velocity.abs());

        // Test the other direction
        let mut sphere_2 = return_centered_sphere();
        sphere_2.transform.position.x = 0.5 * SCREEN_WIDTH;
        sphere_2.transform.position.y = 0.5 * SCREEN_HEIGHT;
        sphere_2.velocity.y = -1.0 * initial_y_velocity;
        let result = particle_sim::calculate_bounce(&sphere, 0.9, 1.0).unwrap();

        assert!(!result.position.y.is_nan());
        assert!(!result.velocity.y.is_nan());
        assert!(result.position.y >= 0.0);
        assert!(result.position.y <= SCREEN_HEIGHT + 0.0001);
        assert_eq!(result.position.x, 0.5 * SCREEN_WIDTH);
        assert!(result.velocity.y >= 0.0);
        assert!(result.velocity.y.abs() > -1.0 * initial_y_velocity.abs());
    }

    #[test]
    fn test_bounce_x_basic() {
        // Test bounces in both x directions
        let initial_x_velocity =
            convert_pixels_to_meters(-0.5 * SCREEN_WIDTH - 1.0, PIXELS_PER_METER);
        let initial_position_y = 0.5 * SCREEN_WIDTH;
        let mut sphere_1 = return_centered_sphere();

        // We want the particle not already colliding on spawn
        // We want the ball to hit the ground within 1 tick
        sphere_1.transform.position.y = initial_position_y;
        sphere_1.velocity.x = initial_x_velocity;
        let result = particle_sim::calculate_bounce(&sphere_1, 0.9, 1.0).unwrap();

        // There should be no errors or nans with the parameters we're using here
        assert!(!result.position.x.is_nan());
        assert!(!result.velocity.x.is_nan());

        // The particle should remain within x bounds
        assert!(result.position.x >= 0.0);
        assert!(result.position.x <= SCREEN_WIDTH + 0.1);

        // The function shouldn't affect the particle's y position
        assert_eq!(result.position.y, initial_position_y);

        // The particle's x velocity should be reversed, and reduced in magnitude
        assert!(result.velocity.x >= 0.0);
        assert!(result.velocity.x.abs() < initial_x_velocity.abs());
    }

    #[test]
    fn test_bounce_coefficient_effect() {
        // Test that the bounce coefficient has the expected influence on particle position and velocity post-bounce
        let initial_x_velocity =
            convert_pixels_to_meters(0.5 * SCREEN_WIDTH + 1.0, PIXELS_PER_METER);
        let mut sphere_1 = return_centered_sphere();
        sphere_1.velocity.x = initial_x_velocity;
        let bounce_coefficient = 0.5;
        let result = particle_sim::calculate_bounce(&sphere_1, bounce_coefficient, 1.0).unwrap();

        // Check we haven't errored or returned nan
        assert!(!result.position.x.is_nan());
        assert!(!result.velocity.x.is_nan());

        // Check that we haven't gone off-screen
        assert!(result.position.x >= 0.0);
        assert!(result.position.x <= SCREEN_WIDTH + 0.1);

        // Check that velocity has been reversed and reduced
        assert!(result.velocity.x <= 0.0);
        println!("{}, {}", result.velocity.x, initial_x_velocity);
        assert!(result.velocity.x < initial_x_velocity);

        // Check that velocity is a fraction of its initial value, as specified by the bounce coefficient
        // This reflects the fact that our bounce coefficient is supposed to be a percentage bounciness, expressed as a value
        // between [0, 1), where the 0 is inclusive, but the 1 is not.
        assert_eq!(
            result.velocity.x.abs(),
            initial_x_velocity * bounce_coefficient
        );
    }

    #[test]
    fn test_bounce_given_invalid_particle_position() {
        // If given a particle position that falls outside of the arena, then the function should return
        // an error, to let the caller handle it
        let mut sphere_1 = return_centered_sphere();
        sphere_1.transform.position.x = SCREEN_WIDTH + 1.0;
        sphere_1.transform.position.y = SCREEN_HEIGHT + 1.0;
        sphere_1.velocity.y = -5.0;
        let result = particle_sim::calculate_bounce(&sphere_1, 1.0, 0.99);
        // TODO: change this to instead be a check for OutOfBoundsError
        assert!(result.is_err());
    }

    #[test]
    fn test_bounce_no_infinite_bouncing() {
        // Check that bounces eventually stop
        let mut sphere_1 = return_centered_sphere();
        sphere_1.velocity.y = -1.0;
        let result = particle_sim::calculate_bounce(&sphere_1, 0.99, 200.0).unwrap();
        assert_eq!(result.velocity.y.abs().floor(), 0.0);

        // The bounced object should remain within bounds, on its bouncing axis
        assert!(result.position.y >= 0.0);
        assert!(result.position.y <= SCREEN_HEIGHT);
    }

    #[test]
    fn test_bounce_terminates_with_object_remaining_in_bounds() {
        // TODO: rename function
        // Check that a bounced object remains above the arena floor
        let mut sphere_1 = return_centered_sphere();
        sphere_1.velocity.y = -1.0;
        let result = particle_sim::calculate_bounce(&sphere_1, 0.01, 200.0).unwrap();
        assert_eq!(result.velocity.y.abs().floor(), 0.0);
        assert!(result.position.y >= 0.0);
    }

    #[test]
    fn test_bounce_at_arena_edge() {
        // Check that bouncing at the very edge of the arena does not bounce it out of bounds
        let mut sphere_1 = return_centered_sphere();

        // We want the particle ever so slightly in bounds

        // ColliderType is an enum, and Rust enums don't allow direct field access like structs do
        // so we can't do sphere_1.collider.radius = 1.0;
        if let ColliderType::SPHERE { ref mut radius, .. } = sphere_1.collider {
            *radius = 1.0;
        }
        println!("{}", sphere_1.collider);
        sphere_1.transform.position.x = 1.0001;
        sphere_1.transform.position.y = 1.0001;
        sphere_1.transform.position.z = 1.0001;

        // We want a negligible velocity, which is significant enough to affect results (in
        // this case greater than the distance to the arena edge), but tiny enough that a lazy
        // programmer may be tempted not to calculate it
        sphere_1.velocity.y = -0.0005;

        // TODO: re-enable this, once we have new collision logic implemented. At the moment,
        // our employed collision logic is using a hard-coded particle radius value
        // Check that the resulting bounce does not move the particle out of bounds
        // let result = particle_sim::calculate_bounce(&sphere_1, 0.9, 1.0).unwrap();
        // println!("Resulting Y position {}", result.position.y);
        // assert!(result.position.y >= PARTICLE_RADIUS_PX);

        // Check the same for the X axis and in the opposite direction
        let initial_position_x = PARTICLE_RADIUS_PX + 0.0001;
        let mut sphere_2 = return_centered_sphere();
        sphere_2.transform.position.x = initial_position_x;
        sphere_2.velocity.x = -0.0005;
        if let ColliderType::SPHERE { ref mut radius, .. } = sphere_1.collider {
            *radius = 1.0;
        }
        let result = particle_sim::calculate_bounce(&sphere_2, 0.9, 1.0).unwrap();
        println!("Resulting X position {}", result.position.x);
        assert!(result.position.x >= PARTICLE_RADIUS_PX);
    }

    #[test]
    fn test_bounce_extreme_velocity_particle_remains_within_bounds() {
        // TODO: add test to check the particle's final velocity after bouncing
        // Check that high velocities update a particle's position and also don't push it out of bounds
        let mut sphere_1 = return_centered_sphere();
        sphere_1.velocity.x = 5000.0;
        sphere_1.velocity.y = -5001.0;

        let result = particle_sim::calculate_bounce(&sphere_1, 0.20, 1.0).unwrap();
        assert!(result.position.x >= 0.0);
        assert!(result.position.x <= SCREEN_WIDTH + 0.1);
        assert!(result.position.y >= 0.0);
        assert!(result.position.y <= SCREEN_HEIGHT + 0.1);

        assert!(result.velocity.y.abs() <= 5001.0);
        assert!(result.velocity.x.abs() <= 5000.0);
    }

    /*
    Integration tests below
    */

    #[test]
    fn test_simulation_tick_basic() {
        // Test that several simulation ticks do not move a particle out of bounds
        let ticks = 10;
        let seconds_elapsed = 1.0;

        let mut sphere_1 = return_centered_sphere();
        sphere_1.velocity.y = 50.0;

        let mut sphere_2 = return_centered_sphere();
        sphere_2.transform.position.x = 0.25 * SCREEN_WIDTH;
        sphere_2.transform.position.y = 0.25 * SCREEN_HEIGHT;
        sphere_2.velocity.y = -50.0;

        let mut particles: Vec<Object> = vec![sphere_1, sphere_2];

        // Check that the resulting bounce does not move the particle out of bounds
        for _i in 0..ticks {
            particle_sim::simulation_tick(&mut particles, seconds_elapsed);
        }
        let result_1 = particles.get(0).unwrap();
        let result_2 = particles.get(0).unwrap();

        // Check we haven't errored or returned nan
        assert!(!result_1.transform.position.x.is_nan());
        assert!(!result_1.velocity.x.is_nan());
        assert!(!result_2.transform.position.x.is_nan());
        assert!(!result_2.velocity.x.is_nan());

        assert!(!result_1.transform.position.y.is_nan());
        assert!(!result_1.velocity.y.is_nan());
        assert!(!result_2.transform.position.y.is_nan());
        assert!(!result_2.velocity.y.is_nan());

        // Check that we haven't gone off-screen
        assert!(result_1.transform.position.x >= 0.0);
        assert!(result_1.transform.position.x <= SCREEN_WIDTH + 0.1);
        assert!(result_2.transform.position.x >= 0.0);
        assert!(result_2.transform.position.x <= SCREEN_WIDTH + 0.1);

        assert!(result_1.transform.position.y >= 0.0);
        assert!(result_1.transform.position.y <= SCREEN_HEIGHT + 0.1);
        assert!(result_2.transform.position.y >= 0.0);
        assert!(result_2.transform.position.y <= SCREEN_HEIGHT + 0.1);
    }

    #[test]
    fn test_simulation_tick_is_deterministic() {
        // Check that running our simulation twice with the same parameters gives the same results each time
        let ticks = 10;
        let seconds_elapsed = 1.0;

        let mut particles_1: Vec<Object> = vec![return_centered_sphere()];
        let mut particles_2: Vec<Object> = vec![return_centered_sphere()];
        for _i in 0..ticks {
            particle_sim::simulation_tick(&mut particles_1, seconds_elapsed);
            particle_sim::simulation_tick(&mut particles_2, seconds_elapsed);
        }
        let result_1 = particles_1.get(0).unwrap();
        let result_2 = particles_2.get(0).unwrap();

        assert_eq!(result_1.velocity.x, result_2.velocity.x);
        assert_eq!(result_1.velocity.y, result_2.velocity.y);
        assert_eq!(result_1.transform.position.y, result_2.transform.position.y);
        assert_eq!(result_1.transform.position.x, result_2.transform.position.x);
    }

    #[test]
    fn test_simulation_tick_frequency_does_not_affect_results() {
        // Check that simulation produces the same results regardless of tick frequency over an identical timespan
        let seconds_elapsed = 1.0;

        let mut particles_1: Vec<Object> = vec![return_centered_sphere()];
        let mut particles_2: Vec<Object> = vec![return_centered_sphere()];

        particle_sim::simulation_tick(&mut particles_1, seconds_elapsed);
        let result_1 = particles_1.get(0).unwrap();

        particle_sim::simulation_tick(&mut particles_2, seconds_elapsed / 2.0);
        particle_sim::simulation_tick(&mut particles_2, seconds_elapsed / 2.0);
        let result_2 = particles_2.get(0).unwrap();

        // TODO: determine why the X pos and velocities agree with one another, but the Y velocity only doesn't?
        // TODO: re-enable
        // It's because I've got my units mixed up. For the bounce travel distance, I use the particle's current velocity,
        // then multiply it by the time elapsed (roughly). That means that if I halve the time examined, I incorrectly
        // quarter the total travel distance
        // assert_eq!(result_1.transform.position.x, result_2.transform.position.x);
        // assert_eq!(result_1.transform.position.y, result_2.transform.position.y);
        // assert_eq!(result_1.velocity.y, result_2.velocity.y);
        // assert_eq!(result_1.velocity.x, result_2.velocity.x);

        // Repeat the test, but with extra ticks, to test for potentially compounding divergence
        // let extra_ticks = 9;
        // for _i in 0..extra_ticks {
        //     particle_sim::simulation_tick(&mut particles_1, seconds_elapsed);
        //
        //     particle_sim::simulation_tick(&mut particles_2, seconds_elapsed / 2.0);
        //     particle_sim::simulation_tick(&mut particles_2, seconds_elapsed / 2.0);
        // }
        // assert_eq!(result_1.position.x, result_2.position.x);
        // assert_eq!(result_1.position.y, result_2.position.y);
        // assert_eq!(result_1.velocity.x, result_2.velocity.x);
        // assert_eq!(result_1.velocity.y, result_2.velocity.y);
    }
}

// TODO: make this file WAY less verbose
// TODO: update tests to respect Z dimension
// TODO: test high speed horizontal and vertical bounces.
// todo: add tests for ultra-high frequency bounces (e.g. more than once per frame)
// todo: add tests for >= 2 particles bouncing into each other at once
// todo: add tests for cascading bounces? Where one bounce triggers other bounces, potentially of already bounced particles
// todo: break these tests up into different functions?

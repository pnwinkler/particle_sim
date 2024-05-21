#[cfg(test)]
mod tests {
    use particle_sim::*;

    #[test]
    fn test_does_circle_intersect() {
        // Test if the following points intersect our imaginary circle
        let result = does_circle_intersect(XY { x: 0.0, y: 0.0 }, 5.0, XY { x: 1.0, y: 1.0 });
        assert!(result == true);

        let result = does_circle_intersect(XY { x: 1.0, y: 1.0 }, 5.0, XY { x: 6.0, y: 1.0 });
        assert!(result == true);

        let result = does_circle_intersect(XY { x: 50.0, y: 50.0 }, 5.0, XY { x: 50.0, y: 55.1 });
        assert!(result == false);
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

        // objects not in contact should have 0 friction
        let particle_1 = Particle {
            x_pos: 5.0,
            y_pos: 5.0,
            x_velocity_m_s: 5.0,
            y_velocity_m_s: 0.0,
        };
        let result = particle_sim::calculate_friction_deceleration(&particle_1, 0.2);
        assert_eq!(result, 0.0);

        // objects in contact should have friction
        let particle_ground = Particle {
            x_pos: 0.0,
            y_pos: touching_ground_y_pos,
            x_velocity_m_s: 5.0,
            y_velocity_m_s: 0.0,
        };
        let result = particle_sim::calculate_friction_deceleration(&particle_ground, 0.2);
        assert_eq!(result, -1.96);

        // fast moving objects should have more friction than slow moving objects
        let particle_slow = Particle {
            x_pos: 5.0,
            y_pos: touching_ground_y_pos,
            x_velocity_m_s: 1.0,
            y_velocity_m_s: 0.0,
        };
        let result_slow = particle_sim::calculate_friction_deceleration(&particle_slow, 0.2);
        let particle_fast = Particle {
            x_pos: 5.0,
            y_pos: touching_ground_y_pos,
            x_velocity_m_s: 10.0,
            y_velocity_m_s: 0.0,
        };
        let result_fast = particle_sim::calculate_friction_deceleration(&particle_fast, 0.2);
        assert!(result_fast.abs() > result_slow.abs());

        // objects in contact should have friction regardless of direction
        let particle_ground = Particle {
            x_pos: 0.0,
            y_pos: touching_ground_y_pos,
            x_velocity_m_s: -1.0,
            y_velocity_m_s: 0.0,
        };
        let result = particle_sim::calculate_friction_deceleration(&particle_ground, 0.2);
        assert_eq!(result, 1.0);

        // TODO: test the relationship between friction coefficient magnitude and velocity magnitude.
        // The idea is to codify a logical relationship between the two units, instead of the current
        // slightly magical numbers (e.g. why does 0.01 friction coefficient almost instantly stop +5 velocity)?
        // what exactly is the relationship between the two numbers?

        // Test negative friction. It's not required, or how physics works, but it's fun.
        let particle_ground = Particle {
            x_pos: 0.0,
            y_pos: touching_ground_y_pos,
            x_velocity_m_s: -5.0,
            y_velocity_m_s: 0.0,
        };
        let result = particle_sim::calculate_friction_deceleration(&particle_ground, -0.2);
        assert_eq!(result, -1.96);
    }

    #[test]
    fn test_apply_gravity_to_particles() {
        let particle_airborne_1 = Particle {
            x_pos: 0.0,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: -5.0,
            y_velocity_m_s: 0.0,
        };
        let result =
            particle_sim::calculate_gravity_effect_on_velocity(&particle_airborne_1, 9.8, 1.0);
        assert_eq!(result, 9.8);

        let particle_airborne_2 = Particle {
            x_pos: 0.0,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: -5.0,
            y_velocity_m_s: 0.0,
        };
        let result =
            particle_sim::calculate_gravity_effect_on_velocity(&particle_airborne_2, 9.8, 2.0);
        assert_eq!(result, 19.6);

        // Particles already touching the ground should not accelerate due to gravity
        let touching_ground_y_pos = SCREEN_HEIGHT - 1.0;
        let particle_ground = Particle {
            x_pos: 0.0,
            y_pos: touching_ground_y_pos,
            x_velocity_m_s: -5.0,
            y_velocity_m_s: 0.0,
        };
        let result = particle_sim::calculate_gravity_effect_on_velocity(&particle_ground, 9.8, 1.0);
        assert_eq!(result, 0.0);

        // Gravity applied over two 0.5 second intervals should equal that of of a single 1.0 second interval
        let particle_1 = Particle {
            x_pos: 0.0,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: -5.0,
            y_velocity_m_s: 0.0,
        };
        let result_1 = particle_sim::calculate_gravity_effect_on_velocity(&particle_1, 9.8, 1.0);
        let mut result_2 = 0.0;
        result_2 += particle_sim::calculate_gravity_effect_on_velocity(&particle_1, 9.8, 0.5);
        result_2 += particle_sim::calculate_gravity_effect_on_velocity(&particle_1, 9.8, 0.5);
        assert_eq!(result_1, result_2);
    }

    #[test]
    fn test_bounce_y_basic() {
        // Test bounces in both Y directions
        let initial_y_velocity =
            convert_pixels_to_meters(-0.5 * SCREEN_HEIGHT - 1.0, PIXELS_PER_METER);
        let initial_x_pos = 0.5 * SCREEN_WIDTH;
        let particle = Particle {
            x_pos: initial_x_pos,
            y_pos: 0.5 * SCREEN_HEIGHT, // we want the particle not already colliding on spawn
            x_velocity_m_s: 0.0,
            y_velocity_m_s: initial_y_velocity, // we want the ball to hit the ground within 1 tick
        };
        let result = particle_sim::calculate_bounce(&particle, 0.9, 1.0).unwrap();

        // There should be no errors or nans with the parameters we're using here
        assert!(!result.y_pos.is_nan());
        assert!(!result.y_velocity.is_nan());

        // The particle should remain within Y bounds
        assert!(result.y_pos >= 0.0);
        assert!(result.y_pos <= SCREEN_HEIGHT + 0.1);

        // The particle's X location should be unchanged
        assert_eq!(result.x_pos, initial_x_pos);

        // The particle's Y velocity should be reversed, and reduced in magnitude
        println!(
            "Y velocity: {} --> {}",
            initial_y_velocity, result.y_velocity
        );
        println!("Y position: {} --> {}", 0.5 * SCREEN_HEIGHT, result.y_pos);
        assert!(result.y_velocity >= 0.0);
        assert!(result.y_velocity.abs() < initial_y_velocity.abs());
    }

    #[test]
    fn test_bounce_x_basic() {
        // Test bounces in both x directions
        let initial_x_velocity =
            convert_pixels_to_meters(-0.5 * SCREEN_WIDTH - 1.0, PIXELS_PER_METER);
        let initial_y_pos = 0.5 * SCREEN_WIDTH;
        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH, // we want the particle not already colliding on spawn
            y_pos: initial_y_pos,
            x_velocity_m_s: initial_x_velocity, // we want the ball to hit the ground within 1 tick
            y_velocity_m_s: 0.0,
        };
        let result = particle_sim::calculate_bounce(&particle, 0.9, 1.0).unwrap();

        // There should be no errors or nans with the parameters we're using here
        assert!(!result.x_pos.is_nan());
        assert!(!result.x_velocity.is_nan());

        // The particle should remain within x bounds
        assert!(result.x_pos >= 0.0);
        assert!(result.x_pos <= SCREEN_WIDTH + 0.1);

        // The particle's y location should be unchanged
        assert_eq!(result.y_pos, initial_y_pos);

        // The particle's x velocity should be reversed, and reduced in magnitude
        println!(
            "x velocity: {} --> {}",
            initial_x_velocity, result.x_velocity
        );
        println!("x position: {} --> {}", 0.5 * SCREEN_WIDTH, result.x_pos);
        assert!(result.x_velocity >= 0.0);
        assert!(result.x_velocity.abs() < initial_x_velocity.abs());
    }

    // TODO: test bounds

    #[test]
    fn test_bounce_coefficient_effect() {
        // Test that the bounce coefficient has the expected influence on particle position and velocity post-bounce
        let initial_x_velocity =
            convert_pixels_to_meters(0.5 * SCREEN_WIDTH + 1.0, PIXELS_PER_METER);
        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: initial_x_velocity, // we want the ball to hit the side within 1 tick
            y_velocity_m_s: 0.0,
        };
        let bounce_coefficient = 0.5;
        let result = particle_sim::calculate_bounce(&particle, bounce_coefficient, 1.0).unwrap();

        // Check we haven't errored or returned nan
        assert!(!result.x_pos.is_nan());
        assert!(!result.x_velocity.is_nan());

        // Check that we haven't gone off-screen
        assert!(result.x_pos >= 0.0);
        assert!(result.x_pos <= SCREEN_WIDTH + 0.1);

        // Check that velocity has been reversed and reduced
        assert!(result.x_velocity <= 0.0);
        println!("{}, {}", result.x_velocity, initial_x_velocity);
        assert!(result.x_velocity < initial_x_velocity);

        // Check that velocity is a fraction of its initial value, as specified by the bounce coefficient
        // This reflects the fact that our bounce coefficient is supposed to be a percentage bounciness, expressed as a value
        // between [0, 1), where the 0 is inclusive, but the 1 is not.
        assert_eq!(
            result.x_velocity.abs(),
            initial_x_velocity * bounce_coefficient
        );
    }

    #[test]
    fn test_bounce_given_invalid_particle_position() {
        // If given a particle position that falls outside of the arena, then the function should return
        // an error, to let the caller handle it
        let particle = Particle {
            x_pos: SCREEN_WIDTH + 1.0,
            y_pos: SCREEN_HEIGHT + 1.0,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: -5.0,
        };
        let result = particle_sim::calculate_bounce(&particle, 1.0, 0.99);
        // TODO: change this to instead be a check for OutOfBoundsError
        assert!(result.is_err());
    }

    #[test]
    fn test_bounce_no_infinite_bouncing() {
        // Check that bounces eventually stop
        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: -1.0,
        };
        let result = particle_sim::calculate_bounce(&particle, 0.9, 200.0).unwrap();
        assert_eq!(result.y_velocity.abs().floor(), 0.0);

        // The bounced object should remain within bounds, on its bouncing axis
        assert!(result.y_pos >= 0.0);
        assert!(result.y_pos <= SCREEN_HEIGHT);
    }

    #[test]
    fn test_bounce_terminates_with_object_remaining_in_bounds() {
        // TODO: rename function
        // Check that a bounced object remains above the arena floor
        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: -1.0,
        };
        let result = particle_sim::calculate_bounce(&particle, 0.01, 200.0).unwrap();
        assert_eq!(result.y_velocity.abs().floor(), 0.0);
        assert!(result.y_pos >= 0.0);
    }

    #[test]
    fn test_bounce_at_arena_edge() {
        // Check that bouncing at the very edge of the arena does not bounce it out of bounds
        let initial_y_pos = SCREEN_HEIGHT - PARTICLE_RADIUS_PX - 0.0001;
        let particle = Particle {
            // we want the particle ever so slightly in bounds
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: initial_y_pos,
            x_velocity_m_s: 0.0,
            // we want a negligible velocity, which is greater than the distance to the arena edge, but
            // still tiny enough that the programmer may be tempted not to calculate it
            y_velocity_m_s: 0.0005,
        };
        // Check that the resulting bounce does not move the particle out of bounds
        let result = particle_sim::calculate_bounce(&particle, 0.9, 1.0).unwrap();
        println!("Resulting Y position {}", result.y_pos);
        assert!(result.y_pos <= SCREEN_HEIGHT - PARTICLE_RADIUS_PX);

        // Check the same for the X axis and in the opposite direction
        let initial_x_pos = PARTICLE_RADIUS_PX + 0.0001;
        let particle = Particle {
            x_pos: initial_x_pos,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: -0.0005,
            // we want a negligible velocity, one which is beyond our velocity cut-off
            y_velocity_m_s: 0.0,
        };
        let result = particle_sim::calculate_bounce(&particle, 0.9, 1.0).unwrap();
        println!("Resulting X position {}", result.x_pos);
        assert!(result.x_pos >= PARTICLE_RADIUS_PX);
    }

    #[test]
    fn test_bounce_extreme_velocity_particle_remains_within_bounds() {
        // TODO: add test to check the particle's final velocity after bouncing
        // Check that high velocities update a particle's position and also don't push it out of bounds
        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 5000.0,
            y_velocity_m_s: -5001.0,
        };
        // TODO: finish
        let result = particle_sim::calculate_bounce(&particle, 0.20, 1.0).unwrap();
        assert!(result.x_pos >= 0.0);
        assert!(result.x_pos <= SCREEN_WIDTH + 0.1);
        assert!(result.y_pos >= 0.0);
        assert!(result.y_pos <= SCREEN_HEIGHT + 0.1);

        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 5000.0,
            y_velocity_m_s: -5001.0,
        };
        let result = particle_sim::calculate_bounce(&particle, 0.20, 1.0).unwrap();
        assert!(result.x_pos >= 0.0);
        assert!(result.x_pos <= SCREEN_WIDTH + 0.1);
        assert!(result.y_pos >= 0.0);
        assert!(result.y_pos <= SCREEN_HEIGHT + 0.1);
    }

    /*
    Integration tests below
    */

    #[test]
    fn test_simulation_tick_basic() {
        // Test that several simulation ticks do not move a particle out of bounds
        let ticks = 10;
        let seconds_elapsed = 1.0;
        let mut particles: Vec<Particle> = vec![
            Particle {
                x_pos: 0.5 * SCREEN_WIDTH,
                y_pos: 0.5 * SCREEN_HEIGHT,
                x_velocity_m_s: 0.0,
                y_velocity_m_s: 50.0,
            },
            Particle {
                x_pos: 0.25 * SCREEN_WIDTH,
                y_pos: 0.25 * SCREEN_HEIGHT,
                x_velocity_m_s: 0.0,
                y_velocity_m_s: -50.0,
            },
        ];
        // Check that the resulting bounce does not move the particle out of bounds
        for _i in 0..ticks {
            particle_sim::simulation_tick(&mut particles, seconds_elapsed);
        }
        let result_1 = particles.get(0).unwrap();
        let result_2 = particles.get(0).unwrap();

        // Check we haven't errored or returned nan
        assert!(!result_1.x_pos.is_nan());
        assert!(!result_1.x_velocity_m_s.is_nan());
        assert!(!result_2.x_pos.is_nan());
        assert!(!result_2.x_velocity_m_s.is_nan());

        assert!(!result_1.y_pos.is_nan());
        assert!(!result_1.y_velocity_m_s.is_nan());
        assert!(!result_2.y_pos.is_nan());
        assert!(!result_2.y_velocity_m_s.is_nan());

        // Check that we haven't gone off-screen
        assert!(result_1.x_pos >= 0.0);
        assert!(result_1.x_pos <= SCREEN_WIDTH + 0.1);
        assert!(result_2.x_pos >= 0.0);
        assert!(result_2.x_pos <= SCREEN_WIDTH + 0.1);

        assert!(result_1.y_pos >= 0.0);
        assert!(result_1.y_pos <= SCREEN_HEIGHT + 0.1);
        assert!(result_2.y_pos >= 0.0);
        assert!(result_2.y_pos <= SCREEN_HEIGHT + 0.1);
    }

    #[test]
    fn test_simulation_tick_is_deterministic() {
        // Check that running our simulation twice with the same parameters gives the same results each time
        let ticks = 10;
        let seconds_elapsed = 1.0;

        let mut particles_1: Vec<Particle> = vec![Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: 0.0,
        }];
        let mut particles_2: Vec<Particle> = vec![Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: 0.0,
        }];
        for _i in 0..ticks {
            particle_sim::simulation_tick(&mut particles_1, seconds_elapsed);
            particle_sim::simulation_tick(&mut particles_2, seconds_elapsed);
        }
        let result_1 = particles_1.get(0).unwrap();
        let result_2 = particles_2.get(0).unwrap();

        assert_eq!(result_1.x_velocity_m_s, result_2.x_velocity_m_s);
        assert_eq!(result_1.y_velocity_m_s, result_2.y_velocity_m_s);
        assert_eq!(result_1.y_pos, result_2.y_pos);
        assert_eq!(result_1.x_pos, result_2.x_pos);
    }

    #[test]
    fn test_simulation_tick_frequency_does_not_affect_results() {
        // Check that simulation produces the same results regardless of tick frequency over an identical timespan
        let seconds_elapsed = 1.0;

        let mut particles_1: Vec<Particle> = vec![Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: 0.0,
        }];

        let mut particles_2: Vec<Particle> = vec![Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: 0.0,
        }];

        particle_sim::simulation_tick(&mut particles_1, seconds_elapsed);
        let result_1 = particles_1.get(0).unwrap();

        particle_sim::simulation_tick(&mut particles_2, seconds_elapsed / 2.0);
        particle_sim::simulation_tick(&mut particles_2, seconds_elapsed / 2.0);
        let result_2 = particles_2.get(0).unwrap();

        // TODO: determine why the X pos and velocities agree with one another, but the Y velocity only doesn't?
        // It's because I've got my units mixed up. For the bounce travel distance, I use the particle's current velocity,
        // then multiply it by the time elapsed (roughly). That means that if I halve the time examined, I incorrectly
        // quarter the total travel distance
        assert_eq!(result_1.x_pos, result_2.x_pos);
        assert_eq!(result_1.y_pos, result_2.y_pos);
        assert_eq!(result_1.y_velocity_m_s, result_2.y_velocity_m_s);
        assert_eq!(result_1.x_velocity_m_s, result_2.x_velocity_m_s);

        // Repeat the test, but with extra ticks, to test for potentially compounding divergence
        // let extra_ticks = 9;
        // for _i in 0..extra_ticks {
        //     particle_sim::simulation_tick(&mut particles_1, seconds_elapsed);
        //
        //     particle_sim::simulation_tick(&mut particles_2, seconds_elapsed / 2.0);
        //     particle_sim::simulation_tick(&mut particles_2, seconds_elapsed / 2.0);
        // }
        // assert_eq!(result_1.x_pos, result_2.x_pos);
        // assert_eq!(result_1.y_pos, result_2.y_pos);
        // assert_eq!(result_1.x_velocity_m_s, result_2.x_velocity_m_s);
        // assert_eq!(result_1.y_velocity_m_s, result_2.y_velocity_m_s);
    }
}

// TODO: test high speed horizontal and vertical bounces.
// todo: add tests for ultra-high frequency bounces (e.g. more than once per frame)
// todo: add tests for >= 2 particles bouncing into each other at once
// todo: add tests for cascading bounces? Where one bounce triggers other bounces, potentially of already bounced particles
// todo: break these tests up into different functions?

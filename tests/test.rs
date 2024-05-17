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
        assert!(result - 0.00001 <= 0.00001);
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
    }

    #[test]
    fn test_update_particle_position() {
        // TODO: test high speed horizontal and vertical bounces.
        // todo: add tests for ultra-high frequency bounces (e.g. more than once per frame)
        // todo: add tests for >= 2 particles bouncing into each other at once
        // todo: add tests for cascading bounces? Where one bounce triggers other bounces, potentially of already bounced particles
        // todo: break these tests up into different functions?

        // Bounces should reverse the direction of a particle, and its direction of travel. Test vertical
        let initial_y_velocity = -0.5 * SCREEN_HEIGHT - 1.0;
        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT, // arbitrarily chosen, but we want the particle not already colliding on spawn
            x_velocity_m_s: 0.0,
            y_velocity_m_s: initial_y_velocity, // we want the ball to hit the ground within 1 tick
        };
        let result = particle_sim::calculate_bounce(&particle, 1.0, 0.9).unwrap();
        assert!(!result.y_pos.is_nan());
        assert!(result.y_pos >= 0.0);
        assert!(!result.y_velocity.is_nan());
        println!(
            "Y velocity: {} --> {}",
            initial_y_velocity, result.y_velocity
        );
        println!("Y position: {} --> {}", 0.5 * SCREEN_HEIGHT, result.y_pos);
        assert!(result.y_velocity >= 0.0);
        assert!(result.y_velocity.abs() < initial_y_velocity.abs());

        // Bounces should reverse the direction of a particle, and its direction of travel. Test horizontal
        let initial_x_velocity = 0.5 * SCREEN_WIDTH + 1.0;
        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: initial_x_velocity, // we want the ball to hit the side within 1 tick
            y_velocity_m_s: 0.0,
        };
        let result = particle_sim::calculate_bounce(&particle, 1000.0 / 144.0, 0.5).unwrap();
        assert!(!result.x_pos.is_nan());
        assert!(result.x_pos >= 0.0);
        // check we haven't gone off-screen
        assert!(result.x_pos <= SCREEN_WIDTH + 0.1);
        assert!(!result.x_velocity.is_nan());
        assert!(result.x_velocity <= 0.0);
        println!("{}, {}", result.x_velocity, initial_x_velocity);
        assert!(result.x_velocity < initial_x_velocity);

        // If given out of range inputs, then the function should return an error, to let the caller handle it
        let particle = Particle {
            x_pos: SCREEN_WIDTH + 1.0,
            y_pos: SCREEN_HEIGHT + 1.0,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: -5.0,
        };
        let result = particle_sim::calculate_bounce(&particle, 1.0, 0.99);
        // TODO: change this to instead be a check for OutOfBoundsError
        assert!(result.is_err());

        // Check that bounces eventually stop
        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: -1.0,
        };
        let result = particle_sim::calculate_bounce(&particle, 200.0, 0.9).unwrap();
        assert_eq!(result.y_velocity.abs().floor(), 0.0);
        assert!(result.y_pos >= 0.0);

        // Check that object remains above the arena floor
        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: -1.0,
        };
        let result = particle_sim::calculate_bounce(&particle, 200.0, 0.01).unwrap();
        assert_eq!(result.y_velocity.abs().floor(), 0.0);
        assert!(result.y_pos >= 0.0);

        // Check that bouncing does not push a particle on the edge of the arena over it
        let initial_y_pos = SCREEN_HEIGHT - PARTICLE_RADIUS_PX - 0.001;
        let particle = Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            // we want the particle ever so slightly in bounds
            y_pos: initial_y_pos,
            x_velocity_m_s: 0.0,
            // we want a negligible velocity, one which is beyond our velocity cut-off
            y_velocity_m_s: 0.05,
        };
        // Check that the resulting bounce does not move the particle out of bounds
        let result = particle_sim::calculate_bounce(&particle, 1.0, 0.9).unwrap();
        println!("{}", result.y_pos);
        assert!(result.y_pos <= SCREEN_HEIGHT - PARTICLE_RADIUS_PX);

        // Check that bouncing does not push a particle on the edge of the arena over it
        let initial_x_pos = PARTICLE_RADIUS_PX + 0.001;
        let particle = Particle {
            x_pos: initial_x_pos,
            // we want the particle ever so slightly in bounds
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: -0.05,
            // we want a negligible velocity, one which is beyond our velocity cut-off
            y_velocity_m_s: 0.0,
        };
        // Check that the resulting bounce does not move the particle out of bounds
        let result = particle_sim::calculate_bounce(&particle, 1.0, 0.9).unwrap();
        println!("THING {}", result.x_pos);
        assert!(result.x_pos >= PARTICLE_RADIUS_PX);
    }

    /*
    Integration tests below
    */

    #[test]
    fn test_simulation_tick() {
        // Check that running our simulation twice with the same parameters gives the same results each time
        // Other properties will need to be added here, if particles acquire more properties!
        let ticks = 10;
        let seconds_elapsed = 1.0;
        let mut particles: Vec<Particle> = vec![Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: 0.0,
        }];
        // Check that the resulting bounce does not move the particle out of bounds
        for _i in 0..ticks {
             particle_sim::simulation_tick(&mut particles, seconds_elapsed);
        }
        let result_1 = particles.get(0).unwrap();

        let mut particles: Vec<Particle> = vec![Particle {
            x_pos: 0.5 * SCREEN_WIDTH,
            y_pos: 0.5 * SCREEN_HEIGHT,
            x_velocity_m_s: 0.0,
            y_velocity_m_s: 0.0,
        }];
        for _i in 0..ticks {
             particle_sim::simulation_tick(&mut particles, seconds_elapsed);
        }
        let result_2 = particles.get(0).unwrap();

        assert_eq!(result_1.y_velocity_m_s, result_2.y_velocity_m_s);
        assert_eq!(result_1.x_velocity_m_s, result_2.x_velocity_m_s);
        assert_eq!(result_1.y_pos, result_2.y_pos);
        assert_eq!(result_1.x_pos, result_2.x_pos);
    }
}

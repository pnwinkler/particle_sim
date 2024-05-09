pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use particle_sim::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
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
        let result = particle_sim::calculate_gravity_effect_on_velocity(9.8, 1.0);
        assert_eq!(result, 9.8);

        let result = particle_sim::calculate_gravity_effect_on_velocity(9.8, 2.0);
        assert_eq!(result, 19.6);
    }

    #[test]
    fn test_update_particle_position() {
        // Bounces should reverse the direction of a particle, and its direction of travel
        let mut particle = Particle {
            x_pos: 0.0,
            y_pos: 0.5 * SCREEN_HEIGHT, // arbitrarily chosen, but we want the particle not already colliding on spawn
            x_velocity_m_s: 0.0,
            y_velocity_m_s: -0.5 * SCREEN_HEIGHT + 1.0, // we want the ball to hit the ground within 1 tick
        };
        let result = particle_sim::update_particle_position(&mut particle, 1.0, 0.99);
        assert!(particle.y_pos >= 0.0);
        assert!(particle.y_velocity_m_s >= 0.0);
        assert!(particle.y_velocity_m_s.abs() < (-0.5 * SCREEN_HEIGHT + 1.0).abs());

        // todo: add tests for ultra-high frequency bounces (e.g. more than once per frame)
        // todo: add tests for >= 2 particles bouncing into each other at once
        // todo: add tests for cascading bounces? Where one bounce triggers other bounces, potentially of already bounced particles
    }
    // Test commented out for now. Handling objects outside of simulation window bounds is currently out of scope.
    // #[test]
    // fn test_update_particle_position() {
    //     // If spawned too high, a particle should not bounce off the top of the simulation
    //     // and out of the window
    //     let mut particle = Particle {
    //         x_pos: 0.0,
    //         y_pos: -1.0,
    //         x_velocity_m_s: 0.0,
    //         y_velocity_m_s: 2.0,
    //     };
    //     let result = particle_sim::update_particle_position(&mut particle, 1.0, 0.99);
    //     assert!(particle.y_pos <= 0.0);
    // }
}

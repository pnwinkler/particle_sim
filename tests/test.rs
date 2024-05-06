// use particle_sim;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // TODO
    // #[test]
    // fn test_convert_velocity_to_newtons() {
    // }

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

    // TODO: implement test once the tested function is pure(r)
    #[test]
    fn test_calulate_friction() {
        // TODO: test for objects not in contact with one another (should be 0 friction)

        // In a simple calculation, you would calculate the normal force of a 2-kg block of wood sitting on a surface as N = 2 kg × 9.8 N/kg = 19.6 N
        let result = particle_sim::calculate_friction(1.0, 2.0, 1.0);
        assert_eq!(result, 1.0);

        let result = particle_sim::calculate_friction(3.0, 1.0, 2.0);
        assert_eq!(result, -1.0);
    }

    // TODO: implement test once the tested function is pure(r)
    // #[test]
    // fn test_apply_gravity_to_particles() {
    //     let result = particle_sim::apply_gravity_to_particles();
    //     assert_eq!(result, 1.0);
    //
    //     let result = particle_sim::apply_gravity_to_particles(3.0, 1.0, 2.0);
    //     assert_eq!(result, -1.0);
    // }
}

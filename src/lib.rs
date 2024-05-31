//! A realistic particle simulator

pub mod objects;
use crate::objects::*;
use macroquad::prelude::*;
use std::fmt;

pub const SCREEN_WIDTH: f32 = 1080.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

// The default diameter in pixels of a particle
pub const PARTICLE_RADIUS_PX: f32 = 10.0;

// The default color of a particle
const PARTICLE_COLOR: Color = RED;

// Simulation parameters
pub const PIXELS_PER_METER: f32 = 10.0;
const GRAVITY_MS: f32 = 9.8;
// const FRICTION_STATIC_COEFFICIENT: f32 = 0.02;
const FRICTION_DYNAMIC_COEFFICIENT: f32 = 0.005;
const BOUNCE_COEFFICIENT: f32 = 0.9;

/* Todo: consider...
- sliders
- make friction apply on bounces
- implement spin, and update bounce logic etc accordingly
- use signed distance functions or similar to calculate when a particle may be out of bounds
- bouncing with object compressibility (more complicated)
- bouncing within a restricted space (bounding object)
- emitters (e.g. mouse emitter) + lifetimes
- collision with other particles / momentum transfer
- colored particles based on properties, e.g. velocity
- performance tests / logging
- air resistance
- shaders
- etc
*/

// Display parameters
const STATS_FONT_SIZE: f32 = 30.0;
const STATS_X_ANCHOR: f32 = SCREEN_WIDTH - (0.4 * SCREEN_WIDTH);
const STATS_COLOR: Color = GREEN;

#[derive(Debug, Clone)]
pub enum BounceError {
    CalculationDepthExceeded,
    OutOfBoundsError(OutOfBoundsError),
}

#[derive(Debug, Clone)]
pub struct OutOfBoundsError {
    object_location_x: f32,
    object_location_y: f32,
}

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: object's location (X={},Y={}) is out of bounds",
            self.object_location_x, self.object_location_y
        )
    }
}

#[derive(Debug, Clone)]
struct CalculationDepthExceeded;

/// Returns true if the point falls within a circle, else false
/// The following formula is used: a^2 + b^2 = c^2. If c <= radius, then the point is considered to be within the circle
pub fn does_circle_intersect(circle_center: XY, circle_radius: f32, point: XY) -> bool {
    // tolerance is to account for floating point imprecision
    let tolerance = 0.0001;
    let a_2 = (circle_center.x - point.x).powf(2.0);
    let b_2 = (circle_center.y - point.y).powf(2.0);
    println!("{},{},{}", a_2, b_2, circle_radius.powf(2.0));
    return (circle_radius.powf(2.0) + tolerance) >= (a_2 + b_2);
}

pub fn draw_particles(particles: &Vec<Particle>) {
    for p in particles {
        draw_circle(
            p.position.x.floor(),
            p.position.y.floor(),
            PARTICLE_RADIUS_PX,
            PARTICLE_COLOR,
        );
    }
}

pub fn convert_meters_to_pixels(meters: f32, pixels_per_meter: f32) -> f32 {
    return meters * pixels_per_meter;
}

pub fn convert_pixels_to_meters(pixels: f32, pixels_per_meter: f32) -> f32 {
    return pixels / pixels_per_meter;
}

/// Calculate the acceleration of a particle in units per second. It expects both velocities to use the same unit.
pub fn calculate_particle_acceleration(
    new_velocity: f32,
    old_velocity: f32,
    time_elapsed_seconds: f32,
) -> f32 {
    // We use the formula F = dv / dt instead of the valid alternative F=m/a
    return (new_velocity - old_velocity) / time_elapsed_seconds;
}

/// Returns a True if the input particle is touching the ground, else False. This function is not suitable for off-screen particles.
pub fn particle_touching_ground(particle: &Particle) -> bool {
    // Edge case: returns true if a particle has fallen off the bottom of the screen.
    return (particle.position.y + PARTICLE_RADIUS_PX) >= SCREEN_HEIGHT;
}

/// Calculate the signed velocity change due to friction for a particle. Returns a value <= 0 if object is moving right, else >= 0.
/// For realistic friction, the coefficients should be positive values.
/// For now, we only apply friction in the horizontal dimension and for particles in contact with the ground.
pub fn calculate_friction_deceleration(
    particle: &Particle,
    friction_dynamic_coefficient: f32,
) -> f32 {
    // We use the following formula: F=ma
    // Todo: implement static coefficient

    if !particle_touching_ground(particle) {
        return 0.0;
    }

    // Arbitrarily chosen value used to decide whether something counts as "moving" or not
    let cutoff_velocity = 0.0001;
    if particle.velocity.x.abs() < cutoff_velocity {
        return 0.0;
    }

    // friction coefficient * mass * gravity
    let f = friction_dynamic_coefficient;
    let friction_force = f * particle.mass * GRAVITY_MS;
    let friction_deceleration = friction_force / particle.mass;

    // The -1.0 multipliers let us oppose the object's velocity
    if friction_deceleration > particle.velocity.x.abs() {
        return -1.0 * particle.velocity.x;
    }
    if particle.velocity.x > 0.0 {
        return -1.0 * friction_deceleration;
    }
    return friction_deceleration;
}

/// Calculate the effect of gravity in meters over the elapsed timeframe, if it's not resting on any surface
/// (currently we only check for the ground)
pub fn calculate_gravity_effect_on_velocity(
    particle: &Particle,
    gravity_acceleration_ms: f32,
    time_elapsed_seconds: f64,
) -> f32 {
    // todo: add checks for if particle is resting on another object
    if particle_touching_ground(particle) {
        return 0.0;
    }
    return gravity_acceleration_ms * time_elapsed_seconds as f32;
}

pub struct BounceResult {
    pub position: XY,
    pub velocity: XY,
}

/// Calculate and return the post-bounce state of the input particle.
///
/// A particle may bounce 0 or more times. If the input particle's position is out of bounds, or its velocity so extreme that it exceeds calculation limits, then return an error.
/// * `particle` - the object for which bounce values should be calculated.
/// * `time_elapsed_seconds` - the time, in seconds, over which to calculate the bounce values.
/// * `bounce_coefficient` - the bounciness of the object, where 0 means no bounce, and 1 means infinite bouncing. This value must be between 0 and 1 inclusive!
///
/// Output - an object representing the updated position and velocities of the input object.
pub fn calculate_bounce(
    particle: &Particle,
    bounce_coefficient: f32,
    time_elapsed_seconds: f64,
) -> Result<BounceResult, BounceError> {
    // Todo: add bounce interactions between particles

    let p = particle;
    let mut result = BounceResult {
        position: XY {
            x: p.position.x,
            y: p.position.y,
        },
        velocity: XY {
            x: p.velocity.x,
            y: p.velocity.y,
        },
    };

    if bounce_coefficient <= 0.0001 {
        // Return the input unmodified
        return Ok(result);
    }
    if p.position.x - PARTICLE_RADIUS_PX < 0.0
        || p.position.y - PARTICLE_RADIUS_PX < 0.0
        || p.position.x + PARTICLE_RADIUS_PX > SCREEN_WIDTH
        || p.position.y + PARTICLE_RADIUS_PX > SCREEN_HEIGHT
    {
        return Err(BounceError::OutOfBoundsError(OutOfBoundsError {
            object_location_x: p.position.x,
            object_location_y: p.position.y,
        }));
    }

    assert!(bounce_coefficient < 1.0);
    let partial_res_x = bounce_helper(
        Axis::X,
        p.position.x,
        p.velocity.x,
        time_elapsed_seconds,
        bounce_coefficient,
    );
    let partial_res_y = bounce_helper(
        Axis::Y,
        p.position.y,
        p.velocity.y,
        time_elapsed_seconds,
        bounce_coefficient,
    );

    match partial_res_x {
        Ok(partial) => {
            result.position.x = partial.axis_position;
            result.velocity.x = partial.axis_velocity;
        }
        Err(_e) => {
            return Err(BounceError::CalculationDepthExceeded);
        }
    }

    match partial_res_y {
        Ok(partial) => {
            result.position.y = partial.axis_position;
            result.velocity.y = partial.axis_velocity;
        }
        Err(_e) => {
            return Err(BounceError::CalculationDepthExceeded);
        }
    }

    println!(
        "Bounce input Y: {}, output Y: {}, input X: {}, output X: {}",
        p.position.y, result.position.y, p.position.x, result.position.x
    );
    return Ok(result);
}

struct PartialBounceResult {
    pub axis_position: f32,
    pub axis_velocity: f32,
}

enum Axis {
    X,
    Y,
}

fn bounce_helper(
    axis: Axis,
    axis_position: f32,
    axis_velocity: f32,
    time_elapsed_seconds: f64,
    bounce_coefficient: f32,
) -> Result<PartialBounceResult, CalculationDepthExceeded> {
    /*
        This function helps calculate bounces
    */
    // TODO: resolve particle getting stuck on left wall
    let mut res = PartialBounceResult {
        axis_position,
        axis_velocity,
    };

    let mut counter = 0;
    // This value is arbitrarily chosen
    let max_bounce_calculations = 100;

    // These values here are signed, and indicate the direction in each axis that the particle can move
    let directional_allowance_0;
    let directional_allowance_1;
    let max_allowed_position;
    match axis {
        Axis::X => {
            // How much distance the particle can legally move left and right respectively.
            directional_allowance_0 = (PARTICLE_RADIUS_PX - axis_position).ceil();
            directional_allowance_1 = ((SCREEN_WIDTH - PARTICLE_RADIUS_PX) - axis_position).floor();
            max_allowed_position = SCREEN_WIDTH - PARTICLE_RADIUS_PX;
        }
        Axis::Y => {
            // How much distance the particle can legally move up and down respectively.
            directional_allowance_0 = (PARTICLE_RADIUS_PX - axis_position).ceil();
            directional_allowance_1 =
                ((SCREEN_HEIGHT - PARTICLE_RADIUS_PX) - axis_position).floor();
            max_allowed_position = SCREEN_HEIGHT - PARTICLE_RADIUS_PX;
        }
    }
    assert!(directional_allowance_0 <= 0.0);
    assert!(directional_allowance_1 >= 0.0);

    let time_multiplier = time_elapsed_seconds as f32;
    // Signed distance tracking the remaining amount of travel the particle can do in the examined timeframe
    let mut travel_remaining =
        convert_meters_to_pixels(axis_velocity * time_multiplier, PIXELS_PER_METER);
    let mut new_velocity = axis_velocity;
    loop {
        // Break if remaining travel distance falls within limits for travel in a given direction,
        // or the particle's moving too slowly to be worth calculating a bounce for.
        if (new_velocity.abs().floor() == 0.0)
            || ((new_velocity <= 0.0) && (travel_remaining >= directional_allowance_0))
            || ((new_velocity >= 0.0) && (travel_remaining <= directional_allowance_1))
        {
            break;
        }

        // This block is here both for performance reasons, and to rule out any possible infinite loop
        counter += 1;
        if counter == max_bounce_calculations {
            return Err(CalculationDepthExceeded);
        }

        travel_remaining = -1.0 * travel_remaining * bounce_coefficient;
        new_velocity = -1.0 * new_velocity * bounce_coefficient;
        println!(
            "T: {},\t\t{}, \t\t{}, \t\t{}",
            travel_remaining, new_velocity, directional_allowance_0, directional_allowance_1
        );
    }
    res.axis_position = axis_position + travel_remaining;
    res.axis_velocity = new_velocity;

    // The break condition in the block above can result in the new position being out of bounds.
    // TODO: think if this velocity nullification makes sense in the X axis? Doesn't this circumvent friction?
    if res.axis_position > max_allowed_position {
        // When this condition is true, the object has negigible velocity and is more or less on
        // the ground, so we can safely nullify its velocity
        res.axis_position = max_allowed_position;
        res.axis_velocity = 0.0;
    }
    if res.axis_position < PARTICLE_RADIUS_PX {
        res.axis_position = PARTICLE_RADIUS_PX;
    }

    return Ok(res);
}

/// Update a particle's properties, while remaining within a range of acceptable values. Also reset the velocity of off-screen particles, and clamp it to be within arena bounds
pub fn set_particle_properties_within_bounds(
    particle: &mut Particle,
    new_x_pos: f32,
    new_y_pos: f32,
    new_x_velocity: f32,
    new_y_velocity: f32,
) {
    let p = particle;
    p.position.x = new_x_pos;
    p.position.y = new_y_pos;
    p.velocity.x = new_x_velocity;
    p.velocity.y = new_y_velocity;

    if SCREEN_HEIGHT < (p.position.y + PARTICLE_RADIUS_PX).floor() {
        println!(
            "DEBUG: particle fully or partially off-screen at Y={}",
            p.position.y
        );
        p.position.y = SCREEN_HEIGHT - PARTICLE_RADIUS_PX;
        p.velocity.y = 0.0;
    } else if 0.0 > (p.position.y - PARTICLE_RADIUS_PX).ceil() {
        println!(
            "DEBUG: particle fully or partially off-screen at Y={}",
            p.position.y
        );
        p.position.y = PARTICLE_RADIUS_PX;
        p.velocity.y = 0.0;
    }

    if SCREEN_WIDTH < (p.position.x + PARTICLE_RADIUS_PX).floor() {
        println!(
            "DEBUG: particle fully or partially off-screen at X={}",
            p.position.x
        );
        p.position.x = SCREEN_WIDTH - PARTICLE_RADIUS_PX;
        p.velocity.x = 0.0;
    } else if 0.0 > (p.position.x - PARTICLE_RADIUS_PX).ceil() {
        println!(
            "DEBUG: particle fully or partially off-screen at X={}",
            p.position.x
        );
        p.position.x = PARTICLE_RADIUS_PX;
        p.velocity.x = 0.0;
    }
}

/// Draw simulation stats to screen
pub fn draw_stats(particles: &Vec<Particle>) {
    // TODO: fix sum_y_positions so that it doesn't overflow or nan or whatever with 1000 particles
    let mut sum_y_velocity = 0.0;
    let mut sum_x_velocity = 0.0;
    let mut sum_y_positions = 0.0;
    for p in particles {
        sum_y_velocity += p.velocity.y;
        sum_x_velocity += p.velocity.x;
        sum_y_positions += p.position.y;
    }
    let avg_y_velocity = sum_y_velocity / particles.len() as f32;
    let avg_x_velocity = sum_x_velocity / particles.len() as f32;
    let y_velocity_str = "Mean Y velocity: ".to_owned() + &avg_y_velocity.to_string();
    let x_velocity_str = "Mean X velocity: ".to_owned() + &avg_x_velocity.to_string();

    let sim_height_meters = SCREEN_HEIGHT / PIXELS_PER_METER;
    let sim_height_meters_str = "Sim height meters: ".to_owned() + &sim_height_meters.to_string();

    let particle_count_str = "Particle count: ".to_owned() + &particles.len().to_string();

    let particle_mean_altitude_px = SCREEN_HEIGHT - (sum_y_positions / particles.len() as f32);
    let particle_mean_altitude_meters =
        convert_pixels_to_meters(particle_mean_altitude_px, PIXELS_PER_METER);
    let particle_mean_altitude_str =
        "Mean altitude (m): ".to_owned() + &particle_mean_altitude_meters.to_string();

    let strings: [&String; 6] = [
        &get_fps().to_string(),
        &y_velocity_str,
        &x_velocity_str,
        &sim_height_meters_str,
        &particle_mean_altitude_str,
        &particle_count_str,
    ];

    let y_offset = 30.0;
    for (idx, s) in strings.iter().enumerate() {
        draw_text(
            &s,
            STATS_X_ANCHOR,
            idx as f32 * y_offset + y_offset,
            STATS_FONT_SIZE,
            STATS_COLOR,
        );
    }
}

pub fn simulation_tick(particles: &mut Vec<Particle>, time_elapsed_seconds: f64) {
    for p in particles.iter_mut() {
        println!(
            "Before calculations: Y={}, Y_vel={}, X={}, X_vel={}",
            p.position.y, p.velocity.y, p.position.x, p.velocity.x
        );

        p.velocity.y += calculate_gravity_effect_on_velocity(p, GRAVITY_MS, time_elapsed_seconds);

        p.velocity.x += calculate_friction_deceleration(p, FRICTION_DYNAMIC_COEFFICIENT);

        let bounce_result = calculate_bounce(p, BOUNCE_COEFFICIENT, time_elapsed_seconds);

        match bounce_result {
            Ok(bounce_result) => {
                set_particle_properties_within_bounds(
                    p,
                    bounce_result.position.x,
                    bounce_result.position.y,
                    bounce_result.velocity.x,
                    bounce_result.velocity.y,
                );
            }
            Err(e) => {
                match e {
                    BounceError::CalculationDepthExceeded => {
                        println!("Warning! Calculation depth exceeded when calculating bounces. Resetting particle parameters");
                    }
                    BounceError::OutOfBoundsError(oob) => {
                        println!("Warning! Failed to calculate bounces. Input particle out of bounds. Resetting particle parameters");
                        println!("{}", oob);
                    }
                }
                set_particle_properties_within_bounds(
                    p,
                    0.5 * SCREEN_WIDTH,
                    0.5 * SCREEN_HEIGHT,
                    0.0,
                    0.0,
                );
            }
        }

        println!(
            "After calculations: Y={}, Y_vel={}, X={}, X_vel={}",
            p.position.y, p.velocity.y, p.position.x, p.velocity.x
        );
    }
}

pub async fn p_main() {
    // Setup
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut particles: Vec<Particle> = Vec::new();
    particles.push(Particle {
        position: XY {
            x: 0.5 * SCREEN_WIDTH,
            y: convert_meters_to_pixels(72.0 - 50.0, PIXELS_PER_METER),
        }, // 0.25 * SCREEN_HEIGHT,
        velocity: XY { x: 55.0, y: 1.0 },
        force: XY { x: 0.0, y: 0.0 },
        mass: 1.0,
    });

    // As of 2024-05-09, 2550 is my maximum number of particles for constant >= 140 FPS
    // for x in 1..500 {
    //     particles.push(Particle {
    //         x_pos: PARTICLE_RADIUS_PX + (x * 1) as f32,
    //         y_pos: PARTICLE_RADIUS_PX +  (x * 1) as f32,
    //         x_velocity_m_s: 250.0,
    //         y_velocity_m_s: 250.0,
    //     });
    // }

    // Constraint checks: check for any unsupported parameter values that aren't obviously ridiculous.
    // A negative bounce coefficient makes no sense. Either an object bounces (val >=0) or doesn't (val == 0)
    assert!(BOUNCE_COEFFICIENT >= 0.0);
    // For accurate results, the particle should spawn fully within simulation bounds
    assert!(particles[0].position.x >= PARTICLE_RADIUS_PX);
    assert!(particles[0].position.y >= PARTICLE_RADIUS_PX);
    assert!(particles[0].position.x <= SCREEN_WIDTH - PARTICLE_RADIUS_PX);
    assert!(particles[0].position.y <= SCREEN_HEIGHT - PARTICLE_RADIUS_PX);

    let mut last_tick_time = get_time();

    // Main loop
    loop {
        let now = get_time();
        let time_elapsed = now - last_tick_time;
        simulation_tick(&mut particles, time_elapsed);

        // FPS limiter copied from https://github.com/not-fl3/macroquad/issues/380#issuecomment-1026728046
        // let minimum_frame_time = 1. / 1.; // 60 FPS
        // let frame_time = get_frame_time();
        // println!("Frame time: {}ms", frame_time * 1000.);
        // if frame_time < minimum_frame_time {
        //     let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        //     println!("Sleep for {}ms", time_to_sleep);
        //     std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        // }

        clear_background(BLACK);
        draw_particles(&particles);
        draw_stats(&particles);

        last_tick_time = now;

        next_frame().await
    }
}

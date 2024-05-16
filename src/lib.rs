//! A realistic particle simulator

use macroquad::prelude::*;
use std::fmt;

pub const SCREEN_WIDTH: f32 = 1080.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

// The default diameter in pixels of a particle
const PARTICLE_RADIUS_PX: f32 = 10.0;

// The weight of each particle in kilograms
const PARTICLE_MASS_KG: f32 = 1.0;

// The default color of a particle
const PARTICLE_COLOR: Color = RED;

// Simulation parameters
const PIXELS_PER_METER: f32 = 10.0;
const GRAVITY_MS: f32 = 9.8;
// const FRICTION_STATIC_COEFFICIENT: f32 = 0.02;
const FRICTION_DYNAMIC_COEFFICIENT: f32 = 0.005;
const BOUNCE_COEFFICIENT: f32 = 0.9;

/* Todo: consider...
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
            "Object's location (X={},Y={}) is out of bounds",
            self.object_location_x, self.object_location_y
        )
    }
}

#[derive(Debug, Clone)]
struct CalculationDepthExceeded;

pub struct Particle {
    // Particle position in pixels
    pub x_pos: f32,
    pub y_pos: f32,

    // Signed particle velocity in meters per second
    pub x_velocity_m_s: f32,
    pub y_velocity_m_s: f32,
}

pub struct XY {
    pub x: f32,
    pub y: f32,
}

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
            p.x_pos.floor(),
            p.y_pos.floor(),
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
    return (particle.y_pos + PARTICLE_RADIUS_PX) >= SCREEN_HEIGHT;
}

/// Calculate the signed velocity change due to friction for a particle. Returns a value <= 0 if object is moving right, else >= 0.
/// For realistic friction, the coefficients should be positive values.
/// For now, we only apply friction in the horizontal dimension and for particles in contact with the ground.
pub fn calculate_friction_deceleration(
    particle: &Particle,
    friction_dynamic_coefficient: f32,
) -> f32 {
    // We use the following formula: F=ma
    // TODO: implement static coefficient

    if !particle_touching_ground(particle) {
        return 0.0;
    }

    // arbitrarily chosen value used to decide whether something counts as "moving" or not
    let cutoff_velocity = 0.0001;
    if particle.x_velocity_m_s.abs() < cutoff_velocity {
        return 0.0;
    }

    // friction coefficient * mass * gravity
    let f = friction_dynamic_coefficient;
    let friction_force = f * PARTICLE_MASS_KG * GRAVITY_MS;
    let friction_deceleration = friction_force / PARTICLE_MASS_KG;

    // The -1.0 multipliers let us oppose the object's velocity
    if friction_deceleration > particle.x_velocity_m_s.abs() {
        return -1.0 * particle.x_velocity_m_s;
    }
    if particle.x_velocity_m_s > 0.0 {
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
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
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
    time_elapsed_seconds: f64,
    bounce_coefficient: f32,
) -> Result<BounceResult, BounceError> {
    // todo: add bounce interactions between particles

    let p = particle;
    let mut result = BounceResult {
        x_pos: p.x_pos,
        y_pos: p.y_pos,
        x_velocity: p.x_velocity_m_s,
        y_velocity: p.y_velocity_m_s,
    };

    if bounce_coefficient <= 0.0001 {
        // Return the input unmodified
        return Ok(result);
    }
    if p.x_pos - PARTICLE_RADIUS_PX < 0.0
        || p.y_pos - PARTICLE_RADIUS_PX < 0.0
        || p.x_pos + PARTICLE_RADIUS_PX > SCREEN_WIDTH
        || p.y_pos + PARTICLE_RADIUS_PX > SCREEN_HEIGHT
    {
        return Err(BounceError::OutOfBoundsError(OutOfBoundsError {
            object_location_x: p.x_pos,
            object_location_y: p.y_pos,
        }));
    }

    assert!(bounce_coefficient < 1.0);
    // TODO: add tests
    let partial_res_x = bounce_helper(
        Axis::X,
        p.x_pos,
        p.x_velocity_m_s,
        time_elapsed_seconds,
        bounce_coefficient,
    );
    let partial_res_y = bounce_helper(
        Axis::Y,
        p.y_pos,
        p.y_velocity_m_s,
        time_elapsed_seconds,
        bounce_coefficient,
    );

    match partial_res_x {
        Ok(partial) => {
            result.x_pos = partial.axis_position;
            result.x_velocity = partial.axis_velocity;
        }
        Err(_e) => {
            return Err(BounceError::CalculationDepthExceeded);
        }
    }

    match partial_res_y {
        Ok(partial) => {
            result.y_pos = partial.axis_position;
            result.y_velocity = partial.axis_velocity;
        }
        Err(_e) => {
            return Err(BounceError::CalculationDepthExceeded);
        }
    }

    println!("INPUT Y: {}, OUTPUT_Y: {}", p.y_pos, result.y_pos);
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
    // TODO: resolve jitter on floor
    // TODO: resolve particle getting stuck on left wall
    let mut res = PartialBounceResult {
        axis_position,
        axis_velocity,
    };

    let time_multiplier = time_elapsed_seconds as f32;
    let mut counter = 0;
    // This value is arbitrarily chosen
    let max_bounce_calculations = 100;

    // These values here are signed, and indicate the direction in each axis that the particle can move
    let directional_allowance_0;
    let directional_allowance_1;
    match axis {
        Axis::X => {
            // How much distance the particle can legally move left and right respectively.
            directional_allowance_0 = (PARTICLE_RADIUS_PX - axis_position).floor();
            directional_allowance_1 = ((SCREEN_WIDTH - PARTICLE_RADIUS_PX) - axis_position).floor();
        }
        Axis::Y => {
            // How much distance the particle can legally move up and down respectively.
            directional_allowance_0 = (PARTICLE_RADIUS_PX - axis_position).floor();
            directional_allowance_1 =
                ((SCREEN_HEIGHT - PARTICLE_RADIUS_PX) - axis_position).floor();
        }
    }
    assert!(directional_allowance_0 <= 0.0);
    assert!(directional_allowance_1 >= 0.0);

    // signed distance tracking the remaining amount of travel the particle will do in the examined timeframe
    let mut travel_remaining =
        convert_meters_to_pixels(axis_velocity * time_multiplier, PIXELS_PER_METER);
    let mut new_velocity = axis_velocity;
    loop {
        // Break if remaining travel distance falls within limits for travel in a given direction
        if (new_velocity.abs().floor() == 0.0)
            || ((new_velocity <= 0.0) && (travel_remaining >= directional_allowance_0))
            || ((new_velocity >= 0.0) && (travel_remaining <= directional_allowance_1))
        {
            break;
        }

        // This block is here both for performance reasons, and to rule out any possible infinite loop
        counter += 1;
        if counter == max_bounce_calculations {
            // let str = format!(
            //     "Warning! Maximum number of bounces per tick ({}) exceeded.",
            //     max_bounce_calculations
            // );
            return Err(CalculationDepthExceeded);
        }

        travel_remaining = -1.0 * travel_remaining * bounce_coefficient;
        new_velocity = -1.0 * new_velocity * bounce_coefficient;
        println!(
            "Y: {},\t\t{}, \t\t{}, \t\t{}",
            travel_remaining, new_velocity, directional_allowance_0, directional_allowance_1
        );
    }
    res.axis_position = axis_position + travel_remaining;
    res.axis_velocity = new_velocity;
    return Ok(res);
}

/// Return the input number with decimal places all set to 0
// pub fn remove_mantissa(num: f32) -> f32 {
//     if num > 0.0 {
//         return num.floor();
//     } else {
//         return (num + 1.0).floor();
//     }
// }

/// Update a particle's properties, while remaining within a range of acceptable values. Also reset the velocity of off-screen particles, and clamp it to be within arena bounds
pub fn set_particle_properties_within_bounds(
    particle: &mut Particle,
    new_x_pos: f32,
    new_y_pos: f32,
    new_x_velocity: f32,
    new_y_velocity: f32,
) {
    let p = particle;
    p.x_pos = new_x_pos;
    p.y_pos = new_y_pos;
    p.x_velocity_m_s = new_x_velocity;
    p.y_velocity_m_s = new_y_velocity;

    if SCREEN_HEIGHT < (p.y_pos + PARTICLE_RADIUS_PX).floor() {
        println!("DEBUG a: {}, {}", p.y_pos, p.y_pos + PARTICLE_RADIUS_PX);
        p.y_pos = SCREEN_HEIGHT - PARTICLE_RADIUS_PX;
        p.y_velocity_m_s = 0.0;
    } else if 0.0 > (p.y_pos - PARTICLE_RADIUS_PX).ceil() {
        println!("DEBUG b: {}", p.y_pos);
        p.y_pos = PARTICLE_RADIUS_PX;
        p.y_velocity_m_s = 0.0;
    }

    if SCREEN_WIDTH < (p.x_pos + PARTICLE_RADIUS_PX).floor() {
        p.x_pos = SCREEN_WIDTH - PARTICLE_RADIUS_PX;
        p.x_velocity_m_s = 0.0;
    } else if 0.0 > (p.x_pos - PARTICLE_RADIUS_PX).ceil() {
        p.x_pos = PARTICLE_RADIUS_PX;
        p.x_velocity_m_s = 0.0;
    }
}

/// Draw simulation stats to screen
pub fn draw_stats(particles: &Vec<Particle>) {
    // todo: fix sum_y_positions so that it doesn't overflow or nan or whatever with 1000 particles
    let mut sum_y_velocity = 0.0;
    let mut sum_x_velocity = 0.0;
    let mut sum_y_positions = 0.0;
    for p in particles {
        sum_y_velocity += p.y_velocity_m_s;
        sum_x_velocity += p.x_velocity_m_s;
        sum_y_positions += p.y_pos;
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

pub async fn p_main() {
    // Note that alt-tabbing or otherwise removing focus from the window may affect simulation results
    //  (my guess is this is due to the OS / graphics driver or something reducing the framerate of unfocused windows)

    // Setup
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut particles: Vec<Particle> = Vec::new();
    // particles.push(Particle {
    //     x_pos: 0.5 * SCREEN_WIDTH,
    //     y_pos: convert_meters_to_pixels(72.0 - 50.0, PIXELS_PER_METER), // 0.25 * SCREEN_HEIGHT,
    //     x_velocity_m_s: 55.0,
    //     y_velocity_m_s: 1.0,
    // });

    let initial_y_velocity = -0.5 * SCREEN_HEIGHT - 1.0;
    particles.push(Particle {
        x_pos: 0.5 * SCREEN_WIDTH,
        y_pos: 0.5 * SCREEN_HEIGHT, // arbitrarily chosen, but we want the particle not already colliding on spawn
        x_velocity_m_s: 0.0,
        y_velocity_m_s: initial_y_velocity, // we want the ball to hit the ground within 1 tick
    });

    // As of 2024-05-09, 2550 is my maximum number of particles for constant >= 140 FPS
    // for x in 1..2550 {
    //     particles.push(Particle {
    //         x_pos: PARTICLE_RADIUS_PX + (x * 1) as f32,
    //         y_pos: PARTICLE_RADIUS_PX +  (x * 1) as f32,
    //         x_velocity_m_s: 250.0,
    //         y_velocity_m_s: 250.0,
    //     });
    // }

    let mut last_tick_time = get_time();

    // Constraint checks: check for any unsupported parameter values that aren't obviously ridiculous.
    // A negative bounce coefficient makes no sense. Either an object bounces (val >=0) or doesn't (val == 0)
    assert!(BOUNCE_COEFFICIENT >= 0.0);
    // The particle needs to spawn fully within simulation bounds
    assert!(particles[0].x_pos >= PARTICLE_RADIUS_PX);
    assert!(particles[0].y_pos >= PARTICLE_RADIUS_PX);
    assert!(particles[0].x_pos <= SCREEN_WIDTH - PARTICLE_RADIUS_PX);
    assert!(particles[0].y_pos <= SCREEN_HEIGHT - PARTICLE_RADIUS_PX);

    // Main loop
    loop {
        let now = get_time();
        let time_elapsed = now - last_tick_time;
        clear_background(BLACK);

        // FPS limiter copied from https://github.com/not-fl3/macroquad/issues/380#issuecomment-1026728046
        // let minimum_frame_time = 1. / 1.; // 60 FPS
        // let frame_time = get_frame_time();
        // println!("Frame time: {}ms", frame_time * 1000.);
        // if frame_time < minimum_frame_time {
        //     let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        //     println!("Sleep for {}ms", time_to_sleep);
        //     std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        // }

        for p in particles.iter_mut() {
            println!("BEFORE: {} pos\t{} vel\t", p.y_pos, p.y_velocity_m_s);

            p.y_velocity_m_s += calculate_gravity_effect_on_velocity(p, GRAVITY_MS, time_elapsed);

            p.x_velocity_m_s += calculate_friction_deceleration(p, FRICTION_DYNAMIC_COEFFICIENT);

            let bounce_result = calculate_bounce(p, time_elapsed, BOUNCE_COEFFICIENT);

            match bounce_result {
                Ok(bounce_result) => {
                    set_particle_properties_within_bounds(
                        p,
                        bounce_result.x_pos,
                        bounce_result.y_pos,
                        bounce_result.x_velocity,
                        bounce_result.y_velocity,
                    );
                }
                // todo: split the handling of different errors out, here
                Err(_error) => {
                    println!("Warning! Error occurred when calculating bounces. Resetting particle parameters");
                    set_particle_properties_within_bounds(
                        p,
                        0.5 * SCREEN_WIDTH,
                        0.5 * SCREEN_HEIGHT,
                        0.0,
                        0.0,
                    );
                }
            }

            // todo: resolve why this function clamps particles to the left or right hand side of the screen
            // clamp_particle_position_to_screen(p, false);
            println!("AFTER: {} pos\t{} vel", p.y_pos, p.y_velocity_m_s);
        }
        draw_particles(&particles);

        draw_stats(&particles);

        last_tick_time = now;

        next_frame().await
    }
}

use macroquad::prelude::*;

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
- fix bouncing
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

pub fn does_circle_intersect(circle_center: XY, circle_radius: f32, point: XY) -> bool {
    // Returns true if the point falls within a circle, else false
    // a^2 + b^2 = c^2. If c <= radius, then the point is within the circle

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

pub fn calculate_particle_acceleration(
    new_velocity: f32,
    old_velocity: f32,
    time_elapsed_seconds: f32,
) -> f32 {
    /*
    Calculate the acceleration of a particle.
    We use the formula F = dv / dt instead of the valid alternative F=m/a
    */

    return (new_velocity - old_velocity) / time_elapsed_seconds;
}

pub fn particle_touching_ground(particle: &Particle) -> bool {
    // Return True if the location of any pixel within a particle has a y location greater than or equal to that of the floor
    // Else, False
    return (particle.y_pos + PARTICLE_RADIUS_PX) >= SCREEN_HEIGHT;
}

pub fn calculate_friction_deceleration(
    particle: &Particle,
    friction_dynamic_coefficient: f32,
) -> f32 {
    /*
    Calculate friction deceleration for a particle. Returns a value <= 0 if object is moving right, else >= 0.
    For realistic friction, the coefficients should be positive values.
    For now, we only apply friction in the horizontal dimension and for particles in contact with the ground.
    We use the following formulas:
        F=ma
    */
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

    // We need to oppose the object's velocity
    if particle.x_velocity_m_s > 0.0 {
        if friction_deceleration > particle.x_velocity_m_s {
            return -1.0 * particle.x_velocity_m_s;
        }
        return -1.0 * friction_deceleration;
    }

    if particle.x_velocity_m_s < 0.0 {
        if friction_deceleration > particle.x_velocity_m_s.abs() {
            return -1.0 * particle.x_velocity_m_s;
        }
    }
    return friction_deceleration;
}

pub fn calculate_gravity_effect_on_velocity(
    gravity_acceleration_ms: f32,
    time_elapsed_seconds: f64,
) -> f32 {
    // Calculate the effect of gravity in meters over the elapsed timeframe.
    return gravity_acceleration_ms * time_elapsed_seconds as f32;
}

fn distance_out_of_bounds(pixel_location: f32, axis_min_val: f32, axis_max_val: f32) -> f32 {
    // Return the unsigned distance out of bounds on a given axis that a pixel is
    // A negative value means the pixel is out of bounds on the left, a positive, on the right.
    if pixel_location < axis_min_val {
        return axis_min_val - pixel_location;
    } else if pixel_location > axis_max_val {
        return pixel_location - axis_max_val;
    }
    return 0.0;
}

pub struct BounceResult {
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
}

pub fn calculate_bounce(
    particle: &Particle,
    time_elapsed_seconds: f64,
    bounce_coefficient: f32,
) -> BounceResult {
    // If a bounce coefficient is provided, then bounce the particle upon reaching the ground.
    // todo: add bounce interactions between particles

    let p = particle;
    let mut result = BounceResult {
        x_pos: p.x_pos,
        y_pos: p.y_pos,
        x_velocity: p.x_velocity_m_s,
        y_velocity: p.y_velocity_m_s,
    };

    if bounce_coefficient <= 0.0001 {
        // clamp_particle_position_to_screen(p, true);
        return result;
    }

    // Using the particle's velocity, update its pixel position, while respecting the ratio of pixels per meter.
    let time_multiplier = time_elapsed_seconds as f32;
    result.y_pos += convert_meters_to_pixels(p.y_velocity_m_s * time_multiplier, PIXELS_PER_METER);
    result.x_pos += convert_meters_to_pixels(p.x_velocity_m_s * time_multiplier, PIXELS_PER_METER);

    // Implementation quirk: both of these overshoots can simultaneously be > 0
    let overshoot_left = distance_out_of_bounds(p.x_pos - PARTICLE_RADIUS_PX, 0.0, SCREEN_WIDTH);
    let overshoot_right = distance_out_of_bounds(p.x_pos + PARTICLE_RADIUS_PX, 0.0, SCREEN_WIDTH);
    // TODO: handle massive overshoots, e.g. out to 5950
    println!("{}, {}", overshoot_left, result.x_pos);
    if overshoot_left > 0.0 && p.x_velocity_m_s < 0.0 {
        // This is the fraction of time that was spent overshooting
        println!("BOUNCE LEFT");
        let fraction = overshoot_left / p.x_velocity_m_s;
        result.x_velocity = -1.0 * (bounce_coefficient * result.x_velocity);
        result.x_pos =
            result.x_pos + overshoot_left - (overshoot_left * (result.x_velocity * fraction));
    } else if overshoot_right > 0.0 && p.x_velocity_m_s > 0.0 {
        println!("BOUNCE RIGHT");
        let fraction = overshoot_right / p.x_velocity_m_s;
        result.x_velocity = -1.0 * (bounce_coefficient * result.x_velocity);
        result.x_pos =
            result.x_pos - overshoot_right + (overshoot_right * (result.x_velocity * fraction));
    }

    let overshoot_top =
        distance_out_of_bounds(result.y_pos - PARTICLE_RADIUS_PX, 0.0, SCREEN_HEIGHT);
    let overshoot_bot =
        distance_out_of_bounds(result.y_pos + PARTICLE_RADIUS_PX, 0.0, SCREEN_HEIGHT);
    if overshoot_top > 0.0 && p.y_velocity_m_s < 0.0 {
        println!("BOUNCE TOP");
        let fraction = overshoot_top / result.y_velocity;
        result.y_velocity = -1.0 * (bounce_coefficient * result.y_velocity);
        result.y_pos =
            result.y_pos + overshoot_top - (overshoot_top * (result.y_velocity * fraction));
    } else if overshoot_bot > 0.0 && p.y_velocity_m_s > 0.0 {
        println!("BOUNCE BOT");
        let fraction = overshoot_bot / result.y_velocity;
        result.y_velocity = -1.0 * (bounce_coefficient * result.y_velocity);
        result.y_pos =
            result.y_pos - overshoot_bot + (overshoot_bot * (result.y_velocity * fraction));
    }

    // DEBUG code
    // print!(
    //     "{},{},{},{} VERSUS ",
    //     p.x_pos, p.y_pos, p.x_velocity_m_s, p.y_velocity_m_s
    // );
    // println!(
    //     "{},{},{},{}",
    //     result.x_pos, result.y_pos, result.x_velocity, result.y_velocity
    // );
    return result;
}

pub fn update_particle_position(
    particle: &mut Particle,
    new_x_pos: f32,
    new_y_pos: f32,
    new_x_velocity: f32,
    new_y_velocity: f32,
) {
    particle.x_pos = new_x_pos;
    particle.y_pos = new_y_pos;
    particle.x_velocity_m_s = new_x_velocity;
    particle.y_velocity_m_s = new_y_velocity;
}

pub fn clamp_particle_position_to_screen(particle: &mut Particle, reset_velocity: bool) {
    // If the particle would be off-screen, move it back on-screen. If reset_velocity, then set its velocity in that axis to zero.
    let p = particle;
    let mut new_y_velocity = p.y_velocity_m_s;
    let mut new_x_velocity = p.x_velocity_m_s;
    if reset_velocity {
        new_y_velocity = 0.0;
        new_x_velocity = 0.0;
    }

    if SCREEN_HEIGHT < (p.y_pos + PARTICLE_RADIUS_PX) {
        p.y_pos = SCREEN_HEIGHT - PARTICLE_RADIUS_PX - 1.0;
        p.y_velocity_m_s = new_y_velocity;
    } else if 0.0 > (p.y_pos - PARTICLE_RADIUS_PX) {
        p.y_pos = PARTICLE_RADIUS_PX + 1.0;
        p.y_velocity_m_s = new_y_velocity;
    }

    if SCREEN_WIDTH < (p.x_pos + PARTICLE_RADIUS_PX) {
        p.x_pos = SCREEN_WIDTH - (PARTICLE_RADIUS_PX + 1.0);
        p.x_velocity_m_s = new_x_velocity;
    } else if 0.0 > (p.x_pos - PARTICLE_RADIUS_PX) {
        // p.x_pos = PARTICLE_RADIUS_PX + 1.0;
        // p.x_velocity_m_s = new_x_velocity;
    }
}

pub fn draw_stats(particles: &Vec<Particle>) {
    // Draw stats to screen

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

    // println!(
    //     "{},{},{}",
    //     sum_y_positions, particle_mean_altitude_px, particle_mean_altitude_meters
    // );

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
    // Setup

    // Note that very fast speeds (in the range of >= 200 m/s) may, for now, lead to erratic behavior.
    // Note also, that alt-tabbing or otherwise removing focus from the window may affect simulation results
    //  (my guess is this is due to the OS / graphics driver or something reducing the framerate of unfocused windows)

    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut particles: Vec<Particle> = Vec::new();
    particles.push(Particle {
        x_pos: 0.5 * SCREEN_WIDTH,
        y_pos: 0.5 * SCREEN_HEIGHT,
        x_velocity_m_s: 0.5 * SCREEN_WIDTH + 1.0,
        y_velocity_m_s: 0.0,
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

        for p in particles.iter_mut() {
            print!("BEFORE: {} pos\t{} vel\t", p.x_pos, p.x_velocity_m_s);
            p.y_velocity_m_s += calculate_gravity_effect_on_velocity(GRAVITY_MS, time_elapsed);

            p.x_velocity_m_s += calculate_friction_deceleration(p, FRICTION_DYNAMIC_COEFFICIENT);

            let bounce_result = calculate_bounce(p, time_elapsed, BOUNCE_COEFFICIENT);

            update_particle_position(
                p,
                bounce_result.x_pos,
                bounce_result.y_pos,
                bounce_result.x_velocity,
                bounce_result.y_velocity,
            );

            // todo: resolve why this function clamps particles to the left or right hand side of the screen
            // clamp_particle_position_to_screen(p, false);
            println!("AFTER: {} pos\t{} vel", p.x_pos, p.x_velocity_m_s);
        }
        draw_particles(&particles);

        draw_stats(&particles);

        last_tick_time = now;

        next_frame().await
    }
}

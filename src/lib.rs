use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 1080.0;
const SCREEN_HEIGHT: f32 = 720.0;
// const FPS: u16 = 60;

// The default color of a particle
const PARTICLE_COLOR: Color = RED;

// The default diameter in pixels of a particle
const PARTICLE_RADIUS_PX: f32 = 10.0;

// The weight of each particle in kilograms
const PARTICLE_MASS_KG: f32 = 1.0;

// Simulation parameters
const PIXELS_PER_METER: f32 = 10.0;
const GRAVITY_MS: f32 = 9.8;
const FRICTION_STATIC_COEFFICIENT: f32 = 0.6;
const FRICTION_DYNAMIC_COEFFICIENT: f32 = 0.5;

/* Todo: consider...
- friction
- separate pure from impure code (e.g. determine new values in pure functions and assign in impure functions)
- add tests
- bouncing
- bouncing within a restricted space (bounding object)
- emitters (e.g. mouse emitter) + lifetimes
- collision with other particles / momentum transfer
- colored particles based on properties, e.g. velocity
- performance tests / logging
- air resistance
- shaders
- etc
*/

const FONT_SIZE: f32 = 30.0;

pub struct Particle {
    // Particle position in pixels
    x_pos: f32,
    y_pos: f32,

    // Signed particle velocity in meters per second
    x_velocity_m_s: f32,
    y_velocity_m_s: f32,

    // Particle lifetime in milliseconds
    lifetime_ms: u16,
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

pub fn convert_newtons_to_velocity(newtons: f32, object_mass_kg: f32) -> f32 {
    // For a given object, return its newtons converted into velocity
    return 3.0; // TODO UNDO
}

pub fn convert_velocity_to_newtons(
    old_velocity: f32,
    new_velocity: f32,
    time_elapsed_seconds: f32,
) -> f32 {
    // TODO: don't forget to encode direction or something
    // !! TODO: confirm that this will not return NEGATIVE Newtons when decelerating !!

    let acceleration =
        calculate_particle_acceleration(new_velocity, old_velocity, time_elapsed_seconds);
    // newtons, KG, m/s^2
    let newton_result = PARTICLE_MASS_KG * acceleration;
    return newton_result;
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

// TODO: rework this function. It's confusing that it returns a velocity
pub fn calculate_friction(
    particle: &Particle,
    friction_static_coefficient: f32,
    friction_dynamic_coefficient: f32,
) -> f32 {
    /*
    Calculate and apply friction to a particle.
    The coefficients should be positive values.
    For now, we only apply friction in the horizontal dimension and for particles in contact with the ground.
    We use the following formulas:
        μ = F/N
            where μ is friction, F is the frictional force and N is the normal force
        N = mg
            where N is the normal force, m is the object's mass, and g the acceleration due to gravity
            we use this formula in cases where our target object rests on top of a flat object
        N = mg cos(θ)
            as above, but theta is the angle of incline of the supporting surface
    */

    // arbitrarily chosen friction coefficient values (the F in μ = F/N)
    // let f_static = 0.6;
    // let f_dynamic = 0.5;

    // if object isn't moving, use static
    // else dynamic
    if !particle_touching_ground(particle) {
        return 0.0;
    }

    // arbitrarily chosen value used to decide whether something counts as "moving" or not
    let cutoff_velocity = 0.001;

    let moving = particle.x_velocity_m_s > cutoff_velocity;
    let f = if moving {
        friction_dynamic_coefficient
    } else {
        friction_static_coefficient
    };
    // for now we assume a flat surface, so we ignore theta
    // TODO: review formula. Does this make sense?
    let n = PARTICLE_MASS_KG * GRAVITY_MS;
    let result_newtons = f / n;
    // TODO: debug. This seems high
    println!("Friction in Newtons: {}", result_newtons);
    println!("Friction in velocity: {}", convert_newtons_to_velocity(result_newtons, PARTICLE_MASS_KG));

    // arbitrary cut-off
    if particle.x_velocity_m_s < cutoff_velocity {
        return 0.0;
    }

    return convert_newtons_to_velocity(result_newtons, PARTICLE_MASS_KG);
}

pub fn apply_velocity_to_particle_position(particle: &mut Particle, time_elapsed_seconds: f64) {
    let mut p = particle;
    let multiplier = time_elapsed_seconds as f32;
    p.y_pos += convert_meters_to_pixels(p.y_velocity_m_s * multiplier, PIXELS_PER_METER);
    p.x_pos += convert_meters_to_pixels(p.x_velocity_m_s * multiplier, PIXELS_PER_METER);
}

pub fn clamp_particle_position_to_screen(particle: &mut Particle, time_elapsed_seconds: f64) {
    // Update the particle's position based on already calculated forces
    let mut p = particle;
    if SCREEN_HEIGHT < (p.y_pos + PARTICLE_RADIUS_PX) {
        p.y_pos = SCREEN_HEIGHT - PARTICLE_RADIUS_PX;
        p.y_velocity_m_s = 0.0;
    } else if 0.0 > (p.y_pos - PARTICLE_RADIUS_PX) {
        p.y_pos = 50.0;
    }

    if SCREEN_WIDTH < (p.x_pos + PARTICLE_RADIUS_PX) {
        p.x_pos = SCREEN_WIDTH - PARTICLE_RADIUS_PX;
        p.x_velocity_m_s = 0.0;
    } else if 0.0 > (p.x_pos - PARTICLE_RADIUS_PX) {
        p.x_pos = PARTICLE_RADIUS_PX;
        p.x_velocity_m_s = 0.0;
    }
}

pub fn calculate_gravity(time_elapsed_seconds: f64) -> f32 {
    return GRAVITY_MS * time_elapsed_seconds as f32;
}

pub fn draw_stats(particles: &Vec<Particle>, last_tick_time: f64) {
    // fps, mean y velocity, mean x velocity, meters height, particle count, mean altitude in meters
    let x_anchor = SCREEN_WIDTH - (0.4 * SCREEN_WIDTH);

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
            x_anchor,
            idx as f32 * y_offset + y_offset,
            FONT_SIZE,
            GREEN,
        );
    }
}

pub async fn p_main() {
    // Setup
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut last_tick_time = get_time();
    let mut particles: Vec<Particle> = Vec::new();
    particles.push(Particle {
        x_pos: SCREEN_WIDTH / 2.0,
        y_pos: 5.0,
        x_velocity_m_s: 5.0,
        y_velocity_m_s: 0.0,
        lifetime_ms: 9999,
    });

    // Main loop
    loop {
        let now = get_time();
        let time_elapsed = now - last_tick_time;
        clear_background(BLACK);

        for p in particles.iter_mut() {
            p.y_velocity_m_s += calculate_gravity(time_elapsed);
            // println!("{}", p.y_velocity_m_s);

            p.x_velocity_m_s -= calculate_friction(
                &p,
                FRICTION_STATIC_COEFFICIENT,
                FRICTION_DYNAMIC_COEFFICIENT,
            );
            p.x_velocity_m_s = if p.x_velocity_m_s > 0.0 {
                p.x_velocity_m_s
            } else {
                0.0
            };

            apply_velocity_to_particle_position(p, time_elapsed);
            clamp_particle_position_to_screen(p, time_elapsed);
        }
        draw_particles(&particles);

        draw_stats(&particles, last_tick_time);

        last_tick_time = now;

        next_frame().await
    }
}


pub struct Object {
    pub position: XY,
    pub velocity: XY,
    pub force: f32,
}

pub struct Particle {
    pub position: XY,
    pub velocity: XY,
    pub force: f32,
}

// pub struct Particle {
//     // Particle position in pixels
//     pub x_pos: f32,
//     pub y_pos: f32,
//
//     // Signed particle velocity in meters per second
//     pub x_velocity_m_s: f32,
//     pub y_velocity_m_s: f32,
// }

pub struct XY {
    pub x: f32,
    pub y: f32,
}


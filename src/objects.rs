use std::ops;
use std::fmt;

pub struct Particle {
    pub position: XYZ,
    // signed velocity in m/s^2 in the X and Y axes
    pub velocity: XYZ,
    // signed force in Newtons in the X and Y axes
    pub force: XYZ,
    // mass in kilograms
    pub mass: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Display for XYZ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XY(X={},Y={},Z={})", self.x, self.y, self.z)
    }
}

// todo: reduce duplication
impl ops::Add<XYZ> for &XYZ {
    type Output = XYZ;

    fn add(self, rhs: XYZ) -> XYZ {
        return XYZ {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}
// impl ops::Add<XYZ> for &XYZ {
//     type Output = XYZ;
//
//     fn add(self, rhs: f32) -> XYZ {
//         return XYZ {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//         };
//     }
// }

impl ops::AddAssign<XYZ> for XYZ {
    fn add_assign(&mut self, rhs: XYZ) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<XYZ> for &XYZ {
    type Output = XYZ;

    fn sub(self, rhs: XYZ) -> XYZ {
        return XYZ {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::SubAssign<XYZ> for XYZ {
    fn sub_assign(&mut self, rhs: XYZ) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Div<XYZ> for &XYZ {
    type Output = XYZ;

    fn div(self, rhs: XYZ) -> XYZ {
        return XYZ {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        };
    }
}

impl ops::Div<f32> for &XYZ {
    type Output = XYZ;

    fn div(self, rhs: f32) -> XYZ {
        return XYZ {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

impl ops::DivAssign<XYZ> for XYZ {
    fn div_assign(&mut self, rhs: XYZ) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl ops::DivAssign<f32> for XYZ {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Mul<XYZ> for &XYZ {
    type Output = XYZ;

    fn mul(self, rhs: XYZ) -> XYZ {
        return XYZ {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}

impl ops::Mul<f32> for XYZ {
    type Output = XYZ;

    fn mul(self, rhs: f32) -> XYZ {
        return XYZ {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl ops::Mul<f32> for &XYZ {
    type Output = XYZ;

    fn mul(self, rhs: f32) -> XYZ {
        return XYZ {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

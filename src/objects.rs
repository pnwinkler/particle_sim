use std::ops;
use std::fmt;

pub struct Particle {
    pub position: XY,
    // signed velocity in m/s^2 in the X and Y axes
    pub velocity: XY,
    // signed force in Newtons in the X and Y axes
    pub force: XY,
    // mass in kilograms
    pub mass: f32,
}

pub struct XY {
    pub x: f32,
    pub y: f32,
}

impl fmt::Display for XY {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XY(X={},Y={})", self.x, self.y)
    }
}

impl ops::Add<XY> for &XY {
    type Output = XY;

    fn add(self, rhs: XY) -> XY {
        return XY {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl ops::AddAssign<XY> for XY {
    fn add_assign(&mut self, rhs: XY) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Div<XY> for &XY {
    type Output = XY;

    fn div(self, rhs: XY) -> XY {
        return XY {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        };
    }
}

impl ops::Div<f32> for &XY {
    type Output = XY;

    fn div(self, rhs: f32) -> XY {
        return XY {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl ops::DivAssign<XY> for XY {
    fn div_assign(&mut self, rhs: XY) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl ops::DivAssign<f32> for XY {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Mul<XY> for &XY {
    type Output = XY;

    fn mul(self, rhs: XY) -> XY {
        return XY {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        };
    }
}

impl ops::Mul<f32> for XY {
    type Output = XY;

    fn mul(self, rhs: f32) -> XY {
        return XY {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl ops::Mul<f32> for &XY {
    type Output = XY;

    fn mul(self, rhs: f32) -> XY {
        return XY {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

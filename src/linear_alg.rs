use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn length(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn rotate_y(self, theta: f64) -> Self {
        Self {
            x: self.x * theta.cos() - self.z * theta.sin(),
            y: self.y,
            z: self.x * theta.sin() + self.z * theta.cos(),
        }
    }

    pub fn rotate_z(self, theta: f64) -> Self {
        Self {
            x: self.x * theta.cos() - self.y * theta.sin(),
            y: self.x * theta.sin() + self.y * theta.cos(),
            z: self.z,
        }
    }

    pub fn swap_x_y(self) -> Self {
        Self {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

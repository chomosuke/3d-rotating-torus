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

pub fn rotate_y(p: Vector, theta: f64) -> Vector {
    Vector {
        x: p.x * theta.cos() - p.z * theta.sin(),
        y: p.y,
        z: p.x * theta.sin() + p.z * theta.cos(),
    }
}

pub fn rotate_z(p: Vector, theta: f64) -> Vector {
    Vector {
        x: p.x * theta.cos() - p.y * theta.sin(),
        y: p.x * theta.sin() + p.y * theta.cos(),
        z: p.z,
    }
}

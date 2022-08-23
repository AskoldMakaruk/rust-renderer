use std::ops::{Add, Mul, Neg, Sub};

use crate::geometry::normal::Normal;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Vector {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) z: f64,
}

impl Vector {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub(crate) fn dot(&self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub(crate) fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub(crate) fn normalize(&self) -> Normal {
        let length = self.length();
        if length == 0.0 {
            return Normal::new(0.0, 0.0, 0.0);
        }
        Normal::new(self.x / length, self.y / length, self.z / length)
    }
}

impl Mul for Vector {
    type Output = f64;

    fn mul(self, other: Vector) -> f64 {
        self.dot(other)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
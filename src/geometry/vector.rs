use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::geometry::normal::Normal;
use crate::geometry::point::Point;

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

    pub(crate) fn cross(&self, other: Vector) -> Vector {
         Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl From<Point> for Vector {
    fn from(point: Point) -> Vector {
        Vector::new(point.x, point.y, point.z)
    }
}

impl From<Normal> for Vector {
    fn from(normal: Normal) -> Vector {
        Vector::new(normal.x, normal.y, normal.z)
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

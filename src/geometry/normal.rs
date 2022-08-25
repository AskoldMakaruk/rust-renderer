use std::ops::Mul;

use crate::geometry::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Normal {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Normal {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Normal {
        debug_assert!(
            x <= 1. && y <= 1. && z <= 1.,
            "Normal must be between 0 and 1, but have {:?}",
            (x, y, z)
        );
        Normal { x, y, z }
    }

    pub(crate) fn dot(&self, other: Normal) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Normal {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
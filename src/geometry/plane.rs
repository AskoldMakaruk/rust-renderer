use crate::geometry::normal::Normal;
use crate::geometry::point::Point;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;

use super::{Intersect, NormalAtPoint};

pub(crate) struct Plane {
    pub(crate) normal: Normal,
    pub(crate) center: Point,
}

impl Plane {
    pub(crate) fn new(normal: Normal, center: Point) -> Plane {
        Plane { normal, center }
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // (p - c)*n = 0 plane equation
        // p = (o + t*d) ray equation
        // k = (o - c)
        // (t*d + k)*n = 0;
        // t*d*n = -k*n;
        // t = -k*n / d*n;
        // if d * n == 0 then the ray is parallel to the plane, so no intersection
        // if t < 0 then the ray is pointing away from the plane, so no intersection

        let normal:Vector = self.normal.into();
        let dn =Vector::from(ray.direction).dot(normal);

        if dn.abs() <= f64::EPSILON {
            return None;
        }

        let k = Vector::from(self.center) - Vector::from(ray.origin);
        let t = k.dot(normal) / dn;
        if t >= 0. {
            Some(t)
        } else {
            None
        }
    }
}

impl NormalAtPoint for Plane {
   fn normal_at_point(&self, point: &Point) -> Normal {
      self.normal 
   } 
}
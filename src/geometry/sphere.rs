use crate::geometry::point::Point;
use crate::geometry::ray::Ray;

use super::Intersect;

#[derive(Debug)]
pub(crate) struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub(crate) fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let k = ray.origin.to_vector() - self.center.to_vector();
        let a = ray.direction.dot(ray.direction);
        let b = 2. * k.dot(ray.direction.to_vector());
        let c = k.dot(k) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return None;
        }
        let square_descriminant = discriminant.sqrt();
        let t1 = (-b - square_descriminant) / (2. * a);
        let t2 = (-b + square_descriminant) / (2. * a);

        if t1 >= 0. && t2 >= 0. {
            Some(t1.min(t2))
        } else if t1 >= 0. {
            Some(t1)
        } else if t2 >= 0. {
            Some(t2)
        } else {
            None
        }
    }
}
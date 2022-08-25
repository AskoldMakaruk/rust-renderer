use crate::geometry::normal::Normal;
use crate::geometry::point::Point;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;

use super::plane::Plane;
use super::Intersect;
use super::Intersection;

pub(crate) struct Disk {
    center: Point,
    radius: f64,
    normal: Normal,
}

impl Disk {
    pub(crate) fn new(center: Point, radius: f64, normal: Normal) -> Disk {
        Disk {
            center,
            radius,
            normal,
        }
    }
}

impl Intersect for Disk {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let plane = Plane::new(self.normal, self.center);
        match dbg!(plane.intersect(ray)) {
            Intersection::Intersect(t) if t > 0. => {
                let point = ray.at(t);
                let distance = (Vector::from(point) - Vector::from(self.center)).length();
                if distance < self.radius {
                    Intersection::Intersect(t)
                } else {
                    Intersection::DoesNotIntersect
                }
            }
            _ => Intersection::DoesNotIntersect,
        }
    }
}

use crate::geometry::normal::Normal;
use crate::geometry::point::Point;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;

use super::{Intersect, Intersection, NormalAtPoint, Transform, Transformation};

#[derive(Debug, Clone, Copy)]
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
    fn intersect(&self, ray: &Ray) -> Intersection {
        let k = Vector::from(ray.origin) - Vector::from(self.center);
        let a = ray.direction.dot(ray.direction);
        let b = 2. * k.dot(Vector::from(ray.direction));
        let c = k.dot(k) - self.radius * self.radius;
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return Intersection::DoesNotIntersect;
        }
        let square_descriminant = discriminant.sqrt();
        let t1 = (-b - square_descriminant) / (2. * a);
        let t2 = (-b + square_descriminant) / (2. * a);

        if t1 > 0. && t2 > 0. {
            Intersection::Intersect(t1.min(t2))
        } else if t1 > 0. {
            Intersection::Intersect(t1)
        } else if t2 > 0. {
            Intersection::Intersect(t2)
        } else {
            Intersection::DoesNotIntersect
        }
    }
}

impl NormalAtPoint for Sphere {
    fn normal_at_point(&self, point: &Point, _: Intersection) -> Normal {
        (Vector::from(*point) - Vector::from(self.center)).normalize()
    }
}

impl Transform for Sphere {
    fn transform(&mut self, transformation: Transformation) {
        match transformation {
            Transformation::Translation(_) => {
                self.center = transformation.transformation_to_matrix() * self.center;
            }
            Transformation::Scale(scale) => {
                self.center = transformation.transformation_to_matrix() * self.center;
                // TODO: if scale is not uniform, this will not work, because it suppose to become a ellipsoid
                self.radius *= scale.x.max(scale.y).max(scale.z);
            }
            Transformation::Rotation(_, _) => {}
        }
    }
}

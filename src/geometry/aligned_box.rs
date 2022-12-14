use crate::geometry::normal::Normal;
use crate::geometry::point::Point;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;

use super::plane::Plane;
use super::Intersect;
use super::Intersection;
use super::NormalAtPoint;

#[derive(Debug, Clone, Copy)]
pub(crate) struct AlignedBox {
    min: Point,
    max: Point,
}

impl AlignedBox {
    pub(crate) fn new(min: Point, max: Point) -> AlignedBox {
        AlignedBox { min, max }
    }

    pub(crate) fn from_dimensions(
        center: Point,
        width: f64,
        height: f64,
        length: f64,
    ) -> AlignedBox {
        let size_vector = Vector::new(width, height, length);
        let center: Vector = center.into();
        AlignedBox::new((center - size_vector).into(), (center + size_vector).into())
    }
}

impl Intersect for AlignedBox {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let mut tmin = (self.min.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.max.x - ray.origin.x) / ray.direction.x;

        let swap_tmin_tmax = |tmin: &mut f64, tmax: &mut f64| {
            if tmin > tmax {
                std::mem::swap(tmin, tmax);
            }
        };
        swap_tmin_tmax(&mut tmin, &mut tmax);

        let mut tymin = (self.min.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.max.y - ray.origin.y) / ray.direction.y;

        swap_tmin_tmax(&mut tymin, &mut tymax);
        if tmin > tymax || tymin > tmax {
            return Intersection::DoesNotIntersect;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.max.z - ray.origin.z) / ray.direction.z;

        swap_tmin_tmax(&mut tzmin, &mut tzmax);
        if tmin > tzmax || tzmin > tmax {
            return Intersection::DoesNotIntersect;
        }
        if tzmin > tmin {
            tmin = tzmin;
        }

        if tmin.is_infinite() || tmin.is_nan() {
            Intersection::DoesNotIntersect
        } else {
            Intersection::Intersect(tmin)
        }
    }
}

impl NormalAtPoint for AlignedBox {
    fn normal_at_point(&self, point: &Point, _: Intersection) -> Normal {
        if (point.x - self.min.x).abs() < 0.001 {
            Normal::new(-1., 0., 0.)
        } else if (point.x - self.max.x).abs() < 0.001 {
            Normal::new(01., 0., 0.)
        } else if (point.y - self.min.y).abs() < 0.001 {
            Normal::new(0., -1., 0.)
        } else if (point.y - self.max.y).abs() < 0.001 {
            Normal::new(0., 1., 0.)
        } else if (point.z - self.min.z).abs() < 0.001 {
            Normal::new(0., 0., -1.)
        } else if (point.z - self.max.z).abs() < 0.001 {
            Normal::new(0., 0., 1.)
        } else {
            panic!("point is not in the box {:?}", point);
        }
    }
}

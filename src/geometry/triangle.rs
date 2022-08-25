use crate::geometry::point::Point;

use super::{normal::Normal, ray::Ray, vector::Vector, Intersect, NormalAtPoint};

pub(crate) struct Triangle {
    a: Vector,
    b: Vector,
    c: Vector,
}

impl Triangle {
    pub(crate) fn new(a: Point, b: Point, c: Point) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let normal = Vector::from(ray.direction).cross(ac);

        let d = ab.dot(normal);
        if d.abs() < f64::EPSILON {
            return None;
        }

        let inv_d = 1.0 / d;
        let ao = Vector::from(ray.origin) - self.a;
        let u_coordinate = ao.dot(normal) * inv_d;
        if u_coordinate < 0.0 || u_coordinate > 1.0 {
            return None;
        }

        let ao_cross_ab = ao.cross(ab);
        let v_coordinate = Vector::from(ray.direction).dot(ao_cross_ab) * inv_d;
        if v_coordinate < 0.0 || u_coordinate + v_coordinate > 1.0 {
            return None;
        }
        let t = ac.dot(ao_cross_ab) * inv_d;
        if t > f64::EPSILON {
            Some(t)
        } else {
            None
        }
    }
}

impl NormalAtPoint for Triangle {
    fn normal_at_point(&self, _: &Point) -> Normal {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        ab.cross(ac).normalize()
    }
}
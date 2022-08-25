use crate::geometry::normal::Normal;
use crate::geometry::point::Point;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector;
use crate::geometry::{Transform, Transformation};
use crate::renderer::viewframe::ViewFrame;

// Ray-tracing camera.
pub(crate) struct Camera {
    // Camera position.
    pub position: Point,
    // Camera view frame.
    pub view_frame: ViewFrame,
}

impl Camera {
    pub(crate) fn new(position: Point, view_frame: ViewFrame) -> Camera {
        Camera {
            position,
            view_frame,
        }
    }

    pub(super) fn ray_for_pixel(
        &self,
        x: usize,
        y: usize,
        image_width: usize,
        image_height: usize,
    ) -> Ray {
        let point = self
            .view_frame
            .point_on_pixel(x, y, image_width, image_height);
        let direction = (point - self.position).normalize();
        Ray::new(self.position, direction)
    }
}

impl Transform for Camera {
    fn transform(&mut self, transform: Transformation) {
        let matrix = transform.transformation_to_matrix();
        self.position = matrix * self.position;
    }
}

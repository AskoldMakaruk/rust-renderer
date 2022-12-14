use crate::geometry::point::Point;

pub(crate) struct ViewFrame {
    pub(crate) origin: Point,
    pub(crate) width: f64,
    pub(crate) height: f64,
}

impl ViewFrame {
    pub(crate) fn new(origin: Point, width: f64, height: f64) -> ViewFrame {
        ViewFrame {
            origin,
            width,
            height,
        }
    }

    pub(crate) fn point_on_pixel(
        &self,
        x: usize,
        y: usize,
        image_width: usize,
        image_height: usize,
    ) -> Point {
        let x_factor = self.width / (image_width as f64);
        let y_factor = self.height / (image_height as f64);

        let x_offset = x as f64 * x_factor;
        let y_offset = y as f64 * y_factor;
        Point::new(
            self.origin.x - self.width / 2.0 + x_offset,
            self.origin.y - self.height / 2.0 + y_offset,
            self.origin.z,
        )
    }
}
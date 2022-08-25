mod geometry;
mod io;
mod renderer;

use std::path::PathBuf;
use geometry::normal::Normal;
use geometry::point::Point;
use geometry::triangle::Triangle;
use renderer::camera::Camera;
use renderer::light::DirectedLight;
use renderer::scene::Scene;
use renderer::viewframe::ViewFrame;
use renderer::RayTracer;

fn main() {
     let object = Triangle::new(
        Point::new(-10., 0., 0.),
        Point::new(10., 0., 0.),
        Point::new(0., 20., 0.),
    );
 let mut scene = Scene::new();
    scene.add_object(Box::new(object));
    scene.add_light(DirectedLight::new(Normal::new(0.0, 0.0, 1.0)));
    // scene.add_light(DirectedLight::new(Normal::new(0.0, 0.7, 0.0)));
    // scene.add_light(DirectedLight::new(Normal::new(-0.4, 0.0, 0.0)));
    let viewframe = ViewFrame::new(Point::new(0.0, 5.0, 50.0), 40.0, 40.0);
    let camera = Camera::new(Point::new(0.0, 5.0, 80.0), viewframe);
    let ray_tracer = RayTracer::new(scene, camera, 500, 500);
    ray_tracer
        .render(io::ppm_image::PPMImage::new(PathBuf::from("./image.ppm")))
        .unwrap();}
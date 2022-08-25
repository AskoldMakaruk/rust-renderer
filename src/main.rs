mod geometry;
mod io;
mod renderer;

use std::path::PathBuf;
use geometry::point::Point;
use geometry::sphere::Sphere;
use renderer::camera::Camera;
use renderer::scene::Scene;
use renderer::viewframe::ViewFrame;
use renderer::RayTracer;

fn main() {
    println!("Hello, world!");
    let mut scene = Scene::new();
    scene.add_object(Box::new(Sphere::new(Point::new(0.0, 10.0, 50.0), 5.)));
    scene.add_object(Box::new(Sphere::new(Point::new(-10.0, 0.0, 50.0), 5.)));
    scene.add_object(Box::new(Sphere::new(Point::new(10.0, 0.0, 50.0), 5.)));
    scene.add_object(Box::new(Sphere::new(Point::new(-5.0, -10.0, 50.0), 5.)));
    scene.add_object(Box::new(Sphere::new(Point::new(5.0, -10.0, 50.0), 5.)));

    let viewframe = ViewFrame::new(Point::new(0.0, 0.0, 50.0), 50.0, 50.0);

    let camera = Camera::new(Point::new(0.0, 0.0, 0.0), viewframe);

    let ray_tracer = RayTracer::new(scene, camera, 500, 500);
    ray_tracer
        .render(io::ppm_image::PPMImage::new(PathBuf::from("./image.ppm")))
        .unwrap();
}
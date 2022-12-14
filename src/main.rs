mod geometry;
mod io;
mod renderer;

use geometry::{Axis, Transformation};
use geometry::normal::Normal;
use geometry::point::Point;
use geometry::sphere::Sphere;
use geometry::triangle::Triangle;
use renderer::camera::Camera;
use renderer::light::Light;
use renderer::scene::Scene;
use renderer::viewframe::ViewFrame;
use renderer::RayTracer;
use std::ffi::OsStr;
use std::path::PathBuf;

fn main() {
    let (source, output) = parse_args();
    let mut scene = Scene::from_obj_file(source).unwrap();
    scene.add_light(Light::new(Point::new(50.0, 0.0, 150.0)));
    for object in scene.objects_mut().iter_mut() {
        object.transform(Transformation::Rotation(Axis::Y, 90.0));
        object.transform(Transformation::Rotation(Axis::Z, 90.0));
    }
    let viewframe = ViewFrame::new(Point::new(20.0, 25.0, 80.0), 75.0, 42.0);
    let camera = Camera::new(Point::new(20.0, 25.0, 130.0), viewframe);
    let ray_tracer = RayTracer::new(scene, camera, 720, 576);
    ray_tracer
        .render(io::ppm_image::PPMImage::new(output))
        .unwrap();
}
fn parse_args() -> (PathBuf, PathBuf) {
    const HELP_MSG: &str = "./graphics --source=path_to_object.obj --output=path_to_result.ppm 
                            The ratracer takes two arguments: the input file and the output file.
                            The input file is a object file in the Wavefront OBJ format.
                            The output file is a image fiile in the PPM file format.";

    let mut source: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    for arg in std::env::args() {
        if arg == "--help" {
            println!("{}", HELP_MSG);
            std::process::exit(0);
        } else if arg.starts_with("--source=") {
            if let Some(path) = arg.split('=').nth(1) {
                let path = PathBuf::from(path);
                if Some(OsStr::new("obj")) == path.extension() {
                    if path.exists() {
                        source = Some(path);
                    } else {
                        println!("The source file does not exist.\n\n{}", HELP_MSG);
                        std::process::exit(1);
                    }
                } else {
                    println!("Incorrect input file format\n\n{}", HELP_MSG);
                    std::process::exit(0);
                }
            }
        } else if arg.starts_with("--output=") {
            if let Some(path) = arg.split('=').nth(1) {
                let path = PathBuf::from(path);
                if Some(OsStr::new("ppm")) == path.extension() {
                    output = Some(path);
                } else {
                    println!("Incorrect output file format\n\n{}", HELP_MSG);
                    std::process::exit(0);
                }
            }
        }
    }

    if source.is_none() || output.is_none() {
        println!("All required arguments is not provided.\n\n{}", HELP_MSG);
        std::process::exit(0);
    }
    (source.unwrap(), output.unwrap())
}

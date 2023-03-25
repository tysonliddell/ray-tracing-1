use std::{io, rc::Rc};

use log::{error, info};

use ray_tracing_1::{
    camera::{Camera, Config as CameraConfig},
    geometry::{sphere::Sphere, vec3::Vec3},
    material::{Dielectric, Lambertian, Material, Metal},
    tracer::{self, World},
    utils::correct_gamma,
};

type RcMaterial = Rc<dyn Material>;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    env_logger::init();
    if let Err(e) = generate_ppm() {
        error!("Error generating image: {}", e);
        std::process::exit(1);
    }
}

fn generate_ppm() -> io::Result<()> {
    let image_config = tracer::ImageConfig {
        width: 400,
        height: 225,
        samples_per_pixel: 100,
        ray_bounce_limit: 50,
    };

    assert_eq!(
        image_config.width as f64 / image_config.height as f64,
        ASPECT_RATIO,
        "Dimensions don't match aspect ratio!"
    );

    let material_ground = Rc::new(Lambertian::new(0.8, 0.8, 0.0));
    let material_center = Rc::new(Lambertian::new(0.1, 0.2, 0.5));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new((0.8, 0.6, 0.2), 0.0));

    let world: World = vec![
        Rc::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Rc::clone(&material_ground) as RcMaterial,
        )),
        Rc::new(Sphere::new(
            (0, 0, -1).into(),
            0.5,
            Rc::clone(&material_center) as RcMaterial,
        )),
        Rc::new(Sphere::new(
            (-1, 0, -1).into(),
            0.5,
            Rc::clone(&material_left) as RcMaterial,
        )),
        Rc::new(Sphere::new(
            (-1, 0, -1).into(),
            -0.45,
            Rc::clone(&material_left) as RcMaterial,
        )),
        Rc::new(Sphere::new(
            (1, 0, -1).into(),
            0.5,
            Rc::clone(&material_right) as RcMaterial,
        )),
    ];

    let camera_config = CameraConfig {
        look_from: (-2, 2, 1).into(),
        look_at: (0, 0, -1).into(),
        vup: (0, 1, 0).into(),
        vfov_degrees: 90.0,
        aspect_ratio: ASPECT_RATIO,
    };
    let camera = Camera::new(camera_config);

    println!("P3");
    println!("{} {}", image_config.width, image_config.height);
    println!("255");

    info!("Rendering world...");
    let mut scanlines = tracer::render(image_config, camera, world);

    info!("Correcting gamma.");
    correct_gamma(&mut scanlines);

    info!("Writing scanlines.");
    for pixel_color in scanlines.iter().flatten() {
        println!(
            "{} {} {}",
            pixel_color.red, pixel_color.green, pixel_color.blue
        );
    }

    info!("Done!");
    Ok(())
}

use std::{f64::consts::PI, io, rc::Rc};

use log::{error, info};

use ray_tracing_1::{
    camera::Camera,
    geometry::{sphere::Sphere, vec3::Vec3},
    material::{Lambertian, Material},
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

    let material_left = Rc::new(Lambertian::new(0.0, 0.0, 1.0));
    let material_right = Rc::new(Lambertian::new(1.0, 0.0, 0.0));

    let r = (PI / 4.0).cos();
    let world: World = vec![
        Rc::new(Sphere::new(
            Vec3::new(-r, 0, -1),
            r,
            Rc::clone(&material_left) as RcMaterial,
        )),
        Rc::new(Sphere::new(
            Vec3::new(r, 0, -1),
            r,
            Rc::clone(&material_right) as RcMaterial,
        )),
    ];

    let camera = Camera::new(90.0, ASPECT_RATIO);

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

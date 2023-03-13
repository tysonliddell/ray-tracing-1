use std::{io, rc::Rc};

use log::{error, info};

use ray_tracing_1::{
    camera::Camera,
    geometry::{sphere::Sphere, vec3::Vec3},
    tracer::{self, World},
    utils::correct_gamma,
};

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

    let world: World = vec![
        Rc::new(Sphere::new(Vec3::new(0, 0, -1.0), 0.5)),
        Rc::new(Sphere::new(Vec3::new(0, -100.5, -1), 100)),
    ];
    let camera = Camera::new(ASPECT_RATIO);

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

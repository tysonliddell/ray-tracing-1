use std::{io, rc::Rc};

use log::{error, info};

use ray_tracing_1::{
    camera::{Camera, Config as CameraConfig},
    geometry::{sphere::Sphere, vec3::Vec3},
    material::{Dielectric, Lambertian, Material, Metal},
    tracer::{self, World},
    utils::{correct_gamma, rand::RTRng},
};

type RcMaterial = Rc<dyn Material>;

const ASPECT_RATIO: f64 = 3.0 / 2.0;

fn main() {
    env_logger::init();
    if let Err(e) = generate_ppm() {
        error!("Error generating image: {}", e);
        std::process::exit(1);
    }
}

fn generate_ppm() -> io::Result<()> {
    let image_config = tracer::ImageConfig {
        width: 1200,
        height: 800,
        samples_per_pixel: 10,
        ray_bounce_limit: 50,
    };

    assert_eq!(
        image_config.width as f64 / image_config.height as f64,
        ASPECT_RATIO,
        "Dimensions don't match aspect ratio!"
    );

    let world = random_scene(&RTRng::new());

    let look_from = Vec3::from((13, 2, 3));
    let look_at = Vec3::from((0, 0, 0));
    let vup = Vec3::from((0, 1, 0));
    let camera_config = CameraConfig {
        look_from,
        look_at,
        vup,
        vfov_degrees: 20.0,
        aspect_ratio: ASPECT_RATIO,
        aperture_diameter: 0.1,
        focus_dist: 10.0,
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

pub fn random_scene(rng: &RTRng) -> World {
    let mut world: World = vec![];

    let material_ground = Rc::new(Lambertian::new(0.5, 0.5, 0.5));
    let ground_sphere = Rc::new(Sphere::new(
        (0, -1000, 0).into(),
        1000,
        Rc::clone(&material_ground) as RcMaterial,
    ));
    world.push(ground_sphere);

    for i in -11..11 {
        for j in -11..11 {
            let center = Vec3::new(
                i as f64 + 0.9 * rng.random_f64(),
                0.2,
                j as f64 + 0.9 * rng.random_f64(),
            );
            let choose_mat = rng.random_f64();

            if (center - Vec3::from((4, 0.2, 0))).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = rng.random_vec3() * rng.random_vec3();
                    let material = Rc::new(Lambertian::new(albedo.x(), albedo.y(), albedo.z()));
                    world.push(Rc::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = rng.random_vec3_range(0.5..1.0);
                    let fuzz = rng.random_f64_range(0.0..0.5);
                    let material = Rc::new(Metal::new((albedo.x(), albedo.y(), albedo.z()), fuzz));
                    world.push(Rc::new(Sphere::new(center, 0.2, material)));
                } else {
                    // glass
                    let material = Rc::new(Dielectric::new(1.5));
                    world.push(Rc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.push(Rc::new(Sphere::new((0, 1, 0).into(), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(0.4, 0.2, 0.1));
    world.push(Rc::new(Sphere::new((-4, 1, 0).into(), 1.0, material2)));

    let material3 = Rc::new(Metal::new((0.7, 0.6, 0.5), 0.0));
    world.push(Rc::new(Sphere::new((4, 1, 0).into(), 1.0, material3)));

    world
}

use std::rc::Rc;

use log::debug;

use crate::{
    camera::Camera,
    color::Color,
    geometry::{hittable::Hittable, ray::Ray, vec3::Vec3},
    utils::rand::RTRng,
};

/// Configuration to be used by the ray tracer.
pub struct ImageConfig {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub ray_bounce_limit: u32,
}

/// Trait object that can be rendered.
pub type RcHittable = Rc<dyn Hittable>;

/// A world to be rendered.
pub type World = Vec<RcHittable>;

/// The color of each pixel in a scanline ordered left to right.
pub type Scanline = Vec<Color>;

/// The scanlines of a rendered image ordered top to bottom.
pub type ImagePixels = Vec<Scanline>;

struct PixelPos {
    row: u32,
    col: u32,
}

/// Render a ray-traced scene.
pub fn render(config: ImageConfig, camera: Camera, world: World) -> ImagePixels {
    // TODO: Make this faster with parallelism.
    debug!("Generating pixels");
    let rng = RTRng::new();

    let get_row = |row| -> Vec<Color> {
        (0..config.width)
            .map(|col| {
                get_multi_sampled_pixel_color(&camera, &world, PixelPos { row, col }, &config, &rng)
            })
            .collect()
    };

    let pixels = (0..config.height)
        .rev()
        .inspect(|row| debug!("Scanlines remaining: {row} "))
        .map(get_row)
        .collect();

    debug!("Done generating pixels");
    pixels
}

fn get_multi_sampled_pixel_color(
    camera: &Camera,
    world: &World,
    pixel_pos: PixelPos,
    config: &ImageConfig,
    rng: &RTRng,
) -> Color {
    let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
    for _ in 0..config.samples_per_pixel {
        let u = (pixel_pos.col as f64 + rng.random_f64()) / (config.width - 1) as f64;
        let v = (pixel_pos.row as f64 + rng.random_f64()) / (config.height - 1) as f64;
        let ray = camera.get_ray(u, v);

        let color = ray_color(&ray, world, rng, config.ray_bounce_limit);
        r += color.red as u32;
        g += color.green as u32;
        b += color.blue as u32;
    }

    Color {
        red: (r / config.samples_per_pixel) as u8,
        green: (g / config.samples_per_pixel) as u8,
        blue: (b / config.samples_per_pixel) as u8,
    }
}

/// Get the color of hittable closest to the ray.
fn ray_color(ray: &Ray, world: &[RcHittable], rng: &RTRng, bounces_remaining: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if bounces_remaining == 0 {
        return (0, 0, 0).into();
    }

    match world.hit(ray, 0.001, f64::INFINITY) {
        Some(hit) => {
            let target_dir = hit.normal + rng.random_unit_vector();
            let color = ray_color(
                &Ray::new(hit.point, target_dir),
                world,
                rng,
                bounces_remaining - 1,
            );
            color.scaled(0.5)
        }
        None => {
            let unit_dir = ray.direction().normalized();
            let t = 0.5 * (unit_dir.y() + 1.0);

            let c1 = Vec3::new(1, 1, 1);
            let c2 = Vec3::new(0.5, 0.7, 1.0);
            ((1.0 - t) * c1 + t * c2).try_into().unwrap()
        }
    }
}

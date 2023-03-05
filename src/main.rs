use ray_tracing_1::{
    color::{Color, RED},
    geometry::{ray::Ray, vec3::Vec3},
};
use std::io::{self, Write};

fn main() {
    if let Err(e) = generate_ppm() {
        eprintln!("Error generating image: {}", e);
        std::process::exit(1);
    }
}

fn generate_ppm() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        io::stderr().flush()?;
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let pixel_color = ray_color(&ray);
            write_color(&mut io::stdout(), pixel_color)?;
        }
    }

    eprintln!("\nDone");
    Ok(())
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let co = ray.origin() - center;
    let b = 2.0 * ray.direction().dot(co);
    let a = ray.direction().dot(ray.direction());
    let c = co.dot(co) - radius * radius;
    let descrim = b * b - 4.0 * a * c;
    descrim > 0.0
}

fn ray_color(ray: &Ray) -> Color {
    let sphere_c = Vec3::new(0.0, 0.0, -1.0);
    if hit_sphere(sphere_c, 0.5, ray) {
        return RED;
    }

    let dir = ray.direction().normalized();
    let t = 0.5 * (dir.y() + 1.0);

    let c1 = Vec3::from(Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    });
    let c2 = Vec3::from(Color {
        red: 0.5,
        green: 0.7,
        blue: 1.0,
    });

    ((1.0 - t) * c1 + t * c2).into()
}

fn write_color<T: io::Write>(writer: &mut T, color: Color) -> io::Result<()> {
    writeln!(
        writer,
        "{} {} {}",
        (255.999 * color.red) as i32,
        (255.999 * color.green) as i32,
        (255.999 * color.blue) as i32,
    )
}

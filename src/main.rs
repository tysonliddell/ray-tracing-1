use ray_tracing_1::color::Color;
use std::io::{self, Write};

fn main() {
    if let Err(e) = generate_ppm() {
        eprintln!("Error generating image: {}", e);
        std::process::exit(1);
    }
}

fn generate_ppm() -> io::Result<()> {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        io::stderr().flush()?;

        for i in 0..image_width {
            let pixel_color = Color {
                red: i as f64 / (image_width - 1) as f64,
                green: j as f64 / (image_height - 1) as f64,
                blue: 0.25,
            };
            write_color(&mut io::stdout(), pixel_color)?;
        }
    }

    eprintln!("\nDone");
    Ok(())
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

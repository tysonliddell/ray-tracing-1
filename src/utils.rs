use crate::tracer::ImagePixels;

pub(crate) mod rand;

/// Apply gamma 2 correction to an image.
pub fn correct_gamma(pixels: &mut ImagePixels) {
    // on second thoughts, it might have been a better idea to leave all color values
    // as floats internally, and only convert to the [0-255] range at the end.
    // ¯\_(ツ)_/¯
    for row in pixels {
        for color in row {
            color.red = ((((color.red as f64) / 255.0).sqrt()) * 255.0) as u8;
            color.green = ((((color.green as f64) / 255.0).sqrt()) * 255.0) as u8;
            color.blue = ((((color.blue as f64) / 255.0).sqrt()) * 255.0) as u8;
        }
    }
}

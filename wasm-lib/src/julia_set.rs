use base64::{engine::general_purpose, Engine};
use image::{self, DynamicImage, ImageBuffer, ImageFormat, ImageResult, Rgb};
use num_complex::Complex;
use std::io::Cursor;

pub fn julia_base64_inner(width: u32, height: u32, real: f32, imaginary: f32) -> String {
    let image = generate_julia_set(width, height, real, imaginary);

    base64_png(DynamicImage::ImageRgb8(image)).expect("Unable to convert image to base64 string")
}

fn base64_png(img: DynamicImage) -> ImageResult<String> {
    let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png)?;
    let base64str = general_purpose::STANDARD.encode(buffer.get_ref());
    Ok(base64str)
}

fn generate_julia_set(
    width: u32,
    height: u32,
    real: f32,
    imaginary: f32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    let red_scale = 255.0 / width as f32;
    let blue_scale = 255.0 / height as f32;

    let c = Complex::new(real, imaginary);

    let mut image_buffer = ImageBuffer::new(width, height);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let r = (red_scale * x as f32) as u8;
        let b = (blue_scale * y as f32) as u8;

        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;
        let mut z = Complex::new(cx, cy);

        let mut i = 0;
        while i < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            i += 1;
        }

        *pixel = Rgb([r, i as u8, b]);
    }

    image_buffer
}

mod tests {
    #[test]
    fn test_julia() {
        let base64julia = crate::julia_set::julia_base64_inner(600, 600, -0.5, 0.6);
        assert!(!base64julia.is_empty());
    }
}

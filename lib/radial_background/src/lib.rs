//! Просто библиотека которая генерирует файл с градиентным фоном
use image::{ImageBuffer, ImageResult, Rgba};

pub fn create_radial_gradient(
    radius: f32,
    width: u32,
    height: u32,
    color1: [u8; 4],
    color2: [u8; 4],
    filename: &str,
) -> ImageResult<()> {
    let mut imgbuf = ImageBuffer::new(width, height);

    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius = width.min(height) as f32 * radius;
    let radius_squared = radius * radius;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let dx = x as f32 - center_x;
        let dy = y as f32 - center_y;
        let distance_squared = dx * dx + dy * dy;
        let t = (distance_squared / radius_squared).tanh();

        let r = (color1[0] as f32 * (1.0 - t) + color2[0] as f32 * t) as u8;
        let g = (color1[1] as f32 * (1.0 - t) + color2[1] as f32 * t) as u8;
        let b = (color1[2] as f32 * (1.0 - t) + color2[2] as f32 * t) as u8;
        let a = (color1[3] as f32 * (1.0 - t) + color2[3] as f32 * t) as u8;

        *pixel = Rgba([r, g, b, a]);
    }

    imgbuf.save(filename)
}

mod tests {

    #[cfg(test)]
    use super::*;
    #[test]
    fn it_works() {
        let color1 = [255u8, 255u8, 255u8, 255u8];
        let color2 = [145u8, 28u8, 139u8, 255u8];
        create_radial_gradient(0.5, 800, 1300, color1, color2, "gradient.png").unwrap();
    }
}

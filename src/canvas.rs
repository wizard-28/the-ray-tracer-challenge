use std::{ops, path::Path};

use image::{DynamicImage, ImageBuffer, Rgb32FImage, RgbImage, Rgba32FImage};

use crate::primitive::Color;

#[derive(Debug)]
pub struct Canvas {
    buffer: Rgb32FImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: Rgb32FImage::new(width, height),
        }
    }

    pub fn width(&self) -> u32 {
        self.buffer.width()
    }

    pub fn height(&self) -> u32 {
        self.buffer.height()
    }

    pub fn export<P: AsRef<Path>>(self, path: P) -> image::ImageResult<()> {
        DynamicImage::from(self.buffer).to_rgba8().save(path)
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.buffer.put_pixel(x, y, color.into());
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        self.buffer.get_pixel(x, y).into()
    }
}

impl From<Color> for image::Rgb<f32> {
    fn from(color: Color) -> Self {
        Self([color.red, color.green, color.blue])
    }
}

impl From<&image::Rgb<f32>> for Color {
    fn from(rgb: &image::Rgb<f32>) -> Self {
        Self::new(rgb.0[0], rgb.0[1], rgb.0[2])
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use image::ImageResult;

    use super::*;

    #[test]
    fn creating() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);

        for (_x, _y, color) in c.buffer.enumerate_pixels() {
            assert_eq!(*color, Color::black().into());
        }
    }

    #[test]
    fn writing() {
        let mut c = Canvas::new(10, 20);
        let red = Color::red();

        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn ppm() -> ImageResult<()> {
        let mut c = Canvas::new(5, 3);

        c.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));

        let test_path = Path::new("test_image.png");
        c.export(test_path)?;
        fs::remove_file(test_path)?;
        Ok(())
    }
}

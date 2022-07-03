use image::{RgbImage, DynamicImage};
use std::ops::{
    Add,
    Mul,
    Div,
    AddAssign,
};

#[derive(Debug, Clone)]
pub struct Image {
    pixels: Box<[Color]>,
    width: u32,
    hight: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Image {
    pub fn new(width: u32, hight: u32) -> Image {
        Image {
            pixels: vec![Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
            }; (width * hight) as usize].into_boxed_slice(),
            width,
            hight,
        }
    }
    pub fn to_image(self) -> DynamicImage{
        let mut image: RgbImage = RgbImage::new(self.width, self.hight);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            *pixel = image::Rgb([
                (self.pixels[x as usize + y as usize * self.width as usize].red * 255.0) as u8,
                (self.pixels[x as usize + y as usize * self.width as usize].green * 255.0) as u8,
                (self.pixels[x as usize + y as usize * self.width as usize].blue * 255.0) as u8,
            ]);
        }
        image::DynamicImage::ImageRgb8(image)
    }
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels[x as usize + y as usize * self.width as usize] = color;
    }
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {
            red,
            green,
            blue,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, rhs: f64) -> Color {
        Color {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Color {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}


#[test]
fn test_image_to_dynamic_image() {
    let image = Image {
        pixels: Box::new([Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }]),
        width: 1,
        hight: 1,
    };
    let dynamic_image = image.to_image();
    assert_eq!(dynamic_image.width(), 1);
    assert_eq!(dynamic_image.height(), 1);
}
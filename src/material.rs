pub mod lambertian;

use crate::{
    ray::Ray,
    image::Color,
    light::Light,
    geometry::Hit,
};

pub trait Material {
    fn shade(&self, ray: &Ray, hit: &Hit, lights: &Box<dyn Light>, depth: u32, color: &mut Color);
}

pub fn default() -> Box<lambertian::Lambertian> {
    Box::new(lambertian::Lambertian::new(Color::new(8.0, 8.0, 8.0)))
}
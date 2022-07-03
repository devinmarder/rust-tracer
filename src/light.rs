pub mod point_source;

use crate::{
    image::Color,
    vec3::Vec3, 
    geometry::Hit, 
    ray::Ray,
};

pub trait Light {
    fn sample(&self) -> Vec3;
    fn color(&self) -> Color;
    fn intensity(&self, ray: &Ray, hit: &Hit) -> f64;
}
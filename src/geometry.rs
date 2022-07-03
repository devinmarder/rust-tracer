pub mod plane;
pub mod sphere;

use crate::{
    ray::Ray,
    geometry::plane::Plane,
    vec3::Vec3,
};

pub trait Geometry{
    fn hit(&self, ray: &Ray) -> Option<Hit>;
    fn sample(&self, p: &Vec3) -> Vec3;
    fn normal_at(&self, p: &Vec3) -> Vec3;
    fn hit_point(&self, ray: &Ray) -> Vec3;
}

pub fn default() -> Box<Plane> {
    Box::new(Plane::default())
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hit {
    pub distance: f64,
    pub normal: Vec3,
    pub point: Vec3,
}

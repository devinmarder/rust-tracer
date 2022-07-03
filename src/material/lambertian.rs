use crate::{
    image::Color,
    ray::Ray,
    material::Material,
    light::Light,
    geometry::Hit,
};

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn shade(&self, ray: &Ray, hit: &Hit, light: &Box<dyn Light>, _depth: u32, color: &mut Color) {
        *color += self.albedo * light.intensity(ray, hit);
    }   
}

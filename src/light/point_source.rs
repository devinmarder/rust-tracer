use crate::{
    image::Color,
    vec3::Vec3,
    light::Light, geometry::Hit,
    ray::Ray,
};

pub struct PointSource {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f64,
}

impl PointSource {
    pub fn new(position: Vec3, color: Color, intensity: f64) -> PointSource {
        PointSource {
            position,
            color,
            intensity,
        }
    }
}

impl Light for PointSource {
    fn sample(&self) -> Vec3 {
        self.position
    }

    fn color(&self) -> Color {
        self.color
    }

    fn intensity(&self, _ray: &Ray, hit: &Hit) -> f64 {
        let distance = (hit.point - self.position).length();
        self.intensity / (distance * distance)
    }
}
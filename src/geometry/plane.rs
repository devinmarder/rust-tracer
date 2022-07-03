use crate::{
    vec3::Vec3,
    geometry::Geometry,
    ray::Ray,
    geometry::Hit,
};

#[derive(Debug, Clone)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f64,
}

impl Plane {
    pub fn new(normal: Vec3, distance: f64) -> Plane {
        Plane {
            normal,
            distance,
        }
    }
    pub fn default() -> Plane {
        Plane {
            normal: Vec3::new(0.0, -1.0, 0.0),
            distance: 0.0,
        }
    }
}

impl Geometry for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let denom = self.normal.dot(&ray.direction);
        let dist = (self.distance - self.normal.dot(&ray.origin)) / denom;
        if dist > 0.0 {
            Some(Hit {
                distance: dist,
                normal: self.normal,
                point: ray.origin + ray.direction * dist,
            })
        } else {
            None
        }
    }
    fn sample(&self, Vec3 { x, y, z }: &Vec3) -> Vec3 {
        Vec3::new(*x, *y, *z)
    }
    fn normal_at(&self, _: &Vec3) -> Vec3 {
        self.normal
    }
    fn hit_point(&self, ray: &Ray) -> Vec3 {
        ray.origin + ray.direction * self.distance
    }
}
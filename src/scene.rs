use crate::{
    object::{Object, self},
    ray::Ray,
    image::Color,
    light::Light,
    geometry::Hit,
};

pub struct Scene {
    objects: Vec<Object>,
    lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }
    pub fn color_at(&self, ray: Ray, depth: u32) -> Color {
        let mut minDist = f64::MAX;
        let mut hit_object: Option<(Hit, &Object)> = None;
        for object in &self.objects {
            if let Some(hit) = object.hit(&ray) {
                if hit.distance < minDist {
                    minDist = hit.distance;
                    hit_object = Some((hit, object));
                }
            }
        }
        match hit_object {
            Some((hit, object)) => 
                object.shade(&ray, &hit, &self.lights, &self.objects, depth),
            None => Color::new(0.0, 0.0, 0.0),
        }
    }
}


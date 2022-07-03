use std::{
    io::{stdout, Write},
};
use crate::camera::Camera;
use crate::scene::Scene;
use crate::image::{Image, Color};

pub struct Tracer {
    scene: Scene,
    camera: Camera,
    width: u32,
    height: u32,
    samples: u32,
    max_depth: u32,
}

impl Tracer {
    pub fn new(scene: Scene, camera: Camera, width: u32, height: u32, samples: u32, max_depth: u32) -> Tracer {
        Tracer {
            scene,
            camera,
            width,
            height,
            samples,
            max_depth,
        }
    }
    pub fn render(&self) -> Image {
        let mut image = Image::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples {
                    let u = (x as f64 + rand::random::<f64>()) / self.width as f64;
                    let v = (y as f64 + rand::random::<f64>()) / self.height as f64;
                    let ray = self.camera.get_ray(u, v);
                    color += self.scene.color_at(ray, self.max_depth);
                }
                color = color / self.samples as f64;
                image.set_pixel(x, y, color);
            }
            print!("\r{}%", ((y as f64 / self.height as f64) * 100.0 as f64).round());
            stdout().flush().unwrap();
        }
        println!("\r completed");
        image
    }
}
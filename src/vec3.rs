use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3::new(self.x / len, self.y / len, self.z / len)
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal) * normal
    }
    pub fn refract(&self, normal: &Vec3, eta: f64) -> Vec3 {
        let dn = self.dot(normal);
        let k = 1.0 - eta * eta * (1.0 - dn * dn);
        if k < 0.0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else {
            *self * eta - (*normal * eta * dn + *normal * k.sqrt())
        }
    }
    pub fn random() -> Vec3 {
        let x = rand::random::<f64>();
        let y = rand::random::<f64>();
        let z = rand::random::<f64>();
        Vec3::new(x, y, z)
    }
    pub fn random_in_unit_sphere() -> f64 {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen::<f64>();
        let mut y = rng.gen::<f64>();
        let mut z = rng.gen::<f64>();
        while x * x + y * y + z * z > 1.0 {
            x = rng.gen::<f64>();
            y = rng.gen::<f64>();
            z = rng.gen::<f64>();
        }
        x * 2.0 - 1.0
    }
    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen::<f64>();
        let mut y = rng.gen::<f64>();
        let mut z = 0.0;
        while x * x + y * y > 1.0 {
            x = rng.gen::<f64>();
            y = rng.gen::<f64>();
        }
        Vec3::new(x, y, z)
    }
    pub fn scale(&self, s: f64) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}


impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
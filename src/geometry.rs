use crate::v3::*;

pub struct Ray {
    pub origin: V3,
    /// not normalized
    pub direction: V3,
}

impl Ray {
    pub fn at(&self, t: f64) -> V3 {
        self.origin + self.direction.scale(t)
    }
}

pub fn hit_sphere(center: P3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.len2();
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.len2() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

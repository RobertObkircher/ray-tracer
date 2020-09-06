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

pub struct Hit {
    pub point: P3,
    /// against the ray, not the surface
    pub normal: V3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub struct Sphere {
    pub center: P3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len2();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let result = |t| {
                let point = ray.at(t);
                let outward = (point - self.center).div(self.radius);
                let front_face = ray.direction.dot(&outward) < 0.0;
                Some(Hit {
                    point,
                    normal: if front_face { outward } else { -outward },
                    t,
                    front_face,
                })
            };

            let first = (-half_b - root) / a;
            let second = (-half_b + root) / a;

            if first < t_max && first > t_min {
                result(first)
            } else if second < t_max && second > t_min {
                result(second)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct HittableList {
    pub spheres: Vec<Sphere>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit = None;
        let mut closest_so_far = t_max;

        for sphere in &self.spheres {
            if let Some(h) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = h.t;
                hit = Some(h);
            }
        }
        hit
    }
}

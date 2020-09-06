use crate::geometry::{Hit, Ray};
use crate::v3::*;

pub enum Material {
    Lambertian {
        albedo: C3,
    },
    Metal {
        albedo: C3,
        /// <= 1
        fuzz: f64,
    },
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(C3, Ray)> {
        match self {
            Self::Lambertian { albedo } => {
                let scattered = Ray {
                    origin: hit.point,
                    direction: hit.normal + V3::random_on_unit_sphere(),
                };
                Some((*albedo, scattered))
            }
            Self::Metal { albedo, fuzz } => {
                let scattered = Ray {
                    origin: hit.point,
                    direction: ray.direction.norm().reflect(&hit.normal)
                        + V3::random_in_unit_sphere().scale(*fuzz),
                };
                if scattered.direction.dot(&hit.normal) > 0.0 {
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }
        }
    }
}

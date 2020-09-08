use crate::geometry::{Hit, Ray};
use crate::v3::*;
use rand::random;

pub enum Material {
    Lambertian {
        albedo: C3,
    },
    Metal {
        albedo: C3,
        /// <= 1
        fuzz: f64,
    },
    Dielectric {
        ref_idx: f64,
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
            Self::Dielectric { ref_idx } => {
                let etai_over_etat = if hit.front_face {
                    1.0 / ref_idx
                } else {
                    *ref_idx
                };

                let unit_direction = ray.direction.norm();

                let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let direction = if etai_over_etat * sin_theta > 1.0 {
                    unit_direction.reflect(&hit.normal)
                } else if random::<f64>() < schlick(cos_theta, etai_over_etat) {
                    unit_direction.reflect(&hit.normal)
                } else {
                    unit_direction.refract(&hit.normal, etai_over_etat)
                };

                Some((
                    C3::all(1.0),
                    Ray {
                        origin: hit.point,
                        direction,
                    },
                ))
            }
        }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}


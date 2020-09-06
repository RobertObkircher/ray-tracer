use crate::geometry::Ray;
use crate::v3::*;

pub struct Camera {
    origin: P3,
    lower_left_corner: P3,
    horizontal: V3,
    vertical: V3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = P3::zero();
        let horizontal = v3(viewport_width, 0.0, 0.0);
        let vertical = v3(0.0, viewport_height, 0.0);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal.div(2.0)
                - vertical.div(2.0)
                - v3(0.0, 0.0, focal_length),
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal.scale(u) + self.vertical.scale(v)
                - self.origin,
        }
    }
}

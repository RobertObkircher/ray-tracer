use crate::geometry::Ray;
use crate::v3::*;

pub struct Camera {
    origin: P3,
    lower_left_corner: P3,
    horizontal: V3,
    vertical: V3,
}

impl Camera {
    pub fn new(
        lookfrom: P3,
        lookat: P3,
        view_up: V3,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        let h = (vertical_fov / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).norm();
        let u = view_up.cross(&w).norm();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u.scale(viewport_width);
        let vertical = v.scale(viewport_height);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal.div(2.0) - vertical.div(2.0) - w,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal.scale(s) + self.vertical.scale(t)
                - self.origin,
        }
    }
}

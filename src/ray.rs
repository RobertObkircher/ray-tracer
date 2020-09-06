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

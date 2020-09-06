use rand::random;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Point
pub type P3 = V3;

/// Color
pub type C3 = V3;

#[derive(Clone, Copy, Debug)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn v3(x: f64, y: f64, z: f64) -> V3 {
    V3 { x, y, z }
}

pub fn c3(x: f64, y: f64, z: f64) -> C3 {
    C3 { x, y, z }
}

pub fn p3(x: f64, y: f64, z: f64) -> P3 {
    P3 { x, y, z }
}

impl V3 {
    pub fn dot(&self, rhs: &V3) -> f64 {
        let v = self * rhs;
        v.x + v.y + v.z
    }

    pub fn cross(&self, rhs: &V3) -> V3 {
        V3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.len2())
    }

    pub fn len2(&self) -> f64 {
        self.dot(self)
    }

    pub fn norm(&self) -> V3 {
        self.div(self.len())
    }

    pub fn scale(&self, b: f64) -> V3 {
        V3 {
            x: self.x * b,
            y: self.y * b,
            z: self.z * b,
        }
    }

    pub fn div(&self, b: f64) -> V3 {
        V3 {
            x: self.x / b,
            y: self.y / b,
            z: self.z / b,
        }
    }

    pub fn zero() -> V3 {
        V3::all(0.0)
    }

    pub fn all(value: f64) -> V3 {
        v3(value, value, value)
    }

    pub fn random() -> V3 {
        v3(random(), random(), random())
    }

    pub fn random_min_max(min: f64, max: f64) -> V3 {
        V3::all(min) + V3::random() * (V3::all(max) - V3::all(min))
    }

    pub fn random_in_unit_sphere() -> P3 {
        loop {
            let point = P3::random_min_max(-1.0, 1.0);
            if point.len2() < 1.0 {
                return point;
            }
        }
    }

    // lambertian distribution
    // picking points on the surface of the unit sphere offset along the surface normal
    pub fn random_on_unit_sphere() -> P3 {
        let a = random::<f64>() * 2.0 * PI;
        let z = -1.0 + 2.0 * random::<f64>();
        let r = (1.0 - z * z).sqrt();
        p3(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: &V3) -> V3 {
        let in_unit_sphere = P3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn reflect(&self, normal: &V3) -> V3 {
        self - normal.scale(2.0 * self.dot(normal))
    }
}

impl Neg for V3 {
    type Output = V3;
    fn neg(self) -> Self::Output {
        V3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &V3 {
    type Output = V3;
    fn neg(self) -> Self::Output {
        V3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

macro_rules! bin_op_impl {
    ($trait:ident, $fun: ident, $left:ty, $right: ty) => {
        impl $trait<$right> for $left {
            type Output = V3;

            fn $fun(self, right: $right) -> Self::Output {
                V3 {
                    x: self.x.$fun(right.x),
                    y: self.y.$fun(right.y),
                    z: self.z.$fun(right.z),
                }
            }
        }
    };
}

macro_rules! bin_op {
    ($trait:ident, $fun: ident) => {
        bin_op_impl!($trait, $fun, V3, V3);
        bin_op_impl!($trait, $fun, &V3, V3);
        bin_op_impl!($trait, $fun, V3, &V3);
        bin_op_impl!($trait, $fun, &V3, &V3);
    };
}

macro_rules! assign_op {
    ($trait:ident, $fun: ident, $op: ident) => {
        impl $trait<V3> for V3 {
            fn $fun(&mut self, right: V3) {
                *self = self.$op(right);
            }
        }
        impl $trait<&V3> for V3 {
            fn $fun(&mut self, right: &V3) {
                *self = self.$op(right);
            }
        }
    };
}

bin_op!(Add, add);
assign_op!(AddAssign, add_assign, add);

bin_op!(Sub, sub);
assign_op!(SubAssign, sub_assign, sub);

bin_op!(Mul, mul);
assign_op!(MulAssign, mul_assign, mul);

bin_op!(Div, div);
assign_op!(DivAssign, div_assign, mul);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_op() {
        let v1 = v3(1.3, 4.5, 2.2);
        let v2 = v3(0.3, 9.8, 7.2);
        let difference = v1 - v2;
        assert_eq!(difference.x, v1.x - v2.x);
        assert_eq!(difference.y, v1.y - v2.y);
        assert_eq!(difference.z, v1.z - v2.z);
    }

    #[test]
    fn assign_op() {
        let v1 = v3(1.3, 4.5, 2.2);
        let v2 = v3(0.3, 9.8, 7.2);

        let mut difference = v1;
        difference -= v2;

        assert_eq!(difference.x, v1.x - v2.x);
        assert_eq!(difference.y, v1.y - v2.y);
        assert_eq!(difference.z, v1.z - v2.z);
    }

    #[test]
    fn len() {
        let v1 = v3(3.0, 4.0, 5.0);

        assert_eq!(v1.len2(), 50.0);
        assert_eq!(v1.len(), (50.0 as f64).sqrt());
        assert!((1.0 - v1.norm().len()).abs() < 0.000000001);
        assert!((1.0 - v1.norm().len2()).abs() < 0.000000001);
    }

    #[test]
    fn dot() {
        let v1 = v3(-1.0, -5.0, -2.0);
        let v2 = v3(-4.0, 4.0, 1.0);

        let result = v1.cross(&v2);
        let expected = v3(3.0, 9.0, -24.0);

        assert_eq!(result.x, expected.x);
        assert_eq!(result.y, expected.y);
        assert_eq!(result.z, expected.z);
    }
}

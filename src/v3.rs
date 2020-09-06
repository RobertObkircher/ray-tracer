use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Point3 = V3;
pub type Color = V3;

#[derive(Clone, Copy, Debug)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn v3(x: f64, y: f64, z: f64) -> V3 {
    V3 { x, y, z }
}

pub fn zero3() -> V3 {
    v3(0.0, 0.0, 0.0)
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

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn norm(&self) -> V3 {
        self.div(self.length())
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

        assert_eq!(v1.length_squared(), 50.0);
        assert_eq!(v1.length(), (50.0 as f64).sqrt());
        assert!((1.0 - v1.norm().length()).abs() < 0.000000001);
        assert!((1.0 - v1.norm().length_squared()).abs() < 0.000000001);
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

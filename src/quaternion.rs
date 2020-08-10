use crate::vec3::Vec3;
use std::ops::{Add, Mul};

/// Quaternion type, a 4 dimensional number type.
#[derive(Copy, Clone, Debug)]
pub struct Quaternion {
    s: f64,
    v: Vec3,
}

impl Quaternion {
    pub fn new(r: f64, i: f64, j: f64, k: f64) -> Self {
        Quaternion {
            s: r,
            v: (i, j, k).into(),
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.s * self.s + self.v.x * self.v.x + self.v.y * self.v.y + self.v.z * self.v.z).sqrt()
    }
}

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            s: self.s + rhs.s,
            v: self.v + rhs.v,
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            s: self.s * rhs.s - self.v.dot(rhs.v),
            v: self.s * rhs.v + rhs.s * self.v + self.v.cross(rhs.v),
        }
    }
}

impl Mul<Quaternion> for f64 {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            s: self * rhs.s,
            v: self * rhs.v,
        }
    }
}

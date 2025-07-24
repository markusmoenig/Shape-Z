use crate::prelude::*;

/// Every values is an [f32;3]
#[derive(Debug, Clone, Copy)]
pub struct Value(pub [f32; 3]);

impl Value {
    // Constructor

    #[inline]
    pub fn zero() -> Self {
        Self([0.0, 0.0, 0.0])
    }

    #[inline]
    pub fn one() -> Self {
        Self([1.0, 1.0, 1.0])
    }

    #[inline]
    pub fn from_float(x: f32) -> Self {
        Self([x, x, x])
    }

    #[inline]
    pub fn from_bool(b: bool) -> Self {
        let f = if b { 1.0 } else { 0.0 };
        Self([f, f, f])
    }

    #[inline]
    pub fn from_vec2(v: Vec2<f32>) -> Self {
        Self([v.x, v.y, 0.0])
    }

    #[inline]
    pub fn from_vec3(v: Vec3<f32>) -> Self {
        Self([v.x, v.y, v.z])
    }

    #[inline]
    pub fn from_components(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    // Getter

    #[inline]
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.0[2]
    }

    // Conversion

    #[inline]
    pub fn as_float(&self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn as_bool(&self) -> bool {
        self.0[0] != 0.0
    }

    #[inline]
    pub fn as_vec2(&self) -> Vec2<f32> {
        Vec2::new(self.0[0], self.0[1])
    }

    #[inline]
    pub fn as_vec3(&self) -> Vec3<f32> {
        Vec3::new(self.0[0], self.0[1], self.0[2])
    }

    // Ops

    #[inline]
    pub fn add(self, other: Self) -> Self {
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = self.0[i] + other.0[i];
        }
        Value(out)
    }

    #[inline]
    pub fn sub(self, other: Self) -> Self {
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = self.0[i] - other.0[i];
        }
        Value(out)
    }

    #[inline]
    pub fn mul(self, other: Self) -> Self {
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = self.0[i] * other.0[i];
        }
        Value(out)
    }

    #[inline]
    pub fn div(self, other: Self) -> Self {
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = self.0[i] / other.0[i];
        }
        Value(out)
    }

    #[inline]
    pub fn length(self) -> Self {
        let len = (self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2)).sqrt();
        Value([len, len, len])
    }

    #[inline]
    pub fn abs(self) -> Self {
        Value([self.0[0].abs(), self.0[1].abs(), self.0[2].abs()])
    }

    #[inline]
    pub fn truthy(self) -> bool {
        self.0[0] != 0.0
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = self.0[i].min(other.0[i]);
        }
        Value(out)
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = self.0[i].max(other.0[i]);
        }
        Value(out)
    }

    #[inline]
    pub fn mix(self, other: Self, factor: Self) -> Self {
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = self.0[i] * (1.0 - factor.0[i]) + other.0[i] * factor.0[i];
        }
        Value(out)
    }

    #[inline]
    pub fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
        let mut out = [0.0; 3];
        for i in 0..3 {
            let t = ((self.0[i] - edge0.0[i]) / (edge1.0[i] - edge0.0[i])).clamp(0.0, 1.0);
            out[i] = t * t * (3.0 - 2.0 * t);
        }
        Value(out)
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = self.0[i].clamp(min.0[i], max.0[i]);
        }
        Value(out)
    }

    #[inline]
    pub fn pow(self, exp: Self) -> Self {
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = self.0[i].powf(exp.0[i]);
        }
        Value(out)
    }

    #[inline]
    pub fn sqrt(self) -> Self {
        Value([self.0[0].sqrt(), self.0[1].sqrt(), self.0[2].sqrt()])
    }

    #[inline]
    pub fn log(self) -> Self {
        Value([self.0[0].ln(), self.0[1].ln(), self.0[2].ln()])
    }

    #[inline]
    pub fn dot(self, other: Self) -> Self {
        let dot = self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2];
        Value::from_float(dot)
    }

    #[inline]
    pub fn cross(self, other: Self) -> Self {
        let x = self.0[1] * other.0[2] - self.0[2] * other.0[1];
        let y = self.0[2] * other.0[0] - self.0[0] * other.0[2];
        let z = self.0[0] * other.0[1] - self.0[1] * other.0[0];
        Value([x, y, z])
    }

    #[inline]
    pub fn tan(self) -> Self {
        Value([self.0[0].tan(), self.0[1].tan(), self.0[2].tan()])
    }

    #[inline]
    pub fn atan(self) -> Self {
        Value([self.0[0].atan(), self.0[1].atan(), self.0[2].atan()])
    }

    #[inline]
    pub fn atan2(self, other: Self) -> Self {
        Value([
            self.0[0].atan2(other.0[0]),
            self.0[1].atan2(other.0[1]),
            self.0[2].atan2(other.0[2]),
        ])
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let len = (self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]).sqrt();

        if len != 0.0 {
            let inv = 1.0 / len;
            Value([self.0[0] * inv, self.0[1] * inv, self.0[2] * inv])
        } else {
            Self::zero()
        }
    }

    #[inline]
    pub fn sin(self) -> Self {
        Value([self.0[0].sin(), self.0[1].sin(), self.0[2].sin()])
    }

    #[inline]
    pub fn cos(self) -> Self {
        Value([self.0[0].cos(), self.0[1].cos(), self.0[2].cos()])
    }
}

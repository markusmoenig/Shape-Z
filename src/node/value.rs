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
}

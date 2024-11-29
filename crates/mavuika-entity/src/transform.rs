use std::fmt;

use bevy_ecs::prelude::*;

#[derive(Component, Clone)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
}

#[derive(Default, PartialEq, Clone, Debug, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn is_zero(&self) -> bool {
        self.x.abs() < 0.0000001 && self.y.abs() < 0.0000001 && self.z.abs() < 0.0000001
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn is_valid_rot(&self) -> bool {
        const VALID_RANGE: std::ops::Range<f32> = 0.0..361.0;

        self.is_valid()
            && VALID_RANGE.contains(&self.x)
            && VALID_RANGE.contains(&self.y)
            && VALID_RANGE.contains(&self.z)
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}

impl From<Vector3> for (f32, f32, f32) {
    fn from(value: Vector3) -> Self {
        (value.x, value.y, value.z)
    }
}

impl From<mavuika_proto::Vector> for Vector3 {
    fn from(value: mavuika_proto::Vector) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vector3> for mavuika_proto::Vector {
    fn from(value: Vector3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector3({},{},{})", self.x, self.y, self.z)
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[pos:{}|rot:{}]", self.position, self.rotation)
    }
}

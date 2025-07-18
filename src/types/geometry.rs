use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Point2D{
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Point3D{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Vector2D{
    pub x: f32,
    pub y: f32,
}

impl Vector2D{
    pub fn angle_to_vector(&self, vector2d: Vector2D) -> f32{
        let dot_product = self.x * vector2d.x + self.y * vector2d.y;

        0.0
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Vector3D{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Rectangle{
    pub width: i32,
    pub height: i32,
}

/// We're exporting meshes from Blender to OBJ file with forward axis -Z and up axis Y
///
/// So our Euler angles are mapped as follows:
/// - `Pitch`: X
/// - `Yaw`: Y
/// - `Roll`: Z
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct EulerAngles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}
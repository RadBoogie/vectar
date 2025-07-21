use std::f32::consts::PI;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    /// # angle_to_other_vector
    /// Pass a 2D vector and will return the angle in radians between self and the passed in vector.
    ///
    /// The vectors are world space and don't need to be normalised first.
    pub fn angle_to_other_vector(&self, other_vector: Vector2D) -> f32 {
        let dot_product = self.normalise().dot_product(&other_vector.normalise());

        f32::acos(dot_product)
    }

    /// # normalise()
    /// Get a normalised copy of the Vector2D i.e. the length of the vector is 1 and the x and y
    /// coords are adjusted to make that the case.
    pub fn normalise(&self) -> Self {
        let h = (self.x * self.x + self.y * self.y).sqrt();
        Self {
            x: self.x / h,
            y: self.y / h,
        }
    }

    /// # dot_product()
    /// Pass in a vector and this will return the dot product of self and the passed in vector
    pub fn dot_product(&self, vector: &Vector2D) -> f32 {
        let a1b1 = self.x * vector.x;
        let a1b2 = self.y * vector.y;
        a1b1 + a1b2
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Rectangle {
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

// Vector2D Tests

// normalise()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// # test_normalise_vector_positive_direction()
    /// Tests to ensure that a normalised vector with a positive direction is 1 unit long.
    ///
    /// Slightly problematic because we're comparing floating point values which can give rounding
    /// problems but calculating the normalised length should eliminate that.
    ///
    fn test_normalise_vector_positive_direction() {
        let x = 5.0;
        let y = 5.0;

        let vector2d = Vector2D { x, y };
        let normalised_vector = vector2d.normalise();

        let normalised_x = f32::sqrt(0.5);

        assert_eq!(normalised_vector.x, normalised_x);
        assert_eq!(normalised_vector.y, normalised_x);
    }

    #[test]
    /// # test_normalise_vector_negative_direction()
    /// Tests to ensure that a normalised vector with a negative direction is 1 unit long.
    ///
    /// Slightly problematic because we're comparing floating point values which can give rounding
    /// problems but calculating the normalised length should eliminate that.
    ///
    fn test_normalise_vector_negative_direction() {
        let x = -5.0;
        let y = -5.0;

        let vector2d = Vector2D { x, y };
        let normalised_vector = vector2d.normalise();

        let normalised_x = -f32::sqrt(0.5);

        assert_eq!(normalised_vector.x, normalised_x);
        assert_eq!(normalised_vector.y, normalised_x);
    }
}

// angle_to_other_vector()

#[test]
/// # test_angle_to_other_vector()
/// Tests that the angle between two non-normalised vectors is as expected.
///
/// Vectors are in world space.
fn test_angle_to_other_vector() {
    let vector2d = Vector2D { x: 0.0, y: 1.0  };

    let other_vector = Vector2D { x: 10.0, y: 0.0 };

    let angle = vector2d.angle_to_other_vector(other_vector);

    assert_eq!(angle, PI / 2.0 );
}

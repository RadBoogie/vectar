use std::f32::consts::PI;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Face {
    pub vert_indices: Vec<usize>,
}

impl Face {
    pub fn new() -> Self{
        Self {vert_indices: Vec::new()}
    }
}

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

impl Point3D {
    /// Translates the point by the given vector and returns a new Point3D
    pub fn translate(&self, vector: &Vector3D) -> Point3D {
        Point3D {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
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
    pub fn angle_to_other_vector(&self, other_vector: &Vector2D) -> f32 {
        let dot_product = self.normalise().dot_product(&other_vector.normalise());

        f32::acos(dot_product)
    }

    /// # normalise
    /// Get a normalised copy of the Vector2D i.e. the length of the vector is 1 and the x and y
    /// coords are adjusted to make that the case.
    pub fn normalise(&self) -> Self {
        let h = (self.x * self.x + self.y * self.y).sqrt();
        Self {
            x: self.x / h,
            y: self.y / h,
        }
    }

    /// # dot_product
    /// Pass in a vector and this will return the dot product of self and the passed in vector
    pub fn dot_product(&self, vector: &Vector2D) -> f32 {
        let a1b1 = self.x * vector.x;
        let a1b2 = self.y * vector.y;
        a1b1 + a1b2
    }

    /// # subtract
    /// Pass in vector and it will be subtracted from self and the result returned.
    ///
    /// This gives you the vector from the tip of self to the tip of vector.
    pub fn subtract(&self, vector: &Vector2D) -> Self {
        Self {
            x: vector.x - self.x,
            y: vector.y - self.y,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    /// # normalise
    /// Get a normalised copy of the Vector3D i.e. the length of the vector is 1 and the x, y, and z
    /// coords are adjusted to make that the case.
    pub fn normalise(&self) -> Self {
        let h = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self {
            x: self.x / h,
            y: self.y / h,
            z: self.z / h,
        }
    }

    /// # angle_to_other_vector
    /// Pass a 3D vector and will return the angle in radians between self and the passed in vector.
    ///
    /// The vectors are world space and don't need to be normalised first.
    pub fn angle_to_other_vector(&self, other_vector: &Vector3D) -> f32 {
        let dot_product = self.normalise().dot_product(&other_vector.normalise());
        f32::acos(dot_product)
    }

    /// # dot_product
    /// Pass in a vector and this will return the dot product of self and the passed in vector
    pub fn dot_product(&self, vector: &Vector3D) -> f32 {
        let a1b1 = self.x * vector.x;
        let a2b2 = self.y * vector.y;
        let a3b3 = self.z * vector.z;
        a1b1 + a2b2 + a3b3
    }

    /// # cross_product
    /// Pass in a vector and this will return the cross product of self and the passed in vector
    pub fn cross_product(&self, vector: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * vector.z - self.z * vector.y,
            y: self.z * vector.x - self.x * vector.z,
            z: self.x * vector.y - self.y * vector.x,
        }
    }

    /// # subtract
    /// Pass in vector and it will be subtracted from self and the result returned.
    ///
    /// This gives you the vector from the tip of self to the tip of vector.
    pub fn subtract(&self, vector: &Vector3D) -> Self {
        Self {
            x: vector.x - self.x,
            y: vector.y - self.y,
            z: vector.z - self.z,
        }
    }

    /// # set_length
    /// Returns a new Vector3D with the same direction as self but with the specified length.
    /// If the vector is zero, returns a zero vector.
    pub fn set_length(&self, length: f32) -> Self {
        let normalized = self.normalise();
        Self {
            x: normalized.x * length,
            y: normalized.y * length,
            z: normalized.z * length,
        }
    }

    /// Rotates the vector around the y-axis (yaw) by the given angle in radians
    pub fn rotate_yaw(&self, radians: f32) -> Vector3D {
        let cos_theta = f32::cos(radians);
        let sin_theta = f32::sin(radians);

        let tx = [
            [cos_theta, 0.0, sin_theta],
            [0.0, 1.0, 0.0],
            [-sin_theta, 0.0, cos_theta],
        ];

        let vector = [self.x, self.y, self.z];
        let mut result = [0.0, 0.0, 0.0];

        for i in 0..3 {
            for j in 0..3 {
                result[i] += tx[i][j] * vector[j];
            }
        }

        Vector3D {
            x: result[0],
            y: result[1],
            z: result[2],
        }
    }

    pub fn rotate(&self, rotation: &Vector3D) -> Vector3D {
        let (sy, cy) = rotation.y.sin_cos(); // Yaw
        let (sp, cp) = rotation.x.sin_cos(); // Pitch
        let (sr, cr) = rotation.z.sin_cos(); // Roll
        let m = [
            [cy * cr + sy * sp * sr, sy * sp * cr - cy * sr, sy * cp],
            [cp * sr, cp * cr, -sp],
            [sy * cr - cy * sp * sr, sy * sr + cy * sp * cr, cy * cp],
        ];
        Vector3D {
            x: m[0][0] * self.x + m[0][1] * self.y + m[0][2] * self.z,
            y: m[1][0] * self.x + m[1][1] * self.y + m[1][2] * self.z,
            z: m[2][0] * self.x + m[2][1] * self.y + m[2][2] * self.z,
        }
    }
}

impl From<&Point3D> for Vector3D {
    fn from(point: &Point3D) -> Self {
        Vector3D {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }
}

impl From<Vector3D> for EulerAngles {
    fn from(vec: Vector3D) -> Self {
        let x = vec.x;
        let y = vec.y;
        let z = vec.z;

        // Compute yaw (rotation around Y-axis, in XZ-plane)
        let yaw = f32::atan2(z, x); // Range: [-π, π]

        // Compute pitch (angle from XZ-plane to the vector)
        let xz_len = (x * x + z * z).sqrt();
        let pitch = f32::atan2(y, xz_len); // Range: [-π/2, π/2]

        // Roll is 0 (no twist information from a single vector)
        let roll = 0.0;

        EulerAngles { yaw, pitch, roll }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

/// We're exporting meshes from Blender to OBJ file with forward axis -Z and up axis Y
///
/// So our Euler angles are mapped as follows:
/// - `Pitch`: X
/// - `Yaw`: Y
/// - `Roll`: Z
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct EulerAngles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl From<EulerAngles> for Vector3D {
    fn from(euler: EulerAngles) -> Self {
        let (sin_pitch, cos_pitch) = euler.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = euler.yaw.sin_cos();

        // Compute direction vector components
        let x = cos_pitch * cos_yaw;
        let y = sin_pitch;
        let z = cos_pitch * sin_yaw;

        Vector3D { x, y, z }
    }
}







/////////////////
// Vector2D Tests
/////////////////

// normalise()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// # test_normalise_vector_positive_direction
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
    /// # test_normalise_vector_negative_direction
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
/// # test_angle_to_other_vector
/// Tests that the angle between two non-normalised vectors is as expected.
///
/// Vectors are in world space.
fn test_angle_to_other_vector() {
    let vector2d = Vector2D { x: 0.0, y: 1.0  };

    let other_vector = Vector2D { x: 10.0, y: 0.0 };

    let angle = vector2d.angle_to_other_vector(&other_vector);

    assert_eq!(angle, PI / 2.0 );
}


// subtract()

#[test]
/// # test_subtract
/// Tests that subtracting vector a from vector b results in a vector from a to b.
fn test_subtract() {
    let vector2d = Vector2D { x: 4.0, y: 6.0 };
    let other_vector = Vector2D { x: 1.0, y: 2.0 };
    let resultant = vector2d.subtract(&other_vector);

    assert_eq!(-3.0, resultant.x); // Updated to B - A
    assert_eq!(-4.0, resultant.y); // Updated to B - A
}

#[test]
/// # test_subtract_negative
/// Tests that subtracting vector a from vector b results in a vector from a to b.
fn test_subtract_negative() {
    let vector2d = Vector2D { x: 0.0, y: -10.0 };
    let other_vector = Vector2D { x: -2.0, y: 0.0 };
    let resultant = vector2d.subtract(&other_vector);

    assert_eq!(-2.0, resultant.x); // Already correct for B - A
    assert_eq!(10.0, resultant.y); // Already correct for B - A
}



/////////////////
// Vector3D Tests
/////////////////

#[test]
/// # test_subtract_vector3d
/// Tests that subtracting vector a from vector b results in a vector from a to b.
fn test_subtract_vector3d() {
    let vector3d = Vector3D { x: 4.0, y: 6.0, z: 8.0 };
    let other_vector = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    let resultant = vector3d.subtract(&other_vector);

    assert_eq!(-3.0, resultant.x); // B - A = 1.0 - 4.0
    assert_eq!(-4.0, resultant.y); // B - A = 2.0 - 6.0
    assert_eq!(-5.0, resultant.z); // B - A = 3.0 - 8.0
}

#[test]
/// # test_subtract_negative_vector3d
/// Tests that subtracting vector a from vector b results in a vector from a to b.
fn test_subtract_negative_vector3d() {
    let vector3d = Vector3D { x: 0.0, y: -10.0, z: 5.0 };
    let other_vector = Vector3D { x: -2.0, y: 0.0, z: -3.0 };
    let resultant = vector3d.subtract(&other_vector);

    assert_eq!(-2.0, resultant.x); // B - A = -2.0 - 0.0
    assert_eq!(10.0, resultant.y); // B - A = 0.0 - (-10.0)
    assert_eq!(-8.0, resultant.z); // B - A = -3.0 - 5.0
}

#[test]
/// Tests the dot product of two 3D vectors.
fn test_dot_product_vector3d() {
    let vector1 = Vector3D { x: 4.0, y: 6.0, z: 8.0 };
    let vector2 = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
    let result = vector1.dot_product(&vector2);
    assert_eq!(40.0, result);
}

#[test]
/// Tests the angle between two 3D vectors in radians.
fn test_angle_to_other_vector3d() {
    let vector1 = Vector3D { x: 4.0, y: 0.0, z: 0.0 };
    let vector2 = Vector3D { x: 0.0, y: 3.0, z: 0.0 };
    let result = vector1.angle_to_other_vector(&vector2);
    assert!((result - std::f32::consts::FRAC_PI_2).abs() < 1e-5); // Approx 1.5708 radians (90 degrees)
}

#[test]
/// Tests setting the length of a 3D vector.
fn test_set_length_vector3d() {
    let vector = Vector3D { x: 3.0, y: 4.0, z: 0.0 };
    let result = vector.set_length(10.0);
    assert!((result.x - 6.0).abs() < 1e-5);
    assert!((result.y - 8.0).abs() < 1e-5);
    assert!((result.z - 0.0).abs() < 1e-5);
    // Verify new length
    let magnitude = (result.x * result.x + result.y * result.y + result.z * result.z).sqrt();
    assert!((magnitude - 10.0).abs() < 1e-5);
}
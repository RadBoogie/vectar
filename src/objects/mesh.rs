use serde::{Deserialize, Serialize};
use crate::types::geometry::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mesh {
    pub verts: Vec<Point3D>,
    pub faces: Vec<Face>,
    pub position: Vector3D,
    pub rotation: EulerAngles,
}

impl Mesh {
    /// Gets a Vec of points for this mesh that have been transformed according to position
    /// and rotation.
    pub fn get_transformed_verts(&self) -> Vec<Point3D> {
        let transformed_verts: &mut Vec<Point3D> = &mut Vec::new();

        for vert in &self.verts {
            transformed_verts.push(self.transform_point(vert));
        }

        transformed_verts.clone()
    }

    /// We apply the rotation first which is local space and then apply the translation which is
    /// World space.
    fn transform_point(&self, point: &Point3D) -> Point3D {
        let transformed_point = &self.apply_rotation(point);
        self.translate_point(transformed_point)
    }


    /// We apply rotations in the order Yaw, Pitch, Roll (YXZ)
    fn apply_rotation(&self, point: &Point3D) -> Point3D {
        let rotated_point = self.rotate_yaw(&point);
        let rotated_point = self.rotate_pitch(&rotated_point);
        self.rotate_roll(&rotated_point)
    }

    /// Translates the mesh's position by the given point, returning the new position in world space.
    pub fn translate_point(&self, point: &Point3D) -> Point3D {
        Point3D {
            x: self.position.x + point.x,
            y: self.position.y + point.y,
            z: self.position.z + point.z,
        }
    }

    //TODO: Break these out into geometry...

    fn rotate_yaw(&self, point: &Point3D) -> Point3D {
        let tx = [
            [f32::cos(self.rotation.yaw), 0.0, f32::sin(self.rotation.yaw), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-f32::sin(self.rotation.yaw), 0.0, f32::cos(self.rotation.yaw), 0.0],
            [0.0, 0.0, 0.0, 1.0]];

        let point_h = [point.x, point.y, point.z, 1.0];

        let mut result = [0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            for j in 0..4 {
                result[i] += tx[i][j] * point_h[j];
            }
        }

        Point3D { x: result[0], y: result[1], z: result[2] }
    }

    fn rotate_pitch(&self, point: &Point3D) -> Point3D {
        let tx = [
            [f32::cos(self.rotation.pitch), -f32::sin(self.rotation.pitch), 0.0, 0.0],
            [f32::sin(self.rotation.pitch), f32::cos(self.rotation.pitch), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ];

        let point_h = [point.x, point.y, point.z, 1.0];

        let mut result = [0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            for j in 0..4 {
                result[i] += tx[i][j] * point_h[j];
            }
        }

        Point3D { x: result[0], y: result[1], z: result[2] }
    }

    fn rotate_roll(&self, point: &Point3D) -> Point3D {
        let tx = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, f32::cos(self.rotation.roll), -f32::sin(self.rotation.roll), 0.0],
            [0.0, f32::sin(self.rotation.roll), f32::cos(self.rotation.roll), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ];

        let point_h = [point.x, point.y, point.z, 1.0];

        let mut result = [0.0, 0.0, 0.0, 0.0];

        for i in 0..4 {
            for j in 0..4 {
                result[i] += tx[i][j] * point_h[j];
            }
        }

        Point3D { x: result[0], y: result[1], z: result[2] }
    }
}






#[test]
fn test_translate_point() {
    let mesh = Mesh {
        verts: vec![],
        faces: vec![],
        position: Vector3D { x: 1.0, y: 2.0, z: 3.0 },
        rotation: EulerAngles { pitch: 0.0, yaw: 0.0, roll: 0.0 },
    };
    let point = Point3D { x: 2.0, y: 3.0, z: 4.0 };
    let new_position = mesh.translate_point(&point);
    assert_eq!(new_position, Point3D { x: 3.0, y: 5.0, z: 7.0 });
}

#[test]
fn test_rotate_yaw() {
    let mesh = Mesh {
        verts: vec![],
        faces: vec![],
        position: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
        rotation: EulerAngles { yaw: std::f32::consts::FRAC_PI_2, pitch: 0.0, roll: 0.0 }, // 90 degrees
    };
    let point = Point3D { x: 1.0, y: 0.0, z: 0.0 };
    let result = mesh.rotate_yaw(&point);
    // 90-degree yaw: (x, z) -> (z, -x), y unchanged
    assert!((result.x - 0.0).abs() < 1e-5, "x: {}", result.x);
    assert!((result.y - 0.0).abs() < 1e-5, "y: {}", result.y);
    assert!((result.z - -1.0).abs() < 1e-5, "z: {}", result.z);
}

#[test]
fn test_rotate_pitch() {
    let mesh = Mesh {
        verts: vec![],
        faces: vec![],
        position: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
        rotation: EulerAngles { yaw: 0.0, pitch: std::f32::consts::FRAC_PI_2, roll: 0.0 }, // 90 degrees
    };
    let point = Point3D { x: 1.0, y: 0.0, z: 0.0 };
    let result = mesh.rotate_pitch(&point);
    // 90-degree pitch: (x, y) -> (-y, x), z unchanged
    assert!((result.x - 0.0).abs() < 1e-5);
    assert!((result.y - 1.0).abs() < 1e-5);
    assert!((result.z - 0.0).abs() < 1e-5);
}

#[test]
fn test_rotate_roll() {
    let mesh = Mesh {
        verts: vec![],
        faces: vec![],
        position: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
        rotation: EulerAngles { yaw: 0.0, pitch: 0.0, roll: std::f32::consts::FRAC_PI_2 }, // 90 degrees
    };
    let point = Point3D { x: 0.0, y: 1.0, z: 0.0 };
    let result = mesh.rotate_roll(&point);
    // 90-degree roll: (y, z) -> (-z, y), x unchanged
    assert!((result.x - 0.0).abs() < 1e-5);
    assert!((result.y - 0.0).abs() < 1e-5);
    assert!((result.z - 1.0).abs() < 1e-5);
}

#[test]
fn test_apply_rotation() {
    let mesh = Mesh {
        verts: vec![],
        faces: vec![],
        position: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
        rotation: EulerAngles {
            yaw: std::f32::consts::FRAC_PI_2, // 90 degrees
            pitch: 0.0,
            roll: 0.0,
        },
    };
    let point = Point3D { x: 1.0, y: 0.0, z: 0.0 };
    let result = mesh.apply_rotation(&point);
    // YXZ: yaw only → (x, z) -> (z, -x), y unchanged
    assert!((result.x - 0.0).abs() < 1e-5, "x: {}", result.x);
    assert!((result.y - 0.0).abs() < 1e-5, "y: {}", result.y);
    assert!((result.z - -1.0).abs() < 1e-5, "z: {}", result.z);
}
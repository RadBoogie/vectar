use serde::{Deserialize, Serialize};
use crate::types::geometry::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Mesh {
    pub verts: Vec<Point3D>,
    pub position: Point3D,
    pub rotation: EulerAngles,
}

impl Mesh {
    pub fn add_vertex(&mut self, vertex: Point3D) {
        self.verts.push(vertex);
    }

    /// Gets a Vec of points for this mesh that have been transformed according to position
    /// and rotation.
    pub fn get_transformed_verts(&self) -> Vec<Point3D> {
        let transformed_verts: &mut Vec<Point3D> = &mut Vec::new();

        for vert in &self.verts {
            transformed_verts.push(self.transform_point(vert));
        }

        transformed_verts.clone()
    }

    fn transform_point(&self, point: &Point3D) -> Point3D {
        let transformed_point = &self.apply_translation(point);
        self.apply_rotation(transformed_point)
    }

    fn apply_translation(&self, point: &Point3D) -> Point3D {
        self.translate_point(point)
    }

    /// We apply rotations in the order Yaw, Pitch, Roll (YXZ)
    fn apply_rotation(&self, point: &Point3D) -> Point3D {
        let rotated_point = self.rotate_yaw(&point);
        let rotated_point = self.rotate_pitch(&rotated_point);
        self.rotate_roll(&rotated_point)
    }

    fn translate_point(&self, point: &Point3D) -> Point3D {
        let tx = [
            [1.0, 0.0, 0.0, point.x],
            [0.0, 1.0, 0.0, point.y],
            [0.0, 0.0, 1.0, point.z],
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
            [1.0, 0.0, 0.0, 0.0],
            [0.0, f32::cos(self.rotation.yaw), -f32::sin(self.rotation.yaw), 0.0],
            [0.0, f32::sin(self.rotation.yaw), f32::cos(self.rotation.yaw), 0.0],
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

    fn rotate_roll(&self, point: &Point3D) -> Point3D {
        let tx = [
            [f32::cos(self.rotation.yaw), -f32::sin(self.rotation.yaw), 0.0, 0.0],
            [f32::sin(self.rotation.yaw), f32::cos(self.rotation.yaw), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
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
}
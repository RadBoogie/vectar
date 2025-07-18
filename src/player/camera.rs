use crate::types::geometry::*;

pub struct Camera{
    position: Point3D,
    direction: Vector3D,
    view_angle: f32,
    viewport: Rectangle,
    near_plane_distance: f32,
    far_plane_distance: f32,
}

impl Camera{
    pub fn new(
        position: Point3D,
        direction: Vector3D,
        view_angle: f32,
        viewport: Rectangle,
        near_plane_distance: f32,
        far_plane_distance: f32,
    ) -> Self {
        Self { position, direction, view_angle, viewport, near_plane_distance, far_plane_distance }
    }

    pub fn to_2d(&self, points_3d: &Vec<Point3D>) -> Vec<Point2D>{
        //TODO: Function to take a 3D Object and render to 2D

        // Is any point within our view? If not scram

        // Calculate x angle from our view direction

        // x = Tan theta * near_plane_distance




        Vec::new()
    }

    //TODO: 



}
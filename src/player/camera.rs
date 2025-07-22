use crate::types::geometry::*;

pub struct Camera {
    position: Point3D,
    direction: Vector3D,
    view_angle: f32,
    viewport: Rectangle,
    near_plane_distance: f32,
    far_plane_distance: f32,
}

impl Camera {
    pub fn new(
        position: Point3D,
        direction: Vector3D,
        view_angle: f32, //TODO: Presume this sets the near plane distance???
        viewport: Rectangle,
        near_plane_distance: f32,
        far_plane_distance: f32,
    ) -> Self {
        Self {
            position,
            direction,
            view_angle,
            viewport,
            near_plane_distance,
            far_plane_distance,
        }
    }

    pub fn to_2d(&self, points_3d: &Vec<Point3D>) -> Vec<Point2D> {
        //TODO: Function to take a 3D Object and render to 2D

        //TODO: Is any point within our view? If not scram
     //   println!("**** START ****");

        let mut projected_points: Vec<Point2D> = Vec::new();

     //   println!("points_3d: {:?}", points_3d);

        for point in points_3d {
        //    println!("**** VERTEX ****");

            // Vector from World origin to vertex
            let vertex_vector = Vector3D {
                x: point.x,
                y: point.y,
                z: point.z,
            };
        //    println!("Vertex vector: {:?}", vertex_vector);

            // Vector to camera position from World origin
            let camera_position_vector = Vector3D {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z,
            };
        //    println!("Camera position vector: {:?}", camera_position_vector);

            // Convert vertex vector origin to match camera origin
            let localised_vertex_vector = &vertex_vector.subtract(&camera_position_vector);
         //   println!("Localised vertex vector: {:?}", localised_vertex_vector);

            // The direction the camera is facing
            let camera_vector = Vector3D {
                x: self.direction.x,
                y: self.direction.y,
                z: self.direction.z,
            }.normalise();
          //  println!("Camera vector: {:?}", camera_vector);

            // Get the angle between the vertex vector and the camera look vector
            let angle_between_vectors = camera_vector.angle_to_other_vector(&localised_vertex_vector);

         //   println!("Angle between vectors: {}", angle_between_vectors);

            // Calculate vector for h i.e. where the vertex vector hits the near plane
            let opposite_side = f32::tan(angle_between_vectors) * self.near_plane_distance;

         //   println!("opposite_side: {}", opposite_side);

            // h is the hypotenuse of a right angle triangle where a is the line from the eye to the
            // near plane, and o is the length of the vertex vector where it hits the near plane
            let h = ((opposite_side * opposite_side) + (self.near_plane_distance * self.near_plane_distance)).sqrt();

         //   println!("h: {}", h);

            // Shorten vertex vector to touch near plane...
            let scaled_localised_vertex_vector = localised_vertex_vector.set_length(h);
          //  println!("Scaled localised vertex vector: {:?}", scaled_localised_vertex_vector);

            // Finally we get a vector from the point where the camera vector touches the near plane
            // to the point where the scaled_localised_vertex_vector touches the near plane. The
            // X & Y coords give us our 3d projection
            let vector_camera_to_point = camera_vector.subtract(&scaled_localised_vertex_vector);
          //  println!("Vector camera to point: {:?}", vector_camera_to_point);

            projected_points.push(Point2D { x: vector_camera_to_point.x, y: vector_camera_to_point.y });
        }

        projected_points
    }
}

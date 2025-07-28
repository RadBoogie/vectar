use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::geometry::*;

pub struct Camera {
    pub position: Point3D,
    pub rotation: EulerAngles,
    view_angle: f32,
    pub viewport: Rectangle,
    near_plane_distance: f32,
    far_plane_distance: f32,
}

impl Camera {
    pub fn new(
        position: Point3D,
        rotation: EulerAngles,
        view_angle: f32,
        viewport: Rectangle,
        far_plane_distance: f32,
    ) -> Self {
        let near_plane_distance = Self::calc_near_plane_distance(view_angle, &viewport);

        Self {
            position,
            rotation,
            view_angle,
            viewport,
            near_plane_distance,
            far_plane_distance,
        }
    }

    pub fn update_viewport_size(&mut self, viewport_size: Rectangle) {
        self.viewport = viewport_size;
        self.near_plane_distance = Self::calc_near_plane_distance(self.view_angle, &self.viewport);
    }

    pub fn rotate(&mut self, x_delta: f32, y_delta: f32) {
        self.rotation.yaw += x_delta.to_radians();

        //TODO: Seems that pitch always applies to World X?
        self.rotation.pitch += y_delta.to_radians();

      //  println!("Yaw: {:?}", self.rotation.yaw.to_degrees());
      //  println!("Pitch: {:?}", self.rotation.pitch.to_degrees());
    }

    pub fn move_strafe(&mut self, delta: f32) {
        // move camera sideways by delta
        let camera_vector: Vector3D = self.rotation.into();
        let camera_vector_rotated_90 = camera_vector.rotate_yaw(90.0_f32.to_radians());

        let scaled_vector = camera_vector_rotated_90.set_length(delta);

        self.position = self.position.translate(&scaled_vector);

      //  println!("Camera Vector: {:?}", self.position);
    }

    pub fn move_forward(&mut self, delta: f32) {
        // move camera along its vector by delta
        let camera_vector: Vector3D = self.rotation.into();

        let scaled_vector = camera_vector.set_length(delta);

        self.position = self.position.translate(&scaled_vector);

     //   println!("Camera Vector: {:?}", self.position);
    }

    fn calc_near_plane_distance(view_angle: f32, viewport: &Rectangle) -> f32 {
        let half_view_port_width = viewport.width as f32 / 2.0;
        half_view_port_width / ((view_angle.to_radians() / 2.0).tan())
    }

    /// # to_2d
    /// The strategy is as follows:
    /// For the vertex that we want to render to the 2D viewport, we first get the vector to the
    /// vertex in World space i.e. the vector relative to World origin at 0, 0, 0.
    ///
    /// We then get the World space vector to the camera location.
    ///
    /// We need to know the angle between where the camera is looking and the vertex so we convert
    /// the vertex vector to be relative (local) to the camera position. The camera position can be
    /// considered the camera local origin or where the eye is.
    ///
    /// Next we get the camera vector i.e. where the camera is looking.
    ///
    /// Now we can determine the angle between the vertex and the camera direction.
    ///
    /// The 2D projection will be the coordinate on the viewport of the location where the vertex
    /// vector passes through it. To find this we calculate the length of the opposite side on the
    /// right angle triangle formed by the near plane distance and the angle we obtained earlier.
    /// NOTE: The distance to the near plane is also the length of the adjacent in our triangle.
    ///
    /// Having the opposite side length we can calculate h (the hypotenuse) of our triangle which is
    /// the length of the vertex vector where it touches the near plane.
    ///
    /// Now we have two vectors, the camera vector to the centre of the viewport (near plane) and
    /// a vector to where the vertex vector touches the near plane. Subtracting the two gives us
    /// a vector from where the camera vector touches the near plane to where the vertex vector
    /// touches the near plane, and the end point holds the 2d (x, y) coordinates of the projected
    /// point.
    pub fn to_2d(&self, points_3d: &Vec<Point3D>) -> Vec<Point2D> {
        //TODO: Is any point within our view? If not scram

        let mut projected_points: Vec<Point2D> = Vec::new();

        unsafe{
            COUNTER += 1;
        }

        let mut vert_count = 0;



        for point in points_3d {
           // let debug = true && point.eq(points_3d.first().unwrap());
            let mut debug = false;
            unsafe {
                if COUNTER % 60 == 0 {
                    debug = true;
                }
            }

            vert_count += 1;

            if debug {
             //   println!(" ");
             //   println!("{} Project Vertex", vert_count);
             //   println!("**************");

               // println!("Yaw: {:?}", self.rotation.yaw.to_degrees());
               // println!("Pitch: {:?}", self.rotation.pitch.to_degrees());
               // println!("Roll: {:?}", self.rotation.roll.to_degrees());
               // println!("Position: {:?}", self.position);
               // println!("**************");
            }

            if debug {
                //   println!("Opposite side: {:?}", opposite_side);
            }

            // Vector from World origin to vertex
            let vertex_vector_world_space = Vector3D::from(point);

            // Vector to camera position from World origin
            let camera_position_vector = Vector3D::from(&self.position);

            // Convert vertex vector origin to match camera origin
            let localised_vertex_vector = &camera_position_vector.subtract(&vertex_vector_world_space);
            //  let localised_vertex_vector = &vertex_vector_world_space.subtract(&camera_position_vector);

            // The direction the camera is facing
            let camera_vector: Vector3D = self.rotation.into();
            let camera_vector = camera_vector.clone().normalise();

            // Get the angle between the vertex vector and the camera look vector
            let angle_between_vectors = camera_vector.angle_to_other_vector(&localised_vertex_vector);

            // Calculate vector for h i.e. where the vertex vector hits the near plane
            let opposite_side = f32::tan(angle_between_vectors) * self.near_plane_distance;

            // h is the hypotenuse of a right angle triangle where a is the line from the eye to the
            // near plane, and o is the length of the vertex vector where it hits the near plane
            let h = ((opposite_side * opposite_side)
                + (self.near_plane_distance * self.near_plane_distance)).sqrt();

            //TODO: I reckon this is where it all goes wrong. For some reason the rotation pitch is
            // with reference to World X not local camera X. Actually the rotation point seems to
            // be very far off to the -Z. Off the screen in fact.

            let (axis, angle) = camera_vector.get_rotation_to_z_axis();
            let camera_vector = camera_vector.rotate_around_axis(&axis, angle);
            let localised_vertex_vector = localised_vertex_vector.normalise().rotate_around_axis(&axis, angle);


            if debug {
                   println!("Angle between vectors: {:?}", angle_between_vectors.to_degrees());
                    println!("Opposite side: {:?}", opposite_side);
                    println!("Hypotenuse: {:?}", h);
                    println!("Camera Vector: {:?}", camera_vector);
                    println!("Localised Vertex Vector: {:?}", localised_vertex_vector);
                    println!("**************");
            }

          //  let localised_vertex_vector = localised_vertex_vector.rotate_pitch(self.rotation.pitch * 10.0);

            // Shorten vertex vector to touch near plane...
            let scaled_localised_vertex_vector = localised_vertex_vector.set_length(h);

            // Finally we get a vector from the point where the camera vector touches the near plane
            // to the point where the scaled_localised_vertex_vector touches the near plane. The
            // X & Y coords give us our 3d projection
            let scaled_vector_camera = camera_vector.set_length(self.near_plane_distance);
            let vector_camera_to_point = scaled_vector_camera.subtract(&scaled_localised_vertex_vector);

            projected_points.push(Point2D {
                x: vector_camera_to_point.x,
                y: vector_camera_to_point.y,
            });
        }

        projected_points
    }
}

static mut COUNTER: i32 = 0;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// # test_calc_near_plane_distance
    fn test_calc_near_plane_distance() {
        let view_angle = 90.0;
        let viewport = Rectangle {
            width: 800.0,
            height: 600.0,
        };

        let near_plane_distance = Camera::calc_near_plane_distance(view_angle, &viewport);

        assert_eq!(near_plane_distance, 400.0);
    }
}

use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::geometry::*;

pub struct Camera {
    pub position: Point3D,
    pub rotation: EulerAngles,
    pub rotation_vector: Vector3D,
    view_angle: f32,
    pub viewport: Rectangle,
    pub near_plane_distance: f32,
    far_plane_distance: f32,
}

impl Camera {
    pub fn new(
        position: Point3D,
        rotation: EulerAngles,
        rotation_v: Vector3D,
        view_angle: f32,
        viewport: Rectangle,
        far_plane_distance: f32,
    ) -> Self {
        let near_plane_distance = Self::calc_near_plane_distance(view_angle, &viewport);

        Self {
            position,
            rotation,
            rotation_vector: rotation_v,
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
        self.rotation_vector = self.rotate_yaw(x_delta.to_radians());

        self.rotation.pitch += y_delta.to_radians();
        self.rotation_vector = self.rotate_pitch(y_delta.to_radians());
    }

     fn rotate_pitch(&mut self, pitch_delta: f32) -> Vector3D {
         println!("Pitch Delta: {:?}", pitch_delta);

         let world_up_vector = Vector3D {x: 0.0, y: 1.0, z: 0.0 };

         let x_axis = self.rotation_vector.cross_product(&world_up_vector).normalise();

         println!("X Axis: {:?}", x_axis);

         self.rotation_vector.rotate_around_axis(&x_axis, pitch_delta)
    }

    fn rotate_yaw(&mut self, yaw_delta: f32) -> Vector3D {
        println!("Yaw Delta: {:?}", yaw_delta);

        let world_up_vector = Vector3D {x: 0.0, y: 1.0, z: 0.0 };

        self.rotation_vector.rotate_around_axis(&world_up_vector, yaw_delta)
    }

    pub fn move_strafe(&mut self, delta: f32) {
        // move camera sideways by delta
      //  let camera_vector: Vector3D = self.rotation.into();
        let camera_vector_rotated_90 = self.rotation_vector.rotate_yaw(90.0_f32.to_radians());

        let scaled_vector = camera_vector_rotated_90.set_length(delta);

        self.position = self.position.translate(&scaled_vector);

      //  println!("Camera Vector: {:?}", self.position);
    }

    pub fn move_forward(&mut self, delta: f32) {
        // move camera along its vector by delta
     //   let camera_vector: Vector3D = self.rotation.into();

        let scaled_vector = self.rotation_vector.set_length(delta);

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

            let vertex_vector_ws = Vector3D::from(point);

            let camera_position_vector_ws = Vector3D::from(&self.position);

            let localised_vertex_vector = &vertex_vector_ws.subtract(&camera_position_vector_ws);

            let angle_between_vectors = self.rotation_vector.angle_to_other_vector(&localised_vertex_vector); // Get the angle between the vertex vector and the camera look vector

            let h = self.near_plane_distance / angle_between_vectors.cos();

            let (axis, angle) = self.rotation_vector.get_rotation_to_z_forward();

            let localised_vertex_vector = localised_vertex_vector.reorient_to_local_space(&self.rotation_vector);

            let scaled_localised_vertex_vector = localised_vertex_vector.set_length(h); // Shorten vertex vector to touch near plane...

            let viewport_width_div_2 = &self.viewport.width / 2.0;
            let viewport_height_div_2 = &self.viewport.height / 2.0;

            projected_points.push(Point2D {
                x: scaled_localised_vertex_vector.x + viewport_width_div_2,
                y: scaled_localised_vertex_vector.y + viewport_height_div_2,
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

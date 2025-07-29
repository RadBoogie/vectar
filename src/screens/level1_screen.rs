//TODO: Code for the Level1 screen
use std::str::from_utf8;
use eframe::egui::{Align2, Color32, FontId, Painter, Pos2, Stroke};

use crate::screens::traits::ScreenRenderer;
use crate::types::geometry::*;

use serde::{Deserialize, Serialize};

use rust_embed::RustEmbed;
use crate::objects::mesh::Mesh;
use crate::player::camera::Camera;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;


pub struct Level1Screen{
    pub meshes: Vec<Mesh>,
}

impl Level1Screen {
    pub fn new() -> Self {
        /*TODO: We don't want to re-load the meshes every time
            if we're re-creating the screen every render! Need to load the meshes once */ 

        let meshes = load_assets().expect("Failed to load meshes");

        Self { meshes, }
    }
}


// Intermediate struct to match the JSON structure
#[derive(Debug, Deserialize, Serialize)]
struct JsonEntity {
    id: String,
    object_type: String,
    model: String,
    position: Vector3D,
    rotation: EulerAngles,
}

pub fn load_assets() -> Result<Vec<Mesh>, Box<dyn std::error::Error>>{
    //TODO Load level definition

    let map_json = Asset::get("maps/level1.json").expect("Failed to load /maps.level1.json");

    let obj_content = from_utf8(&map_json.data).expect("Invalid UTF-8 in maps/level1.json");

    //TODO: Load verts from obj
    // Deserialize JSON into Vec<JsonEntity>
    let entities: Vec<JsonEntity> = serde_json::from_str(&obj_content)?;

    // Convert JsonEntity to Mesh (verts will be empty for now)
    let meshes: Vec<Mesh> = entities
        .into_iter()
        .map(|entity| Mesh {
            verts: load_model_verts(&entity.model), // Load verts from model
            faces: load_model_faces(&entity.model),
            position: entity.position,
            rotation: entity.rotation,
        })
        .collect();

    Ok(meshes)
}

fn load_model_verts(model_name: &String) -> Vec<Point3D> {
    // Load model from assets
    let cube_obj = Asset::get(model_name).expect(&format!("Failed to load {}", model_name));

    let obj_content = from_utf8(&cube_obj.data).expect(&format!("Invalid UTF-8 in  {}", model_name));

    // Parse vertices (basic, assumes +Y Forward, +Z Up)
    let mut verts = Vec::new();

    for line in obj_content.lines() {
        if line.starts_with("v ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                verts.push(Point3D {
                    x: parts[1].parse().expect("Invalid x"),
                    y: parts[2].parse().expect("Invalid y"),
                    z: parts[3].parse().expect("Invalid z"),
                });
            }
        }
    }

    verts
}

fn load_model_faces(model_name: &String) -> Vec<Face> {
    // Load model from assets
    let cube_obj = Asset::get(model_name).expect(&format!("Failed to load {}", model_name));

    let obj_content = from_utf8(&cube_obj.data).expect(&format!("Invalid UTF-8 in  {}", model_name));

    // Parse vertices (basic, assumes +Y Forward, +Z Up)
    let mut faces = Vec::new();

    for line in obj_content.lines() {
        if line.starts_with("f ") {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let mut face = Face::new();

            for part in parts {

                if part.contains("f") {
                    continue;
                }

                let mut parts: Vec<&str> = part.split("//").collect();

                face.vert_indices.push(parts[0].parse().expect("Invalid vertex index"));
            }

            faces.push(face);
        }
    }

    faces
}

// TODO: Remove temporary frame counter
static mut rotation: f32 = 0.0;

impl ScreenRenderer for Level1Screen {
    fn render(&self, camera: &Camera, painter: &Painter) {
        // TODO: Remove temporary counter
        unsafe {
            rotation += 2.5;
        }

        //TODO: Calculate object positions
        
        //TODO: Render map
        
        //TODO: Render game objects

        let viewport_width = camera.viewport.width;
        let viewport_height = camera.viewport.height;
        let viewport_width_div_2 = &camera.viewport.width / 2.0;
        let viewport_height_div_2 = &camera.viewport.height / 2.0;
        
        for mesh in &self.meshes {

            //TODO: Render 3D Points to 2D via camera

            //TODO: Need to apply transforms before projecting

            let mut mut_mesh = mesh.clone();

            unsafe {
              //  mut_mesh.rotation.pitch += rotation.to_radians();
             //   mut_mesh.rotation.roll += rotation.to_radians();
                mut_mesh.rotation.yaw += rotation.to_radians();
            }

            let points_2d = camera.to_2d(&mut_mesh.get_transformed_verts());

            for face in &mut_mesh.faces {
                let mut verts = Vec::new();

                for vert_index in &face.vert_indices {
                    verts.push(points_2d.get(*vert_index - 1).unwrap());
                }

                let num_verts = verts.len();

                let mut face_outside_viewport = true;

                // Ignore faces outside view (doesn't really work well)...
                for i in 0..num_verts {
                    let first_vert = verts.get(i).unwrap();
                    let second_vert = if i == num_verts - 1 { verts.get(0).unwrap() } else {verts.get(i + 1).unwrap()} ;

                    let start: Pos2 = [first_vert.x + viewport_width_div_2, first_vert.y + viewport_height_div_2].into();
                    let end: Pos2 = [second_vert.x + viewport_width_div_2, second_vert.y + viewport_height_div_2].into();

                    if  (start.x >= 0.0 && start.x <= viewport_width) || (start.y >= 0.0 && start.y <= viewport_height) ||
                        (end.x >= 0.0 && end.x <= viewport_width) || (end.y >= 0.0 && end.y <= viewport_height) {
                        face_outside_viewport = false;
                    }
                }

                if face_outside_viewport{
                  //  continue;
                }

                // Render faces
                for i in 0..num_verts {
                    let first_vert = verts.get(i).unwrap();
                    let second_vert = if i == num_verts - 1 { verts.get(0).unwrap() } else {verts.get(i + 1).unwrap()} ;

                    let start: Pos2 = [first_vert.x, first_vert.y].into();
                    let end: Pos2 = [second_vert.x, second_vert.y].into();

                    painter.line_segment(
                        [start, end],
                        Stroke::new(2.0, Color32::GREEN),
                    );
                }
            }
        }
    }


}


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
    position: Point3D,
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

impl ScreenRenderer for Level1Screen {
    fn render(&self, camera: &Camera, painter: &Painter) {
        //TODO: Calculate object positions
        
        //TODO: Render map
        
        //TODO: Render game objects

        for mesh in &self.meshes {

            //TODO: Render 3D Points to 2D via camera

            //TODO: Need to apply transforms before projecting

            let points_2d = camera.to_2d(&mesh.get_transformed_verts());


            for (i, vert) in points_2d.iter().enumerate() {

                if i < points_2d.len() - 1 {
                    let current_vert = points_2d.get(i).unwrap();
                    let next_vert = points_2d.get(i + 1).unwrap(); // Returns Option<&Point3D>

                    let start: Pos2 = [(current_vert.x + 2.0) * 100.0, (current_vert.y + 2.0) * 100.0].into();
                    let end: Pos2 = [(next_vert.x + 2.0) * 100.0, (next_vert.y + 2.0) * 100.0].into();

                    painter.line_segment(
                        [start, end],
                        Stroke::new(2.0, Color32::GREEN),
                    );

                }


            }

        }
    }
}



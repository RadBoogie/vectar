use eframe::egui;
use eframe::egui::{Align2, Color32, FontId, Painter, Pos2};
use crate::player::camera::Camera;
//TODO: Hud stuff...
use crate::screens::traits::{HudRenderer};

pub struct TitleHud;

impl TitleHud {
    pub fn new() -> Self{
        Self
    }
}

/* The intent here is to call the render function to get the hud to draw itself on the screen */
impl HudRenderer for TitleHud {
    fn render(&self, camera: &Camera, painter: &Painter) {
        // Draw text at a specific position
        painter.text(
            Pos2::new(10.0, 10.0),
            Align2::LEFT_TOP,
            format!("Pos: {:?}", &camera.position),
            FontId::new(16.0, egui::FontFamily::Proportional),
            Color32::WHITE,
        );

        painter.text(
            Pos2::new(10.0, 30.0),
            Align2::LEFT_TOP,
            format!("Pitch:{} Roll:{} Yaw:{}", &camera.rotation.pitch.to_degrees(), &camera.rotation.roll.to_degrees(), &camera.rotation.yaw.to_degrees()),
            FontId::new(16.0, egui::FontFamily::Proportional),
            Color32::WHITE,
        );
    }
}


pub struct GameHud;

impl GameHud {
    pub fn new() -> Self{
        Self
    }
}

impl HudRenderer for GameHud {
    fn render(&self, camera: &Camera, painter: &Painter) {
        // Draw text at a specific position
        painter.text(
            Pos2::new(10.0, 10.0),
            Align2::LEFT_TOP,
            format!("Pos: {:?}", &camera.position),
            FontId::new(24.0, egui::FontFamily::Proportional),
            Color32::WHITE,
        );
    }
}
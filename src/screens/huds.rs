use eframe::egui;
use eframe::egui::{Align2, Color32, FontId, Painter, Pos2};

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
    fn render(&self, painter: &Painter) {
        // Draw text at a specific position
        painter.text(
            Pos2::new(500.0, 500.0),
            Align2::LEFT_TOP,      
            "Title HUD",  
            FontId::new(24.0, egui::FontFamily::Monospace),
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
    fn render(&self, painter: &Painter) {
        // Draw text at a specific position
        painter.text(
            Pos2::new(500.0, 500.0),
            Align2::LEFT_TOP,
            "Game HUD",
            FontId::new(24.0, egui::FontFamily::Proportional),
            Color32::WHITE,
        );
    }
}
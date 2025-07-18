//TODO: Code for the title screen
use eframe::egui;
use eframe::egui::{Align2, Color32, FontId, Painter, Pos2};
use crate::player::camera::Camera;
use crate::screens::traits::ScreenRenderer;

pub struct TitleScreen;

impl TitleScreen {
    pub fn new() -> Self {
        Self
    }
}

impl ScreenRenderer for TitleScreen {
    fn render(&self, camera: &Camera, painter: &Painter) {
        //TODO: 
    }
}





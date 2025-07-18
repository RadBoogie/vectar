use eframe::egui::Painter;
use crate::player::camera::Camera;

pub trait HudRenderer {
    fn render(&self, painter: &Painter);
}

pub trait ScreenRenderer {
    fn render(&self, camera: &Camera, painter: &Painter);
}
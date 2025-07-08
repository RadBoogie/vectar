use eframe::egui::Painter;

pub trait HudRenderer {
    fn render(&self, painter: &Painter);
}

pub trait ScreenRenderer {
    fn render(&self, painter: &Painter);
}
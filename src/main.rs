mod screens;

use eframe::{egui};
use eframe::epaint::StrokeKind;
use egui::{Color32, Pos2, Rect, Stroke, Vec2};
use crate::screens::{huds, title_screen};
use crate::screens::traits::{HudRenderer, ScreenRenderer};


struct Game {
    hud: Box<dyn HudRenderer>,
    current_screen:  Box<dyn ScreenRenderer>,
    start_point: Pos2,
    end_point: Pos2,
}

impl Game {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            hud: Box::new(huds::TitleHud::new()),
            current_screen: Box::new(title_screen::TitleScreen::new()),
            start_point: Pos2::new(100.0, 100.0),
            end_point: Pos2::new(200.0, 200.0),
        }
    }
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // Create a central panel that fills the window
        egui::CentralPanel::default().show(ctx, |ui| {
            // Define a canvas area (e.g., a rectangle for drawing)
            let canvas_size = Vec2::new(400.0, 300.0);
            let canvas_rect = Rect::from_min_size(ui.min_rect().min, canvas_size);
            
            // Get the painter for custom drawing
            let painter = ui.painter();
            
            demo_stuff(painter, ui, canvas_rect, self); // Delete me

            // Screen is rendered first
            self.current_screen.render(painter);
            
            // HUD is last
            self.hud.render(painter);
        });
    }
    
    
    
}

fn demo_stuff(painter: &egui::Painter, ui: &egui::Ui, canvas_rect: Rect, game: &mut Game) {
    // Draw a white background for the canvas
    painter.rect_filled(canvas_rect, 0.0, Color32::WHITE);

    // Draw a border around the canvas
    painter.rect_stroke(canvas_rect, 0.0, Stroke::new(1.0, Color32::BLACK), StrokeKind::Middle);

    // Example: Draw a line
    painter.line_segment(
        [game.start_point, game.end_point],
        Stroke::new(2.0, Color32::RED),
    );

    // Example: Draw a rectangle
    let rect = Rect::from_min_max(
        Pos2::new(150.0, 150.0),
        Pos2::new(250.0, 200.0),
    );
    
    painter.rect_filled(rect, 0.0, Color32::BLUE);

    // Example: Handle mouse input to update line endpoints
    if let Some(pos) = ui.input(|i| i.pointer.latest_pos()) {
        if ui.input(|i| i.pointer.primary_down()) {
            game.start_point = pos;
        }
        if ui.input(|i| i.pointer.secondary_down()) {
            game.end_point = pos;
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    // Set up the native window options
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]), // Set the window size
        ..Default::default()
    };

    // Run the egui application
    eframe::run_native(
        "Vectar",
        native_options,
        Box::new(|cc| Ok(Box::new(Game::new(cc)))),
    )
}

mod screens;
mod player;
mod types;
mod objects;

use eframe::{egui};
use eframe::epaint::StrokeKind;
use egui::{Color32, Pos2, Rect, Stroke, Vec2};
use crate::screens::{huds, title_screen, level1_screen};
use crate::screens::traits::{HudRenderer, ScreenRenderer};
use crate::player::camera::Camera;
use crate::types::geometry::*;

const SCREEN_WIDTH: f32 = 1024.0;
const SCREEN_HEIGHT: f32 = 768.0;

struct Game {
    hud: Box<dyn HudRenderer>,
    current_screen:  Box<dyn ScreenRenderer>,
    camera: Camera,

    start_point: Pos2,
    end_point: Pos2,
}

impl Game {
    fn new(_cc: &eframe::CreationContext<'_>) -> Game {
        let camera = player::camera::Camera::new(
            types::geometry::Point3D { x: 0.0, y: 0.0, z: -5.0 },
            EulerAngles { pitch: 0.0, yaw: 0.0, roll: 0.0 },
            90.0,
            Rectangle { width: SCREEN_WIDTH, height: SCREEN_HEIGHT },
            1000.0,
        );

        Game {
            hud: Box::new(huds::TitleHud::new()),
            current_screen: Box::new(level1_screen::Level1Screen::new()),
            camera,
            start_point: Pos2::new(100.0, 100.0),
            end_point: Pos2::new(200.0, 200.0),
        }
    }
}

impl eframe::App for Game {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request a repaint every frame for continuous updates
        ctx.request_repaint();

        // Get the current window size
        let current_window_size = ctx.input(|i| i.screen_rect.size());

        self.camera.update_viewport_size(Rectangle{width: current_window_size.x, height: current_window_size.y});


        ctx.input(|input| {
            if let Some(pos) = input.pointer.latest_pos() {
                // Use pos.x, pos.y for absolute mouse position
            }
            let mouse_delta = input.pointer.delta();
            if mouse_delta != egui::Vec2::ZERO && input.pointer.primary_down() {
                // Use mouse_delta.x, mouse_delta.y to adjust camera (e.g., yaw/pitch)
                self.camera.rotate(-mouse_delta.x / 5.0, mouse_delta.y / 5.0);
            }
            if input.pointer.button_clicked(egui::PointerButton::Primary) {
                // Handle left-click (e.g., shoot or select)
                println!("Click");
            }

            if input.key_down(egui::Key::W) {
                // Move camera forward
                self.camera.move_forward(-0.1);
            }

            if input.key_down(egui::Key::A) {
                self.camera.move_strafe(-0.1);
            }

            if input.key_down(egui::Key::S) {
                self.camera.move_forward(0.1);
            }

            if input.key_down(egui::Key::D) {
                self.camera.move_strafe(0.1);
            }
            
            
        });


        // Create a central panel that fills the window
        egui::CentralPanel::default().show(ctx, |ui| {
            // Use the current window size for the canvas
          //  let canvas_size = current_window_size; // Or adjust based on your needs
          //  let canvas_rect = Rect::from_min_size(ui.min_rect().min, canvas_size);

            // Get the painter for custom drawing
            let painter = ui.painter();
            
         //   demo_stuff(painter, ui, canvas_rect, self); // Delete me

            //TODO: Move player (camera)

            // Screen is rendered first
            self.current_screen.render(&self.camera, painter);
            
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
            .with_inner_size([SCREEN_WIDTH, SCREEN_HEIGHT]), // Set the window size
        ..Default::default()
    };

    // Run the egui application
    eframe::run_native(
        "Vectar",
        native_options,
        Box::new(|cc| Ok(Box::new(Game::new(cc)))),
    )
}

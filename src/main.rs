mod screens;
mod player;
mod types;
mod objects;
mod utils;

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
}

impl Game {
    fn new(_cc: &eframe::CreationContext<'_>) -> Game {
        let camera = player::camera::Camera::new(
            types::geometry::Point3D { x: 25.0, y: 0.0, z: 0.0 },
            EulerAngles { pitch: 0.0, yaw: 90.0_f32.to_radians(), roll: 0.0 },
            Vector3D{x: -1.0, y: 0.0, z: 0.0},
            90.0,
            Rectangle { width: SCREEN_WIDTH, height: SCREEN_HEIGHT },
            1000.0,
        );

        Game {
            hud: Box::new(huds::TitleHud::new()),
            current_screen: Box::new(level1_screen::Level1Screen::new()),
            camera,
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
                self.camera.rotate(mouse_delta.x / 5.0, -mouse_delta.y / 5.0);
            }
            if input.pointer.button_clicked(egui::PointerButton::Primary) {
                // Handle left-click (e.g., shoot or select)
                println!("Click");
            }

            if input.key_down(egui::Key::W) {
                // Move camera forward
                self.camera.move_forward(0.1);
            }

            if input.key_down(egui::Key::A) {
                self.camera.move_strafe(0.1);
            }

            if input.key_down(egui::Key::S) {
                self.camera.move_forward(-0.1);
            }

            if input.key_down(egui::Key::D) {
                self.camera.move_strafe(-0.1);
            }
            
            
        });


        // Create a central panel that fills the window
        egui::CentralPanel::default().show(ctx, |ui| {
            // Use the current window size for the canvas
          //  let canvas_size = current_window_size; // Or adjust based on your needs
          //  let canvas_rect = Rect::from_min_size(ui.min_rect().min, canvas_size);

            // Get the painter for custom drawing
            let painter = ui.painter();

            //TODO: Move player (camera)

            // Screen is rendered first
            self.current_screen.render(&self.camera, painter);
            
            // HUD is last
            self.hud.render(&self.camera, painter);
        });
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

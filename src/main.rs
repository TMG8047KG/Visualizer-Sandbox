use std::f32::consts::PI;

use eframe::egui;
use egui::Pos2;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Viualizer Sandbox",
        native_options,
        Box::new(|cc| Ok(Box::new(VisualizerApp::new(cc)))),
    );
}

#[derive(Default)]
struct VisualizerApp {
    amplitude: f32,
    frequency: f32,
    phase: f32,
    period: i32, //time
    color: egui::Color32,
}

impl VisualizerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            amplitude: 20.0,
            frequency: 30.0,
            phase: 2.0,
            period: 2000,
            color: egui::Color32::RED,
        }
    }
}

impl eframe::App for VisualizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::Panel::left("controls").show_inside(ui, |ui| {
            ui.add(
                egui::Slider::new(&mut self.amplitude, 0.0..=1000.0)
                    .text("Amplitude")
                    .step_by(1.0),
            );
            ui.add(
                egui::Slider::new(&mut self.frequency, 0.0..=1000.0)
                    .text("Frequency")
                    .step_by(1.0),
            );
            ui.add(
                egui::Slider::new(&mut self.phase, 0.0..=2.0)
                    .text("Phase")
                    .step_by(0.05),
            );
            ui.add(
                egui::Slider::new(&mut self.period, 1..=2000)
                    .text("Period")
                    .step_by(1.0),
            );
            ui.color_edit_button_srgba(&mut self.color);
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let rect = ui.available_rect_before_wrap();
            let painter = ui.painter();

            let width = rect.width();
            let height = rect.height();
            let center_y = height / 2.0;

            let mut points = Vec::new();

            for i in 0..self.period {
                let t = i as f32 / (self.period - 1) as f32;

                let x = t * width * 2.0;

                let angle = t * self.frequency * self.phase * PI;
                let y = center_y + (angle.sin() * self.amplitude);
                points.push(Pos2 { x, y });
            }

            painter.add(egui::Shape::line(
                points,
                egui::Stroke::new(2.0, self.color),
            ));
        });
    }
}

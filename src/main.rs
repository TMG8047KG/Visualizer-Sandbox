use std::vec;

use eframe::{
    egui,
    wgpu::{Color, naga::Range, wgc::id},
};
use egui::{Color32, Pos2, Stroke};
use egui_plot::{Line, Plot, PlotItem, PlotPoint, PlotPoints, Points};

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
    x: f32,
    y: f32,
    t: i32,
}

impl VisualizerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for VisualizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::Panel::left("controls").show_inside(ui, |ui| {
            ui.add(egui::Slider::new(&mut self.x, 0.0..=1000.0).text("X offset"));
            ui.add(egui::Slider::new(&mut self.y, 0.0..=1000.0).text("Amplitude"));
            ui.add(egui::Slider::new(&mut self.t, 10..=2000).text("Points"));
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let rect = ui.available_rect_before_wrap();
            let painter = ui.painter();

            let width = rect.width();
            let height = rect.height();

            let mut points = Vec::new();

            for i in 0..self.t {
                let t_norm = i as f32 / (self.t - 1) as f32; // 0 → 1

                // X goes across full width
                let x_screen = rect.left() + t_norm * width;
                //let y_screen = height/2;

                // Your sine function (data space)
                let x_data = t_norm * std::f32::consts::TAU * 2.0; // 2 periods
                let y_data = x_data.sin();

                // Map [-1, 1] → [0, 1]
                let y_norm = (y_data + 1.0) * 0.5;

                // Flip Y (because screen Y grows downward)
                let y_screen = rect.bottom() - y_norm * height / 2.0;

                points.push(egui::Pos2 {
                    x: x_screen + self.x,           // optional offset
                    y: y_screen + (self.y - 500.0), // amplitude-ish offset
                });
            }

            painter.add(egui::Shape::line(
                points,
                egui::Stroke::new(2.0, egui::Color32::RED),
            ));
        });
    }
}

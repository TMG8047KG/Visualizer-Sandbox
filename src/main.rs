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
struct VisualizerApp {}

impl VisualizerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for VisualizerApp {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let mut points: Vec<Pos2> = vec![];
            for i in 0..10000 {
                let x = i as f32 * 1.0;
                points.push(Pos2 {
                    x: x,
                    y: x.sin() + 300.0,
                });
            }
            ui.painter()
                .line(points, egui::Stroke::new(2.0, egui::Color32::RED));
            //ui.content_rect()
        });
    }
}

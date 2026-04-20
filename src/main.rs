use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Viualizer Sandbox", native_options, Box::new(|cc| Ok(Box::new(VisualizerApp::new(cc)))));
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
            ui.heading("Hello world!");
        });
    }
}

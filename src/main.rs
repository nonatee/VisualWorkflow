fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(EGuiApp::new(cc)))));
}

#[derive(Default)]
struct EGuiApp {
    position: Pos2,
}

impl EGuiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            position: Pos2::new(400.0, 400.0),
        }
    }
}
use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2};
impl eframe::App for EGuiApp {
    
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
   egui::CentralPanel::default().show(ctx, |ui| {
    let rect = ui.allocate_rect(
        egui::Rect::from_min_size(self.position, egui::vec2(100.0, 80.0)),
        egui::Sense::drag(),
    );
    ui.painter().rect(rect.rect, 10.0, egui::Color32::LIGHT_BLUE, egui::Stroke::new(2.0, egui::Color32::BLACK), egui::StrokeKind::Middle);

    if rect.dragged() {
        println!("Rectangle clicked!");
        self.position += rect.drag_delta();
    }
});
   }
}
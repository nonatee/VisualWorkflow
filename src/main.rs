fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(EGuiApp::new(cc)))));
}

#[derive(Default)]
struct EGuiApp {
    rect_pos: Pos2,
    cursor_pos: Pos2,
    point1: Pos2,
    point2: Pos2,
    drawing: bool,
}

impl EGuiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            rect_pos: Pos2::new(400.0, 400.0),
            cursor_pos: Pos2::new(400.0, 400.0),
            point1: Pos2::new(0.0, 0.0),
            point2: Pos2::new(0.0,0.0),
            drawing: false,
        }
    }
}
use egui::{pos2, Color32, Pos2, Rect, Sense, Stroke, Vec2};
impl eframe::App for EGuiApp {
    
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let rect = ui.allocate_rect(
            egui::Rect::from_min_size(self.rect_pos, egui::vec2(100.0, 80.0)),
            egui::Sense::click_and_drag(),
    );
    ui.painter().rect(rect.rect, 10.0, egui::Color32::LIGHT_BLUE, egui::Stroke::new(2.0, egui::Color32::BLACK), egui::StrokeKind::Middle);

    if rect.dragged_by(egui::PointerButton::Primary) {
        self.rect_pos += rect.drag_delta();
        ctx.request_repaint();
    }
    if rect.dragged_by(egui::PointerButton::Secondary) {
        if self.drawing {
            self.point2 = ctx.input(|i| i.pointer.hover_pos()).unwrap();
        }
        else {
            self.point1 = ctx.input(|i| i.pointer.hover_pos()).unwrap();
        }
        self.drawing = true
    }
    else {
        self.drawing = false;
    }
    ui.painter().line_segment([self.point1,self.point2], Stroke::new(5.0,Color32::RED));
});
   }
}
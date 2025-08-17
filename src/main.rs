mod node_rect;
mod connector;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(EGuiApp::new(cc)))),
    );
}

#[derive(Default)]
struct EGuiApp  {
    cursor_pos: Pos2,
    drawing: bool,
    rects: Vec<NodeRect>,
}

impl  EGuiApp  {
    fn new( cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            cursor_pos: Pos2::new(400.0, 400.0),
            drawing: false,
            rects: Vec::new(),
        };
        let rect1 = NodeRect::new(Pos2::new(400.0, 400.0),Vec2::new(100.0, 100.0));
        let rect2 = NodeRect::new(Pos2::new(0.0, 0.0),Vec2::new(100.0, 100.0));
        app.rects.push(rect1);
        app.rects.push(rect2);
        app
    }
}
use egui::{Color32, Pos2, Stroke, Vec2};

use crate::node_rect::NodeRect;
impl eframe::App for EGuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        let rects_clone = self.rects.clone();
        egui::CentralPanel::default().show(ctx, |ui| {
            
            for node in &mut self.rects {
                node.update_this(ctx, ui);
                node.check_new_connector(ctx, &rects_clone);
                for connector in &node.connectors {
                    if connector.point2.is_some() {
                        ui.painter().line_segment([connector.point1.unwrap(), connector.point2.unwrap()], Stroke::new(5.0, Color32::RED));
                    }
                } 
            }
        });
    }
}

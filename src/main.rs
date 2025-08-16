mod node_rect;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(EGuiApp::new(cc)))),
    );
}

#[derive(Default)]
struct EGuiApp {
    rect_pos: Pos2,
    cursor_pos: Pos2,
    point1: Pos2,
    point2: Pos2,
    drawing: bool,
    rects: Vec<NodeRect>,
}

impl EGuiApp {
    fn new( cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            rect_pos: Pos2::new(400.0, 400.0),
            cursor_pos: Pos2::new(400.0, 400.0),
            point1: Pos2::new(0.0, 0.0),
            point2: Pos2::new(0.0, 0.0),
            drawing: false,
            rects: Vec::new(),
        };
        let mut Rect1 = NodeRect::new(Pos2::new(400.0, 400.0),Vec2::new(100.0, 100.0));
        let mut Rect2 = NodeRect::new(Pos2::new(0.0, 0.0),Vec2::new(100.0, 100.0));
        app.rects.push(Rect1);
        app.rects.push(Rect2);
        app
    }
}
use egui::{accesskit::Node, pos2, Color32, Pos2, Rect, Sense, Stroke, Vec2};

use crate::node_rect::NodeRect;
impl eframe::App for EGuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        
        egui::CentralPanel::default().show(ctx, |ui| {
            for node in &mut self.rects {
                node.update_this(ctx, ui);
            }
            /* 
            if rect.dragged_by(egui::PointerButton::Secondary) {
                if self.drawing {
                    self.point2 = ctx.input(|i| i.pointer.hover_pos()).unwrap();
                } else {
                    self.point1 = ctx.input(|i| i.pointer.hover_pos()).unwrap();
                }
                self.drawing = true
            } else {
                self.drawing = false;
            }
            ui.painter().line_segment([self.point1, self.point2], Stroke::new(5.0, Color32::RED));*/
        });
    }
}

mod node_rect;
mod connector;
mod start_button;
mod button_struct;

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
    rects: Vec<NodeRect>,
    buttons: Vec<Box<dyn ButtonStruct>>
}

impl  EGuiApp  {
    fn new( cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            rects: Vec::new(),
            buttons: Vec::new(),
        };
        
        let rect1 = NodeRect::new(Pos2::new(400.0, 400.0),Vec2::new(100.0, 100.0));
        let rect2 = NodeRect::new(Pos2::new(0.0, 0.0),Vec2::new(100.0, 100.0));
        app.rects.push(rect1);
        app.rects.push(rect2);
        let start_button = StartButton::new("Hello".to_string(), Rect::from_min_size(Pos2::new(100.0,100.0),Vec2::new(100.0, 100.0)));
        app.buttons.push(Box::new(start_button));
        app
    }
}
use egui::{Rect, Color32, Pos2, Stroke, Vec2};

use crate::{button_struct::ButtonStruct, node_rect::NodeRect, start_button::StartButton};
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
            
            for button in &mut self.buttons {
                button.init_button(ui);
                button.check_pressed();
            }

        });
        
    }
}

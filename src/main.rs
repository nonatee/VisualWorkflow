mod node_rect;
mod connector;
mod start_button;
mod button_struct;
mod node_trait;
mod start_node;
mod text_node;
mod jumble_node;

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
    rects: Vec<Box<dyn NodeTrait>>,
    buttons: Vec<Box<dyn ButtonStruct>>
}

impl  EGuiApp  {
    fn new( cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            rects: Vec::new(),
            buttons: Vec::new(),
        };
        
        let rect1 = NodeRect::new(Pos2::new(400.0, 400.0),Vec2::new(100.0, 100.0), 0, Color32::GREEN);
        let start_rect1 = StartNode{node_rect:rect1};
        let rect2 = NodeRect::new(Pos2::new(0.0, 0.0),Vec2::new(100.0, 100.0), 1,Color32::WHITE);
        let text_rect1 = TextNode{node_rect:rect2, text: "todo!()".to_string() };
        let rect3 = NodeRect::new(Pos2::new(200.0, 200.0),Vec2::new(100.0, 100.0), 2, Color32::BLUE);
        let jumble_rect1 = JumbleNode{node_rect:rect3};
        app.rects.push(Box::new(start_rect1));
        app.rects.push(Box::new(text_rect1));
        app.rects.push(Box::new(jumble_rect1));
        let start_button = StartButton::new("Hello".to_string(), Rect::from_min_size(Pos2::new(100.0,100.0),Vec2::new(100.0, 100.0)));
        app.buttons.push(Box::new(start_button));
        app
    }
}
use egui::{Rect, Color32, Pos2, Stroke, Vec2};

use crate::{button_struct::ButtonStruct, jumble_node::JumbleNode, node_rect::NodeRect, node_trait::NodeTrait, start_button::StartButton, start_node::StartNode, text_node::TextNode};
impl eframe::App for EGuiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        let mut vec_rects = Vec::new();
        for node in &self.rects {
            vec_rects.push(Rect::from_min_size(node.get_rect().position, node.get_rect().size));
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            for node in &mut self.rects {
                node.update_this(ui, &vec_rects);
                for connector in &node.get_rect().connectors {
                    if connector.point2.is_some() {
                        ui.painter().line_segment([connector.point1.unwrap(), connector.point2.unwrap()], Stroke::new(5.0, Color32::RED));
                    }
                } 
            }
            for button in &mut self.buttons {
                button.init_button(ui);
                button.check_pressed(&self.rects);
            }
            

        });
        
    }
}

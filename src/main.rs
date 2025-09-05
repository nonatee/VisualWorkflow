mod button_struct;
mod connector;
mod jumble_node;
mod node_rect;
mod node_trait;
mod start_button;
mod start_node;
mod text_node;
mod uppercase_node;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(EGuiApp::new(cc)))),
    );
}

#[derive(Default)]
struct EGuiApp {
    rects: Vec<Box<dyn NodeTrait>>,
    buttons: Vec<Box<dyn ButtonStruct>>,
    connectors: Vec<Connector>,
}

impl EGuiApp {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            rects: Vec::new(),
            buttons: Vec::new(),
            connectors: Vec::new(),
        };

        let rect1 = NodeRect::new(
            Pos2::new(400.0, 400.0),
            Vec2::new(100.0, 100.0),
            0,
            Color32::GREEN,
        );
        let start_rect1 = StartNode { node_rect: rect1 };
        let rect2 = NodeRect::new(
            Pos2::new(0.0, 0.0),
            Vec2::new(100.0, 100.0),
            1,
            Color32::WHITE,
        );
        let text_rect1 = TextNode {
            node_rect: rect2,
            text: "todo!()".to_string(),
        };
        let rect3 = NodeRect::new(
            Pos2::new(200.0, 200.0),
            Vec2::new(100.0, 100.0),
            2,
            Color32::BLUE,
        );
        let jumble_rect1 = JumbleNode { node_rect: rect3 };
        let rect4 = NodeRect::new(
            Pos2::new(600.0, 600.0),
            Vec2::new(100.0, 100.0),
            3,
            Color32::YELLOW,
        );
        let uppercase_rect1 = UppercaseNode {node_rect:rect4};
        app.rects.push(Box::new(start_rect1));
        app.rects.push(Box::new(text_rect1));
        app.rects.push(Box::new(jumble_rect1));
        app.rects.push(Box::new(uppercase_rect1));
        let start_button = StartButton::new(
            "Hello".to_string(),
            Rect::from_min_size(Pos2::new(100.0, 100.0), Vec2::new(100.0, 100.0)),
        );
        app.buttons.push(Box::new(start_button));
        app
    }
}
use egui::{Color32, Pos2, Rect, Stroke, Vec2};

use crate::{
    button_struct::ButtonStruct, connector::Connector, jumble_node::JumbleNode, node_rect::NodeRect, node_trait::NodeTrait, start_button::StartButton, start_node::StartNode, text_node::TextNode, uppercase_node::UppercaseNode
};
impl eframe::App for EGuiApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut vec_rects = Vec::new();
        for node in &self.rects {
            vec_rects.push(Rect::from_min_size(
                node.get_rect().position,
                node.get_rect().size,
            ));
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            for node in &mut self.rects {
                node.update_this(ui, &vec_rects,&mut self.connectors);
            }
            for connector in &self.connectors {
                    if connector.point2.is_some() {
                        ui.painter().line_segment(
                            [connector.point1.unwrap(), connector.point2.unwrap()],
                            Stroke::new(5.0, Color32::RED),
                        );
                    }
                }
            for button in &mut self.buttons {
                button.init_button(ui);
                button.check_pressed(&self.rects,&mut self.connectors);
            }
        });
    }
}

use crate::{connector::Connector, EGuiApp};
use egui::{Pos2, Response, Vec2};
#[derive(Clone)]
pub struct NodeRect {
    position: Pos2,
    size: Vec2,
    pub response: Option<Response>,
    connecting: bool,
    pub connectors: Vec<Connector>,
}
impl NodeRect {
    pub fn new(position: Pos2, size: Vec2) -> Self {
        Self {
            position: position,
            size: size,
            response: None,
            connecting: false,
            connectors: Vec::new(),
        }
    }
    fn assign_rect(&mut self, ui: &mut egui::Ui) {
        self.response = Some(ui.allocate_rect(
            egui::Rect::from_min_size(self.position, self.size),
            egui::Sense::click_and_drag(),
        ));
    }
    fn paint_rect(&self, ui: &mut egui::Ui) {
        ui.painter().rect(
            self.response.as_ref().unwrap().rect,
            10.0,
            egui::Color32::LIGHT_BLUE,
            egui::Stroke::new(2.0, egui::Color32::BLACK),
            egui::StrokeKind::Middle,
        );
    }
    fn handle_drag(&mut self) {
        let response = self.response.as_ref().unwrap();
        if response.dragged_by(egui::PointerButton::Primary) {
            self.position += response.drag_delta();
            for connector in &mut self.connectors {
                connector.point1 = Some((response.drag_delta() + connector.point1.unwrap().to_vec2()).to_pos2());
            }
        }
    }
    pub fn check_new_connector(&mut self, ctx: &egui::Context,  rects: &Vec<NodeRect>) {
        if self.response.as_ref().unwrap().dragged_by(egui::PointerButton::Secondary) {
            if !self.connecting {
                let mut pos = None;
                ctx.input(|i| {
                    pos = i.pointer.hover_pos();
                });
                let new_connector = Some(Connector::new(pos.unwrap()));
                self.connectors.push(new_connector.unwrap());
            }
            if self.connecting {
                let length = self.connectors.len()-1;
                self.connectors[length].point2 = ctx.input(|i| i.pointer.hover_pos());
            }  
            self.connecting = true;
        }
        else {
            if self.connecting {
                let length = self.connectors.len()-1;
                for (index, node_rect) in rects.iter().enumerate() {
                    if node_rect.response.as_ref().unwrap().rect.contains(ctx.input(|i| i.pointer.hover_pos()).unwrap()) {
                        self.connectors[length].connected_node = Some(index);
                        println!("{}", self.connectors[length].connected_node.unwrap());
                    }
                }
            }
            self.connecting = false;
        }
    }
    pub fn update_this(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        self.assign_rect(ui);
        self.paint_rect(ui);
        self.handle_drag();
    }
}

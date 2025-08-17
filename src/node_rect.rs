use crate::connector::Connector;
use egui::{Pos2, Response, Vec2};

pub struct NodeRect {
    position: Pos2,
    size: Vec2,
    response: Option<Response>,
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
    fn paint_rect(&self, ui: &mut egui::Ui, response: &egui::Response) {
        ui.painter().rect(
            response.rect,
            10.0,
            egui::Color32::LIGHT_BLUE,
            egui::Stroke::new(2.0, egui::Color32::BLACK),
            egui::StrokeKind::Middle,
        );
    }
    fn handle_drag(&mut self, ctx: &egui::Context, response: &egui::Response) {
        if response.dragged_by(egui::PointerButton::Primary) {
            self.position += response.drag_delta();
        }
    }
    fn check_new_connector(&mut self, ctx: &egui::Context, response: &egui::Response) {
        if response.dragged_by(egui::PointerButton::Secondary) {
            if !self.connecting {
                let mut pos = None;
                ctx.input(|i| {
                    pos = i.pointer.hover_pos();
                });
                let new_connector = Some(Connector::new(pos.unwrap()));
                self.connectors.push(new_connector.unwrap());
            }
            if self.connecting {
                self.connectors[0].point2 = ctx.input(|i| i.pointer.hover_pos());
            }
            self.connecting = true;
        }
    }
    pub fn update_this(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let response = ui.allocate_rect(
            egui::Rect::from_min_size(self.position, self.size),
            egui::Sense::click_and_drag(),
        );
        self.paint_rect(ui, &response);
        self.check_new_connector(ctx, &response);
        self.handle_drag(ctx, &response);
    }
}

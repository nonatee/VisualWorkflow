use crate::{connector::{self, Connector}, node_trait::NodeTrait, EGuiApp};
use egui::{Pos2, Rect, Response, Vec2};
#[derive(Clone)]
pub struct NodeRect {
    pub position: Pos2,
    pub size: Vec2,
    pub response: Option<Response>,
    connecting: bool,
    pub connectors: Vec<Connector>,
    pub index: usize,
}
impl NodeRect {
    pub fn new(position: Pos2, size: Vec2, index:usize) -> Self {
        Self {
            position: position,
            size: size,
            response: None,
            connecting: false,
            connectors: Vec::new(),
            index: index
        }
    }
    pub fn assign_rect(&mut self, ui: &mut egui::Ui) {
        self.response = Some(ui.allocate_rect(
            egui::Rect::from_min_size(self.position, self.size),
            egui::Sense::click_and_drag(),
        ));
    }
    pub fn paint_rect(&self, ui: &mut egui::Ui) {
        ui.painter().rect(
            self.response.as_ref().unwrap().rect,
            10.0,
            egui::Color32::LIGHT_BLUE,
            egui::Stroke::new(2.0, egui::Color32::BLACK),
            egui::StrokeKind::Middle,
        );
    }
    pub fn handle_drag(&mut self) {
        let response = self.response.as_ref().unwrap();
        if response.dragged_by(egui::PointerButton::Primary) {
            self.position += response.drag_delta();
            for connector in &mut self.connectors {
                connector.point1 = Some((response.drag_delta() + connector.point1.unwrap().to_vec2()).to_pos2());
            }
        }
    }
    pub fn check_new_connector(&mut self, ui: &mut egui::Ui,  rects: &Vec<Rect>) {
        let ctx = ui.ctx();
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
                for (index, rect) in rects.iter().enumerate() {
                    if rect.contains(ctx.input(|i| i.pointer.hover_pos()).unwrap()) {
                        self.connectors[length].connected_node = Some(index);
                        println!("{}", self.connectors[length].connected_node.unwrap());
                    }
                }
            }
            self.connecting = false;
        }
    }
    pub fn progress_node(&self, mut args: Option<Vec<String>>, rects: &mut Vec<Box<dyn NodeTrait>>) {
        if self.connectors.len() > 0 {
            if args.is_some() {
                args.as_mut().unwrap().push("hello from one of the nodes".to_string());
            }
            else {
                args = Some(Vec::new());
                args.as_mut().unwrap().push("started it!".to_string());
            }
            for connector in &self.connectors {
                if connector.connected_node.is_some() {
                    if rects[connector.connected_node.unwrap()].get_rect().index != self.index  {
                        rects[connector.connected_node.unwrap()].progress_node(args.clone(), rects);
                    }
                }
            }
        }
        else {
            if args.is_some() {
                args.as_mut().unwrap().push("ending!".to_string());
            }
            else {
                args = Some(Vec::new());
                args.as_mut().unwrap().push("starting and ending it!".to_string());
            }
            for x in args.unwrap() {
                println!("{}", x);
            }
        }
    }
    pub fn update_this(&mut self, ui: &mut egui::Ui,  rects: &mut Vec<Rect>) {
        self.assign_rect(ui);
        self.paint_rect(ui);
        self.handle_drag();
        self.check_new_connector(ui, rects);
    }
}

use egui::Rect;

use crate::{connector::Connector, node_rect::NodeRect};

pub trait NodeTrait {
    fn progress_node(&self, args: Option<Vec<String>>, rects: &Vec<Box<dyn NodeTrait>>, connectors: &Vec<Connector>);
    fn update_this(&mut self, ui: &mut egui::Ui,rects:&Vec<Rect>, connectors: &mut Vec<Connector>) {
        let node_rect = self.get_mut_rect();
        node_rect.assign_rect(ui);
        node_rect.paint_rect(ui);
        node_rect.handle_drag(connectors);
        node_rect.check_new_connector(ui, rects,connectors);
    }
    fn get_mut_rect(&mut self)-> &mut NodeRect;
    fn get_rect(&self)-> &NodeRect;
}
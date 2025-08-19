use egui::Rect;

use crate::node_rect::NodeRect;

pub trait NodeTrait {
    fn progress_node(&self, args: Option<Vec<String>>, rects: &Vec<Box<dyn NodeTrait>>);
    fn update_this(&mut self, ui: &mut egui::Ui,rects:&Vec<Rect>) {
        let node_rect = self.get_mut_rect();
        node_rect.assign_rect(ui);
        node_rect.paint_rect(ui);
        node_rect.handle_drag();
        node_rect.check_new_connector(ui, rects);
    }
    fn get_mut_rect(&mut self)-> &mut NodeRect;
    fn get_rect(&self)-> &NodeRect;
}
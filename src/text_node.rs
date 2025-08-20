use egui::Rect;

use crate::{connector::Connector, node_rect::NodeRect, node_trait::NodeTrait};

pub struct TextNode {
    pub node_rect: NodeRect,
    pub text: String
}
impl NodeTrait for TextNode {
    fn progress_node(&self, mut args: Option<Vec<String>>, rects: &Vec<Box<dyn NodeTrait>>, connectors: &Vec<Connector>) {
        args.as_mut().unwrap().push(self.text.clone());
        let mut self_connectors: Vec<&Connector> = Vec::new();
            for index in self.node_rect.connectors.clone() {
                self_connectors.push(&connectors[index]);
            }
        println!("{}", self.node_rect.connectors.len());
        if self.node_rect.connectors.len() > 0 {
            for connector in self_connectors {
                if connector.connected_node.is_some() {
                    if rects[connector.connected_node.unwrap()].get_rect().index != self.node_rect.index  {
                        rects[connector.connected_node.unwrap()].progress_node(args.clone(), rects,connectors);
                    }
                }
            }
        }
        else {
            for x in args.unwrap() {
                println!("{}", x);
            }
        }
    }
    fn get_mut_rect(&mut self)-> &mut NodeRect {
        &mut self.node_rect
    }
    fn get_rect(&self)-> &NodeRect {
        &self.node_rect
    }
    fn update_this(&mut self, ui: &mut egui::Ui,rects:&Vec<Rect>, connectors: &mut Vec<Connector>) {
        self.do_text_box(ui);
        let node_rect = self.get_mut_rect();

        node_rect.assign_rect(ui);
        node_rect.paint_rect(ui);
        node_rect.handle_drag(connectors);
        node_rect.check_new_connector(ui, rects,connectors);
        
    }
}
impl TextNode {
    pub fn do_text_box(&mut self, ui: &mut egui::Ui) {
        egui::Area::new(egui::Id::new(self.node_rect.index)).fixed_pos(self.node_rect.position).movable(false)
        .show(ui.ctx(), |ui| {
                ui.set_min_size(egui::Vec2::new(1.0,1.0)); // force size to match rect
                ui.text_edit_singleline(&mut self.text);
            });
    }
}
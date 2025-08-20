use egui::Rect;

use crate::{
    node_rect::NodeRect,
    node_trait::{self, NodeTrait},
};

pub struct JumbleNode {
    pub node_rect: NodeRect,
}
impl NodeTrait for JumbleNode {
    fn progress_node(&self, mut args: Option<Vec<String>>, rects: &Vec<Box<dyn NodeTrait>>) {
        if args.is_some() {
            args = Some(self.jumble_string(args.as_mut().unwrap().to_vec()));
        } else {
            args.as_mut()
                .unwrap()
                .push("Jumble node got an empty string!".to_string());
        }
        if self.node_rect.connectors.len() > 0 {
            for connector in &self.node_rect.connectors {
                if connector.connected_node.is_some() {
                    if rects[connector.connected_node.unwrap()].get_rect().index
                        != self.node_rect.index
                    {
                        rects[connector.connected_node.unwrap()].progress_node(args.clone(), rects);
                    }
                }
            }
        } else {
            for x in args.unwrap() {
                println!("{}", x);
            }
        }
    }
    fn get_mut_rect(&mut self) -> &mut NodeRect {
        &mut self.node_rect
    }
    fn get_rect(&self) -> &NodeRect {
        &self.node_rect
    }
    fn update_this(&mut self, ui: &mut egui::Ui, rects: &Vec<Rect>) {
        let node_rect = self.get_mut_rect();
        node_rect.assign_rect(ui);
        node_rect.paint_rect(ui);
        node_rect.handle_drag();
        node_rect.check_new_connector(ui, rects);
    }
}
use rand::Rng;
impl JumbleNode {
    fn jumble_string(&self, args: Vec<String>) -> Vec<String>{
        let mut string = String::new();
        let mut finalstring = String::new();
        for string_arg in args {
            string += &string_arg;
        }
        let mut jumbleleft = string.len();
        while jumbleleft > 0 {
            finalstring += &string
                .remove(rand::rng().random_range(0..jumbleleft))
                .to_string();
            jumbleleft -= 1;
        }
        let mut vec = Vec::new();
        vec.push(finalstring);
        vec 
    }
}

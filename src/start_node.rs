use crate::{node_rect::NodeRect, node_trait::{self, NodeTrait}};

pub struct StartNode {
    pub node_rect: NodeRect,
}
impl NodeTrait for StartNode {
    fn progress_node(&self, mut args: Option<Vec<String>>, rects: &Vec<Box<dyn NodeTrait>>) {
        args = Some(Vec::new());
        if self.node_rect.connectors.len() > 0 {
            for connector in &self.node_rect.connectors {
                if connector.connected_node.is_some() {
                    if rects[connector.connected_node.unwrap()].get_rect().index != self.node_rect.index  {
                        rects[connector.connected_node.unwrap()].progress_node(args.clone(), rects);
                    }
                }
            }
        }
        else {
            args = Some(Vec::new());
            args.as_mut().unwrap().push("starting and ending it!".to_string());
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
}
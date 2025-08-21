use crate::{connector::Connector, node_rect::NodeRect, node_trait::NodeTrait};

pub struct StartNode {
    pub node_rect: NodeRect,
}
impl NodeTrait for StartNode {
    fn progress_node(&self, mut args: Option<Vec<String>>, rects: &Vec<Box<dyn NodeTrait>>, connectors: &Vec<Connector>, mut participated_nodes: Vec<bool>) {
        if *participated_nodes.get(self.node_rect.index).unwrap_or(&false) {
            for x in args.unwrap() {
                println!("{}", x);
            }
            return};
        if self.node_rect.index >= participated_nodes.len() {
            participated_nodes.resize(self.node_rect.index + 1, false);
        }
        participated_nodes[self.node_rect.index] = true;
        args = Some(Vec::new());
        let mut self_connectors: Vec<&Connector> = Vec::new();
            for index in self.node_rect.connectors.clone() {
                self_connectors.push(&connectors[index]);
            }
        if self.node_rect.connectors.len() > 0 {
            for connector in self_connectors {
                if connector.connected_node.is_some() {
                    if rects[connector.connected_node.unwrap()].get_rect().index != self.node_rect.index  {
                        rects[connector.connected_node.unwrap()].progress_node(args.clone(), rects,connectors,participated_nodes.clone());
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
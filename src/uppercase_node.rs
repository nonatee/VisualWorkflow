use crate::{
    connector::Connector, node_rect::NodeRect, node_trait::NodeTrait
};

pub struct UppercaseNode {
    pub node_rect: NodeRect,
}
impl NodeTrait for UppercaseNode {
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
        if args.is_some() {
            args = Some(self.uppercase_string(args.as_mut().unwrap().to_vec()));
        } else {
            args.as_mut()
                .unwrap()
                .push("Uppercase node got an empty string!".to_string());
        }
        let mut self_connectors: Vec<&Connector> = Vec::new();
            for index in self.node_rect.connectors.clone() {
                self_connectors.push(&connectors[index]);
            }
        if self.node_rect.connectors.len() > 0 {
            for connector in self_connectors {
                if connector.connected_node.is_some() {
                    if rects[connector.connected_node.unwrap()].get_rect().index
                        != self.node_rect.index
                    {
                        rects[connector.connected_node.unwrap()].progress_node(args.clone(), rects,connectors,participated_nodes.clone());
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
    
}
impl UppercaseNode {
    fn uppercase_string(&self, args: Vec<String>) -> Vec<String>{
        let mut string = String::new();
        let mut finalstring = String::new();
        for string_arg in args {
            string += &string_arg;
        }
        for char in string.chars() {
            finalstring += &char.to_uppercase().to_string();
        }
        let mut vec = Vec::new();
        vec.push(finalstring);
        vec 
    }
}

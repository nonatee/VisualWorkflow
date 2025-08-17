use egui::Pos2;

use crate::node_rect::NodeRect;
#[derive(Clone)]
pub struct Connector {
    pub point1: Option<Pos2>,
    pub point2: Option<Pos2>,
    pub connected_node: Option<usize>
}
impl Connector {
    pub fn new(point1: Pos2) -> Self {
        Self {
            point1:Some(point1),
            point2:None,
            connected_node:None
        }
    }
}
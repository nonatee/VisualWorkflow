use egui::Ui;

use crate::{connector::Connector, node_trait::NodeTrait};

pub trait ButtonStruct {
    fn check_pressed(&self, rects: &Vec<Box<dyn NodeTrait>>, connectors: &mut Vec<Connector>);
    fn init_button(&mut self, ui: &mut Ui);
}
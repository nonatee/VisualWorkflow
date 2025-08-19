use egui::Ui;

use crate::{node_rect::NodeRect, node_trait::NodeTrait};

pub trait ButtonStruct {
    fn check_pressed(&self, rects: &Vec<Box<dyn NodeTrait>>);
    fn init_button(&mut self, ui: &mut Ui);
}
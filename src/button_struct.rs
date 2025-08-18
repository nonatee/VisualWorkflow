use egui::Ui;

use crate::node_rect::NodeRect;

pub trait ButtonStruct {
    fn check_pressed(&mut self, rects: &Vec<NodeRect>);
    fn init_button(&mut self, ui: &mut Ui);
}
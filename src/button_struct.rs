use egui::Ui;

pub trait ButtonStruct {
    fn check_pressed(&mut self);
    fn init_button(&mut self, ui: &mut Ui);
}
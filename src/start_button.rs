use egui::{Button, Rect, Response, Ui};

use crate::button_struct::ButtonStruct;
pub struct StartButton {
    response: Option<Response>,
    text: String,
    rect: Rect,
}
impl StartButton {
    pub fn new(text: String, rect: Rect) -> Self {
        Self {response: None,
        text:text,
    rect:rect}
    }
    pub fn new_button(&mut self) -> Button{
        Button::new(self.text.clone())
    }
    
    
}
impl ButtonStruct for StartButton {
    fn check_pressed(&mut self) {
        if self.response.as_ref().unwrap().clicked() {
            println!("yes i am pressed");
        }
    }
    fn init_button(&mut self, ui: &mut Ui) {
        self.response = Some(ui.put(self.rect, self.new_button()));
    }
}
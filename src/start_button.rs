use egui::{Button, Rect, Response, Ui};
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
    pub fn init_button(&mut self, ui: &mut Ui) {
        self.response = Some(ui.put(self.rect, self.new_button()));
    }
}
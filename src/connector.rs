use egui::Pos2;

pub struct Connector {
    pub point1: Option<Pos2>,
    pub point2: Option<Pos2>
}
impl Connector {
    pub fn new(point1: Pos2) -> Self {
        Self {
            point1:Some(point1),
            point2:None
        }
    }
}
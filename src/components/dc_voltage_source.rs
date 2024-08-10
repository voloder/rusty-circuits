use eframe::egui;
use eframe::egui::{Pos2, Rect, Stroke, Vec2};
use crate::{CircuitElement, ElementType};

#[derive(Clone, Debug)]
pub struct DCVoltageSource {
    pos: Pos2,
    size: Vec2,
    id: u32,
}


impl CircuitElement for DCVoltageSource {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32) -> Box<dyn CircuitElement> {
        Box::new(DCVoltageSource { pos, size, id })
    }

    fn draw(&self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2) {
        let center = screen_pos + screen_size / 2.0;

        let normalized = Vec2::new(screen_size.x, screen_size.y) / screen_size.length();
        let normal = Vec2::new(screen_size.y, -screen_size.x) / screen_size.length();

        let spacing = grid_step * 0.125;
        let length = grid_step * 0.5;
        let half_length = grid_step * 0.25;

        ui.painter().line_segment([center + normalized * spacing + normal * length, center + normalized * spacing - normal * length], stroke);
        ui.painter().line_segment([center - normalized * spacing + normal * half_length, center - normalized * spacing - normal * half_length], stroke);

        ui.painter().line_segment([center + normalized * spacing, screen_pos + screen_size], stroke);
        ui.painter().line_segment([center - normalized * spacing, screen_pos], stroke);


    }

    fn pos(&self) -> Pos2 {
        self.pos
    }

    fn size(&self) -> Vec2 {
        self.size
    }

    fn get_id(&self) -> u32  {
        self.id
    }

    fn get_type(&self) -> ElementType {
        ElementType::DCVoltageSource
    }
}
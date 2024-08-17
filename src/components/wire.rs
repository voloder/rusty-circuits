use eframe::egui;
use eframe::egui::{Pos2, Rect, Stroke, Vec2};
use crate::{CircuitElement, ElementType};

#[derive(Clone, Debug)]

pub struct Wire {
    pos: Pos2,
    size: Vec2,
    id: u32,
}

impl CircuitElement for Wire {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32) -> Box<dyn CircuitElement> {
        Box::new(Wire { pos, size, id })
    }

    fn draw(&mut self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2) {
        ui.painter().line_segment([screen_pos, screen_pos + screen_size], stroke);
    }

    fn pos(&self) -> Pos2 {
        self.pos
    }

    fn size(&self) -> Vec2 {
        self.size
    }

    fn get_admittance(&self) -> f64 {
        f64::MAX
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_type(&self) -> ElementType {
        ElementType::Wire
    }

    fn shorted(&self) -> bool {
        true
    }
}
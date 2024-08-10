use eframe::egui;
use eframe::egui::{Pos2, Rect, Shape, Stroke, Vec2};
use eframe::epaint::PathShape;
use crate::{CircuitElement, ElementType};

#[derive(Clone, Debug)]

pub struct Resistor {
    pos: Pos2,
    size: Vec2,
    resistance: f64,
    id: u32,
}

impl CircuitElement for Resistor {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32) -> Box<dyn CircuitElement> {
        Box::new(Resistor { pos, size, id, resistance: 10.0 })
    }

    fn draw(&self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2) {
        let center = screen_pos + screen_size / 2.0;

        let normalized = Vec2::new(screen_size.x, screen_size.y) / screen_size.length();
        let normal = Vec2::new(screen_size.y, -screen_size.x) / screen_size.length();

        let width = grid_step * 0.25;
        let height = grid_step * 0.5;

        let rectangle = PathShape::closed_line(vec![
            center + normalized * height + normal * width,
            center + normalized * height - normal * width,
            center - normalized * height - normal * width,
            center - normalized * height + normal * width,
        ], stroke);

        ui.painter().add(Shape::Path(rectangle));

        ui.painter().line_segment([center + normalized * height, screen_pos + screen_size], stroke);
        ui.painter().line_segment([center - normalized * height, screen_pos], stroke);


    }

    fn pos(&self) -> Pos2 {
        self.pos
    }

    fn size(&self) -> Vec2 {
        self.size
    }

    fn get_admittance(&self) -> f64 {
        1.0 / self.resistance
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_type(&self) -> ElementType {
        ElementType::Resistor
    }
}


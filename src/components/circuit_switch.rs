use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Values;
use eframe::egui;
use eframe::egui::{Pos2, Rect, Sense, Stroke, Vec2};
use crate::{CircuitElement, ElementType, Node};

#[derive(Clone, Debug)]

pub struct Switch {
    pos: Pos2,
    size: Vec2,
    id: u32,
    closed: bool,
    nodes: Vec<u32>
}

impl CircuitElement for Switch {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32, nodes: Vec<u32>) -> Box<dyn CircuitElement> {
        Box::new(Switch { pos, size, id, closed: false, nodes})
    }

    fn draw(&mut self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2, nodes: &HashMap<(i32, i32), Node>) {
        let center = screen_pos + screen_size / 2.0;

        let response = ui.allocate_rect(Rect::from_two_pos(center - Vec2::splat(grid_step), center + Vec2::splat(grid_step)), Sense::click());
        if response.clicked() {
            self.closed = !self.closed;
        }

        let normalized = Vec2::new(screen_size.x, screen_size.y) / screen_size.length();
        let normal = Vec2::new(screen_size.y, -screen_size.x) / screen_size.length();

        let gap = grid_step * 0.3;
        let length = grid_step * 0.6;
        if self.closed {
            ui.painter().line_segment([screen_pos, screen_pos + screen_size], stroke);
        } else {
            ui.painter().line_segment([center + normalized * length, screen_pos + screen_size], stroke);
            ui.painter().line_segment([center - normalized * length, screen_pos], stroke);
            ui.painter().line_segment([center - normalized * length, center + normalized * length + normal * gap], stroke);
        }

    }

    fn pos(&self) -> Pos2 {
        self.pos
    }

    fn size(&self) -> Vec2 {
        self.size
    }

    fn get_admittance(&self) -> f64 {
        0.0
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_type(&self) -> ElementType {
        ElementType::Wire
    }

    fn shorted(&self) -> bool {
        self.closed
    }

    fn set_nodes(&mut self, nodes: Vec<u32>) {
        self.nodes = nodes;
    }

    fn get_nodes(&self) -> Vec<u32> {
        self.nodes.clone()
    }
}
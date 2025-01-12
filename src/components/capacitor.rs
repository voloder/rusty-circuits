use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Values;
use eframe::egui;
use eframe::egui::{Pos2, Rect, Stroke, Vec2};
use crate::{CircuitElement, ElementType, Node};

#[derive(Clone, Debug)]

pub struct Capacitor {
    pos: Pos2,
    size: Vec2,
    id: u32,
    nodes: Vec<u32>
}
impl CircuitElement for Capacitor {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32, nodes: Vec<u32>) -> Box<dyn CircuitElement> {
        Box::new(Capacitor { pos, size, id, nodes })
    }

    fn draw(&mut self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2, nodes: &HashMap<(i32, i32), Node>) {
        let center = screen_pos + screen_size / 2.0;

        let normalized = Vec2::new(screen_size.x, screen_size.y) / screen_size.length();
        let normal = Vec2::new(screen_size.y, -screen_size.x) / screen_size.length();

        let spacing = grid_step * 0.125;
        let length = grid_step * 0.5;

        ui.painter().line_segment([center + normalized * spacing + normal * length, center + normalized * spacing - normal * length], stroke);
        ui.painter().line_segment([center - normalized * spacing + normal * length, center - normalized * spacing - normal * length], stroke);
        ui.painter().line_segment([center + normalized * spacing, screen_pos + screen_size], stroke);
        ui.painter().line_segment([center - normalized * spacing, screen_pos], stroke);
    }

    fn pos(&self) -> Pos2 {
        self.pos
    }

    fn size(&self) -> Vec2 {
        self.size
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_type(&self) -> ElementType {
        ElementType::Capacitor
    }

    fn set_nodes(&mut self, nodes: Vec<u32>) {
        self.nodes = nodes;
    }

    fn get_nodes(&self) -> Vec<u32> {
        self.nodes.clone()
    }
}

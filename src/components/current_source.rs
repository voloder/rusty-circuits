use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Values;
use eframe::egui;
use eframe::egui::{Color32, Pos2, Rect, Stroke, Vec2};
use crate::{CircuitElement, ElementType, Node};

#[derive(Clone, Debug)]
pub struct CurrentSource {
    pos: Pos2,
    size: Vec2,
    id: u32,
    nodes: Vec<u32>
}


impl CircuitElement for CurrentSource {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32, nodes: Vec<u32>) -> Box<dyn CircuitElement> {
        Box::new(CurrentSource { pos, size, id, nodes})
    }

    fn draw(&mut self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2, nodes: &HashMap<(i32, i32), Node>) {
        let center = screen_pos + screen_size / 2.0;

        let normalized = Vec2::new(screen_size.x, screen_size.y) / screen_size.length();
        let normal = Vec2::new(screen_size.y, -screen_size.x) / screen_size.length();

        let radius = grid_step * 0.5;
        let line_length = grid_step * 0.6;

        ui.painter().circle(center, radius, Color32::TRANSPARENT, stroke);


        ui.painter().line_segment([center + normalized * radius, screen_pos + screen_size], stroke);
        ui.painter().line_segment([center - normalized * radius, screen_pos], stroke);
        ui.painter().arrow(
            center - normalized * line_length * 0.5,
            normalized * line_length,
            stroke
        );
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
        ElementType::CurrentSource
    }

    fn set_nodes(&mut self, nodes: Vec<u32>) {
        self.nodes = nodes;
    }

    fn get_nodes(&self) -> Vec<u32> {
        self.nodes.clone()
    }
}
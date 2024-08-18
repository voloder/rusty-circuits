use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Values;
use eframe::egui;
use eframe::egui::{Pos2, Rect, Stroke, Vec2};
use eframe::egui::accesskit::NodeClassSet;
use crate::{CircuitElement, ElementType, Node};

#[derive(Clone, Debug)]

pub struct Wire {
    pos: Pos2,
    size: Vec2,
    id: u32,
    nodes: Vec<u32>
}

impl CircuitElement for Wire {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32, nodes: Vec<u32>) -> Box<dyn CircuitElement> {
        Box::new(Wire { pos, size, id, nodes })
    }

    fn draw(&mut self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2, nodes: &HashMap<(i32, i32), Node>) {
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

    fn set_nodes(&mut self, nodes: Vec<u32>) {
        self.nodes = nodes;
    }

    fn get_nodes(&self) -> Vec<u32> {
        self.nodes.clone()
    }
}
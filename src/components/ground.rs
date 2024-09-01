use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Values;
use eframe::egui;
use eframe::egui::{Pos2, Rect, Stroke, Vec2};
use nalgebra::{DMatrix, DVector};
use crate::{CircuitElement, ElementType, Node};

#[derive(Clone, Debug)]
pub struct Ground {
    pos: Pos2,
    size: Vec2,
    id: u32,
    nodes: Vec<u32>,
    voltage_node: u32,
}


impl CircuitElement for Ground {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32, nodes: Vec<u32>) -> Box<dyn CircuitElement> {
        Box::new(Ground { pos, size, id, nodes, voltage_node: 0 })
    }

    fn draw(&mut self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2, nodes: &HashMap<(i32, i32), Node>) {

        let normalized = Vec2::new(screen_size.x, screen_size.y) / screen_size.length();
        let normal = Vec2::new(screen_size.y, -screen_size.x) / screen_size.length();

        let spacing = grid_step * 0.125;
        let length = grid_step * 0.5;
        let half_length = grid_step * 0.25;


        ui.painter().line_segment([screen_pos, screen_pos + screen_size], stroke);
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
        ElementType::Ground
    }

    fn set_nodes(&mut self, nodes: Vec<u32>) {
        self.nodes = nodes;
    }

    fn get_nodes(&self) -> Vec<u32> {
        self.nodes.clone()
    }


    fn stamp_matrix(&self, matrix: &mut DMatrix<f64>, vector: &mut DVector<f64>, nodes: &Vec<Node>) {
        let node = nodes.iter().position(|node| node.id == self.nodes[0]).unwrap();

        matrix[(0, node)] += 1.0;
        matrix[(node, 0)] += 1.0;
    }

    fn get_node_positions(&self) -> Vec<(i32, i32)> {
        vec![(self.pos().x as i32, self.pos().y as i32)]
    }
}
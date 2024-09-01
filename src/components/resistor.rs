use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Values;
use eframe::egui;
use eframe::egui::{Pos2, Rect, Shape, Stroke, Ui, Vec2};
use eframe::egui::debug_text::print;
use eframe::epaint::PathShape;
use nalgebra::{DMatrix, DVector};
use crate::{CircuitElement, ElementType, Node};

#[derive(Clone, Debug)]

pub struct Resistor {
    pos: Pos2,
    size: Vec2,
    resistance: f64,
    id: u32,
    nodes: Vec<u32>
}

impl CircuitElement for Resistor {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32, nodes: Vec<u32>) -> Box<dyn CircuitElement> {
        Box::new(Resistor { pos, size, id, resistance: 10.0, nodes })
    }

    fn draw(&mut self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2, nodes: &HashMap<(i32, i32), Node>) {
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

        if self.nodes.len() > 1 {
            let node1 = nodes.values().find(|node| node.id == self.nodes[0]).unwrap();
            let node2 = nodes.values().find(|node| node.id == self.nodes[1]).unwrap();
            let voltage = node1.voltage - node2.voltage;
            ui.allocate_ui_at_rect(Rect::from_two_pos(center + Vec2::new(10.0, 0.0), center + Vec2::new(50.0, 50.0)), |ui| {
                ui.label(format!("{:.2}V", voltage));
            });
        }
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
    fn set_nodes(&mut self, nodes: Vec<u32>) {
        self.nodes = nodes
    }

    fn get_nodes(&self) -> Vec<u32> {
        self.nodes.clone()
    }

    fn stamp_matrix(&self, matrix: &mut DMatrix<f64>, vector: &mut DVector<f64>, nodes: &Vec<Node>) {
        let n1 = nodes.iter().position(|node| node.id == self.nodes[0]).unwrap();
        let n2 = nodes.iter().position(|node| node.id == self.nodes[1]).unwrap();

        matrix[(n1, n1)] += 1.0 / self.resistance;
        matrix[(n2, n2)] += 1.0 / self.resistance;
        matrix[(n1, n2)] -= 1.0 / self.resistance;
        matrix[(n2, n1)] -= 1.0 / self.resistance;
    }

    fn draw_window(&mut self, ctx: &egui::Context) -> Option<Pos2> {
        let window = egui::Window::new(format!("Resistor (id {})", self.id)).show(ctx, |ui| {
            ui.label(format!("Resistance: {:.2} Ohms", self.resistance));
            ui.add(egui::Slider::new(&mut self.resistance, 1.0..=1000.0).text("Resistance"));
        });

        if let Some(window) = window {
            if window.response.hovered() || window.response.has_focus() {
                return Some(window.response.rect.center())
            }
        }

        None
    }
}


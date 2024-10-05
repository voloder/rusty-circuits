use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Values;
use eframe::egui;
use eframe::egui::{Frame, Pos2, Rect, Stroke, Vec2};
use nalgebra::{DMatrix, DVector};
use crate::{CircuitElement, ElementType, Node};

#[derive(Clone, Debug)]
pub struct DCVoltageSource {
    pos: Pos2,
    size: Vec2,
    id: u32,
    nodes: Vec<u32>,
    voltage: f64,
    voltage_node: u32,
    window_hovered: bool,
}


impl CircuitElement for DCVoltageSource {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32, nodes: Vec<u32>) -> Box<dyn CircuitElement> {
        Box::new(DCVoltageSource { pos, size, id, nodes, voltage: 5.0, voltage_node: 0, window_hovered: false})
    }

    fn draw(&mut self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2, nodes: &HashMap<(i32, i32), Node>) {
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

    fn set_nodes(&mut self, nodes: Vec<u32>) {
        self.nodes = nodes;
    }

    fn get_nodes(&self) -> Vec<u32> {
        self.nodes.clone()
    }

    fn get_voltage_source_count(&self) -> u32 {
        1
    }

    fn set_voltage_node(&mut self, node: u32) {
        self.voltage_node = node;
    }

    fn stamp_matrix(&self, matrix: &mut DMatrix<f64>, vector: &mut DVector<f64>, nodes: &Vec<Node>) {
        let node1 = nodes.iter().position(|node| node.id == self.nodes[0]).unwrap();
        let node2 = nodes.iter().position(|node| node.id == self.nodes[1]).unwrap();
        let voltage_node = nodes.len() + self.voltage_node as usize;

        matrix[(voltage_node, node1)] -= 1.0;
        matrix[(voltage_node, node2)] += 1.0;
        vector[voltage_node] = self.voltage;
        matrix[(node1, voltage_node)] -= 1.0;
        matrix[(node2, voltage_node)] += 1.0;
    }


    fn draw_window(&mut self, ctx: &egui::Context) -> Option<Pos2> {
        let mut window = egui::Window::new(format!("DC Voltage Source (id {})", self.id));

        if self.window_hovered {
            window = window.frame(
                Frame::window(&ctx.style()).stroke(
                    Stroke::new(1.0, egui::Color32::GREEN),
                ),
            );
        }

        let window_response = window.show(ctx, |ui| {
            ui.label(format!("Voltage: {:.2} V", self.voltage));
            ui.add(egui::Slider::new(&mut self.voltage, 0.0..=1000.0).text("Voltage"));
        });

        if let Some(window) = window_response {
            if window.response.hovered() || window.response.has_focus() {
                self.window_hovered = true;
                return Some(window.response.rect.center());
            }
        }
        self.window_hovered = false;
        None
    }
}
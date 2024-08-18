#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example


use nalgebra::{LU, Matrix3, Vector3};


mod circuit_solver;
mod components;

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::collections::hash_map::Values;
use std::num::FpCategory::Zero;
use std::ops::Add;
use eframe::{egui, WindowBuilder};
use eframe::egui::{FontFamily, Key, Rgba, RichText, WidgetText};
use eframe::emath::Vec2;
use eframe::epaint::{Color32, Pos2, Shape, Stroke};
use egui::{Rect, Sense};
use nalgebra::DMatrix;
use crate::circuit_solver::simplify_graph;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ElementType {
    Wire,
    Resistor,
    Capacitor,
    DCVoltageSource,
    CurrentSource,
    Switch,
}


trait CircuitElement: ElementClone + std::fmt::Debug {
    fn new_boxed(pos: Pos2, size: Vec2, id: u32, nodes: Vec<u32>) -> Box<dyn CircuitElement>
    where
        Self: Sized;
    fn draw(&mut self, ui: &mut egui::Ui, stroke: Stroke, grid_step: f32, screen_pos: Pos2, screen_size: Vec2, values: &HashMap<(i32, i32), Node>);
    fn pos(&self) -> Pos2;
    fn size(&self) -> Vec2;
    fn get_admittance(&self) -> f64 {
        0.0
    }
    fn get_id(&self) -> u32;
    fn get_type(&self) -> ElementType;
    fn shorted(&self) -> bool {
        false
    }

    fn set_nodes(&mut self, nodes: Vec<u32>);
    fn get_nodes(&self) -> Vec<u32>;
}

trait ElementClone {
    fn clone_box(&self) -> Box<dyn CircuitElement>;
}

impl<T> ElementClone for T
where
    T: 'static + CircuitElement + Clone,
{
    fn clone_box(&self) -> Box<dyn CircuitElement> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CircuitElement> {
    fn clone(&self) -> Box<dyn CircuitElement> {
        self.clone_box()
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Rusty Circuits",
        options,
        Box::new(|cc| {
            Ok(Box::<RustyCircuits>::default())
        }),
    )
}

#[derive(Debug, Clone)]
struct Node {
    id: u32,
    voltage: f64,
    connections: BTreeSet<u32>,
}

struct DebugOptions {
    show_node_numbers: bool,
    show_node_voltages: bool,

    info_simplfication: bool,
    info_admittance_matrix: bool,
    info_injected_currents: bool,
    info_node_voltages: bool,
    info_node_map: bool,
}

impl DebugOptions {
    fn new() -> Self {
        Self {
            show_node_numbers: false,
            show_node_voltages: false,

            info_simplfication: false,
            info_admittance_matrix: false,
            info_injected_currents: false,
            info_node_voltages: false,
            info_node_map: false,
        }
    }
}

struct RustyCircuits {
    offset: Vec2,
    grid_step: f32,
    selected_element_type: ElementType,
    current_element: Option<Box<dyn CircuitElement>>,
    elements: BTreeMap<u32, Box<dyn CircuitElement>>,
    nodes: HashMap<(i32, i32), Node>,
    debug_options: DebugOptions,
}

impl Default for RustyCircuits {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            grid_step: 35.0,
            selected_element_type: ElementType::Wire,
            current_element: None,
            elements: BTreeMap::new(),
            nodes: HashMap::new(),
            debug_options: DebugOptions::new(),
        }
    }
}

impl eframe::App for RustyCircuits {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // for continuous updates
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            let input = ctx.input(|i| i.clone());
            let mut debug_info = String::new();
            ui.input(|input_state| {});

            for line in (0..(ui.available_width() + self.grid_step) as i32).step_by(self.grid_step as usize) {
                let x = line as f32 + self.offset.x % self.grid_step;
                ui.painter().line_segment([Pos2::new(x as f32, 0.0), Pos2::new(x as f32, ui.available_height() + self.grid_step)], Stroke::new(0.5, Color32::from_gray(64)));
            }

            for column in (0..(ui.available_height() + self.grid_step) as i32).step_by(self.grid_step as usize) {
                let y = column as f32 + self.offset.y % self.grid_step;
                ui.painter().line_segment([Pos2::new(0.0, y as f32), Pos2::new(ui.available_width() + self.grid_step, y as f32)], Stroke::new(0.5, Color32::from_gray(64)));
            }

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_element_type, ElementType::Wire, "Wire");
                ui.selectable_value(&mut self.selected_element_type, ElementType::Resistor, "Resistor");
                ui.selectable_value(&mut self.selected_element_type, ElementType::Capacitor, "Capacitor");
                ui.selectable_value(&mut self.selected_element_type, ElementType::DCVoltageSource, "DC Voltage Source");
                ui.selectable_value(&mut self.selected_element_type, ElementType::CurrentSource, "Current Source");
                ui.selectable_value(&mut self.selected_element_type, ElementType::Switch, "Switch");
            });

            ui.allocate_ui_at_rect(Rect::from_min_size(Pos2::new(ui.available_width() - 180.0, 0.0), Vec2::new(180.0, 150.0)), |ui| {
                ui.label("Debug options");
                ui.checkbox(&mut self.debug_options.show_node_numbers, "Show node numbers");
                ui.checkbox(&mut self.debug_options.show_node_voltages, "Show node voltages");

                ui.checkbox(&mut self.debug_options.info_simplfication, "Show simplification info");
                ui.checkbox(&mut self.debug_options.info_admittance_matrix, "Show admittance matrix");
                ui.checkbox(&mut self.debug_options.info_node_map, "Show node map");
                ui.checkbox(&mut self.debug_options.info_injected_currents, "Show injected current vector");
                ui.checkbox(&mut self.debug_options.info_node_voltages, "Show node voltage vector");
            });


            let (rect, response) =
                ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());


            if response.dragged() {
                if input.key_down(Key::C) {
                    self.offset += response.drag_delta();
                } else if self.current_element.is_some() {
                    let element = self.current_element.as_ref().unwrap();
                    let new_size = response.interact_pointer_pos().unwrap().to_vec2() - self.grid_to_screen(element.pos()).to_vec2();
                    self.current_element = Some(self.create_element(element.pos(), self.screen_to_grid_vec(new_size), 0, Vec::new()));
                } else {
                    self.current_element = Some(self.create_element(
                        self.screen_to_grid(response.interact_pointer_pos().unwrap() - response.drag_delta()),
                        self.screen_to_grid_vec(response.drag_delta()), 0, Vec::new(),
                    ));
                }
            } else if self.current_element.is_some() {
                if self.current_element.as_ref().unwrap().size() != Vec2::ZERO {
                    let element = self.current_element.as_ref().unwrap();

                    let node1 = (element.pos().x as i32, element.pos().y as i32);
                    let node2 = (element.pos().x as i32 + element.size().x as i32, element.pos().y as i32 + element.size().y as i32);

                    // find the first unused id
                    let id = self.get_next_element_id();

                    let node1_id;
                    if self.nodes.contains_key(&node1) {
                        let node = self.nodes.get_mut(&node1).unwrap();
                        node1_id = node.id;
                        node.connections.insert(id);
                    } else {
                        node1_id = self.get_next_node_id();
                        self.nodes.insert(node1, Node { id: node1_id, voltage: 0.0, connections: BTreeSet::from([id]) });
                    }

                    let node2_id;
                    if self.nodes.contains_key(&node2) {
                        let node = self.nodes.get_mut(&node2).unwrap();
                        node2_id = node.id;
                        node.connections.insert(id);
                    } else {
                        node2_id = self.get_next_node_id();
                        self.nodes.insert(node2, Node { id: node2_id, voltage: 0.0, connections: BTreeSet::from([id]) });
                    }

                    println!("Node 1: {}, Node 2: {}", node1_id, node2_id);
                    self.elements.insert(id, self.create_element(element.pos(), element.size(), id, [node1_id, node2_id].to_vec()));
                }
                self.current_element = None;
            }

            for element in self.elements.values_mut() {
                let stroke = Stroke::new(2.0, Color32::WHITE);
                let screen_pos = element.pos() * self.grid_step + self.offset;
                let screen_size = element.size() * self.grid_step;

                element.draw(ui, stroke, self.grid_step, screen_pos, screen_size, &self.nodes);

                let center = screen_pos + screen_size / 2.0;

                /*ui.allocate_ui_at_rect(Rect::from_two_pos(center, center + Vec2::new(20.0, 20.0)), |ui| {
                    ui.label(egui::RichText::new(format!("{}", element.get_id())).color(Rgba::GREEN));
                });*/
            }

            for (pos, node) in self.nodes.iter() {
                let screen_pos = self.grid_to_screen(Pos2::new(pos.0 as f32, pos.1 as f32));
                if self.debug_options.show_node_numbers {
                    ui.allocate_ui_at_rect(Rect::from_two_pos(screen_pos + Vec2::new(5.0, 5.0), screen_pos + Vec2::new(50.0, 50.0)), |ui| {
                        ui.label(RichText::new(format!("{}", node.id)).color(Rgba::RED));
                    });
                }

                if self.debug_options.show_node_voltages {
                    ui.allocate_ui_at_rect(Rect::from_two_pos(screen_pos + Vec2::new(15.0, 5.0), screen_pos + Vec2::new(100.0, 50.0)), |ui| {
                        ui.label(RichText::new(format!("{:.2}V", node.voltage)).color(Rgba::GREEN));
                    });
                }
            }

            if self.current_element.is_some() {
                let mut element = self.current_element.as_deref_mut().unwrap();
                let screen_pos = element.pos() * self.grid_step + self.offset;
                let screen_size = element.size() * self.grid_step;

                ui.painter().rect_stroke(Rect::from_two_pos(
                    screen_pos,
                    screen_pos + screen_size),
                                         0.0, Stroke::new(1.0, Color32::DARK_GREEN));
                let stroke = Stroke::new(2.0, Color32::WHITE);
                element.draw(ui, stroke, self.grid_step, screen_pos, screen_size, &self.nodes);
            }

            let mut nodes: Vec<_> = self.nodes.values().collect::<Vec<&Node>>().into_iter().cloned().collect();
            let mut elements = self.elements.clone();
            let mut simplification_info = String::new();

            let nodes_map = simplify_graph(&mut nodes, &mut elements, &mut simplification_info);
            if self.debug_options.info_simplfication {
                debug_info += "------ Simplifying nodes ------\n";
                debug_info += simplification_info.as_str();
                debug_info += "-------------------------------\n";
            }

            if self.debug_options.info_node_map {
                debug_info += format!("Node map: {:?}\n", nodes_map).as_str();
            }

            if nodes.len() > 0 {
                let mut admittance_matrix = DMatrix::from_element(nodes.len(), nodes.len(), 0.0);

                for (y, node) in nodes.iter().enumerate() {

                    // diagonal elements: sum of self-admittances
                    for connection in node.connections.iter() {
                        let element = self.elements.get(connection).unwrap();
                        admittance_matrix[(y, y)] += element.get_admittance();
                    }

                    // off-diagonal elements: -admittance for shared elements
                    for (x, other_node) in nodes.iter().enumerate() {
                        if x == y {
                            continue;
                        }
                        let shared_connections: Vec<_> = node.connections.intersection(&other_node.connections).collect();
                        for connection in shared_connections {
                            let element = self.elements.get(connection).unwrap();
                            admittance_matrix[(x, y)] -= element.get_admittance();
                        }
                    }
                }

                let lu = LU::new(admittance_matrix.clone());

                if self.debug_options.info_admittance_matrix {
                    debug_info += format!("Admittance Matrix:{}\n", admittance_matrix).as_str();
                }

                let mut currents = DMatrix::<f64>::zeros(admittance_matrix.nrows(), 1);

                for element in elements.values() {
                    let element_nodes = element.get_nodes();
                    if element.get_type() == ElementType::CurrentSource && element_nodes.len() > 1 {
                        let node1_index = nodes.iter().position(|node| node.id == element_nodes[0]).unwrap();
                        let node2_index = nodes.iter().position(|node| node.id == element_nodes[1]).unwrap();

                        let current = 1.0;

                        currents[(node2_index, 0)] += current;
                        currents[(node1_index, 0)] -= current;
                    }
                }
                if self.debug_options.info_injected_currents {
                    debug_info += format!("Injected currents:{}\n", currents).as_str();
                }
                let solution = lu.solve(&currents);

                if let Some(voltages) = solution {
                    for (index, node) in nodes.into_iter().enumerate() {
                        let mapped_node_ids = nodes_map.get(&node.id);
                        // if the node is mapped to another node, also update the voltage of the mapped node
                        if let Some(mapped_node_ids) = mapped_node_ids {
                            for mapped_node_id in mapped_node_ids.iter() {
                                let node = self.nodes.values_mut().find(|n| n.id == *mapped_node_id).unwrap();
                                node.voltage = voltages[(index, 0)];
                            }
                        }

                        let node = self.nodes.values_mut().find(|n| n.id == node.id).unwrap();
                        node.voltage = voltages[(index, 0)];
                    }
                    if self.debug_options.info_node_voltages {
                        debug_info += format!("Node voltages:{}\n", voltages).as_str();
                    }
                } else {
                    debug_info += "Matrix is singular\n";
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label(RichText::new(debug_info).family(FontFamily::Monospace).small());
            });
        });
    }
}

impl RustyCircuits {
    fn create_element(&self, pos: Pos2, size: Vec2, id: u32, nodes: Vec<u32>) -> Box<dyn CircuitElement> {
        match self.selected_element_type {
            ElementType::Wire => components::wire::Wire::new_boxed(pos, size, id, nodes),
            ElementType::Resistor => components::resistor::Resistor::new_boxed(pos, size, id, nodes),
            ElementType::Capacitor => components::capacitor::Capacitor::new_boxed(pos, size, id, nodes),
            ElementType::DCVoltageSource => components::dc_voltage_source::DCVoltageSource::new_boxed(pos, size, id, nodes),
            ElementType::CurrentSource => components::current_source::CurrentSource::new_boxed(pos, size, id, nodes),
            ElementType::Switch => components::circuit_switch::Switch::new_boxed(pos, size, id, nodes),
        }
    }

    fn grid_to_screen(&self, pos: Pos2) -> Pos2 {
        pos * self.grid_step + self.offset
    }

    fn screen_to_grid(&self, pos: Pos2) -> Pos2 {
        ((pos - self.offset) / self.grid_step).round()
    }

    fn screen_to_grid_vec(&self, vec: Vec2) -> Vec2 {
        (vec / self.grid_step).round()
    }

    fn get_next_element_id(&self) -> u32 {
        let mut id = 0;
        for i in 0.. {
            if !self.elements.contains_key(&(i as u32)) {
                id = i as u32;
                break;
            }
        }
        id
    }

    fn get_next_node_id(&self) -> u32 {
        let mut id = 0;
        for i in 0.. {
            if !self.nodes.values().any(|node| node.id == i as u32) {
                id = i as u32;
                break;
            }
        }
        id
    }
}
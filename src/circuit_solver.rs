use eframe::egui::{Pos2, Vec2};
use eframe::egui::ahash::{HashMap, HashMapExt};
use crate::ElementType;

struct Node {
    id: usize,
    voltage: f64,
    current: f64,
    neighbors: Vec<usize>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            voltage: 0.0,
            current: 0.0,
            neighbors: vec![],
        }
    }
}


fn solve_circuit(elements: &Vec<Box<dyn crate::CircuitElement>>) -> Vec<Node> {
    let mut nodes = vec![];
    let mut node_map = HashMap::new();

    for element in elements.iter() {
        let pos = element.pos();
        let size = element.size();

        let mut add_node = |pos: Pos2| {
            let id = nodes.len();
            let node = Node::new(id);
            nodes.push(node);
            node_map.insert(&(pos.x, pos.y), id);
            id
        };

        let mut get_node = |pos: Pos2| {
            if let Some(&id) = node_map.get(&(pos.x, pos.y)) {
                id
            } else {
                add_node(pos)
            }
        };

        let mut add_edge = |pos1: Pos2, pos2: Pos2| {
            let id1 = get_node(pos1);
            let id2 = get_node(pos2);
            nodes[id1].neighbors.push(id2);
            nodes[id2].neighbors.push(id1);
        };

        match element.as_ref() {
            ElementType::Wire => {
                add_edge(pos, pos + size);
            }
            ElementType::Resistor => {
                add_edge(pos, pos + Vec2::new(0.0, size.y));
                add_edge(pos + Vec2::new(size.x, 0.0), pos + size);
            }
            ElementType::VoltageSource => {
                add_edge(pos, pos + Vec2::new(0.0, size.y));
                add_edge(pos + Vec2::new(size.x, 0.0), pos + size);
            }
        }
    }

    nodes
}
use std::collections::{HashMap, HashSet};
use crate::{CircuitElement, ElementType, Node};
use itertools::Itertools;

pub fn simplify_graph(
    nodes: &mut Vec<Node>,
    elements: &mut HashMap<u32, Box<dyn CircuitElement>>,
) {
    println!("------ Simplifying graph ------ ");
    let mut pass = 0;
    let mut changed = true;
    while changed {
        pass += 1;

        println!("Pass {}:", pass);
        changed = false;
        // remove dangling nodes
        let len_old = nodes.len();
        nodes.retain(|node| {
            node.connections.len() > 1
        });
        if nodes.len() != len_old {
            println!("Removed {} dangling nodes", len_old - nodes.len());
            changed = true;
        }

        // remove elements that are not connected to any nodes
        let len_old = elements.len();
        elements.retain(|id, element| {
            nodes.iter().filter(|node| {
                node.connections.contains(&id)
            }).take(2).count() > 1
        });

        if elements.len() != len_old {
            println!("Removed {} elements", len_old - elements.len());
            changed = true;
        }

        // remove connections in nodes whose elements were removed
        for node in nodes.iter_mut() {
            let len_old = node.connections.len();
            node.connections.retain(|connection| {
                elements.keys().any(|id| {
                    *id == *connection
                })
            });
            if node.connections.len() != len_old {
                println!("Removed {} connections in node {}", len_old - node.connections.len(), node.id);
                changed = true;
            }
        }

        // simplify wires
        let mut nodes_to_remove = HashSet::new();
        let mut node_connections: HashMap<u32, Vec<u32>> = HashMap::new();

        // collect nodes and connections to update
        for i in 0..nodes.len() {
            let node = &nodes[i];
            if nodes_to_remove.contains(&node.id) {
                continue;
            }

            for j in 0..nodes.len() {
                if i == j {
                    continue;
                }

                let other = &nodes[j];
                if nodes_to_remove.contains(&other.id) {
                    continue;
                }

                let common_connections: Vec<u32> = node.connections.iter()
                    .filter(|&conn| other.connections.contains(conn))
                    .copied()
                    .collect();

                for connection in common_connections {
                    if let Some(element) = elements.get(&connection) {
                        if element.get_type() == ElementType::Wire {
                            println!("Merging node {} -> {}", other.id, node.id);

                            node_connections.entry(node.id)
                                .or_default()
                                .extend(other.connections.iter().copied());

                            nodes_to_remove.insert(other.id);
                            changed = true;
                        }
                    }
                }
            }
        }

        // apply node connections updates
        for node in nodes.iter_mut() {
            if let Some(connections) = node_connections.remove(&node.id) {
                node.connections.extend(connections);
                node.connections.sort();
                node.connections.dedup();
                println!("Updated node {} connections: {:?}", node.id, node.connections);
            }
        }

        // remove nodes
        nodes.retain(|node| !nodes_to_remove.contains(&node.id));
    }
}
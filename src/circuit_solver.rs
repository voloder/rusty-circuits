use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};
use crate::{CircuitElement, Node};

// does the following things:
// - remove dangling nodes and elements
// - simplify wires (interconnected wires should be a single node)
pub fn simplify_graph(
    nodes: &mut Vec<Node>,
    elements: &mut BTreeMap<u32, Box<dyn CircuitElement>>,
    debug_info: &mut String,
) -> BTreeMap<u32, BTreeSet<u32>> {
    let mut pass = 0;
    let mut changed = true;

    // todo map
    let mut nodes_map: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();

    while changed {
        pass += 1;

        *debug_info += format!("\nPass {}:\n", pass).as_str();

        changed = false;

        let nodes_clone = nodes.clone();

        // remove dangling nodes
        let len_old = nodes.len();
        nodes.retain(|node| {
            node.connections.len() > 1
        });

        if nodes.len() != len_old {
            *debug_info += format!("Removed {} dangling nodes\n", len_old - nodes.len()).as_str();
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
            *debug_info += format!("Removed {} elements\n", len_old - elements.len()).as_str();
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
                *debug_info += format!("Removed {} connections in node {}\n", len_old - node.connections.len(), node.id).as_str();
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
                        if element.shorted() {
                            *debug_info += format!("Merging node {} -> {}\n", other.id, node.id).as_str();

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
                *debug_info += format!("Updated node {} connections: {:?}\n", node.id, node.connections).as_str();
            }
        }

        // remove nodes
        nodes.retain(|node| {
            !nodes_to_remove.contains(&node.id)
        });

        if !changed {
            *debug_info += "Nothing to do\n";
        }
    }

    nodes_map
}
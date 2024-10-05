use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};
use crate::{CircuitElement, ElementType, Node};

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

    let mut nodes_map: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();

    while changed {
        pass += 1;

        *debug_info += format!("\nPass {}:\n", pass).as_str();

        changed = false;

        // simplify wires
        let mut nodes_to_remove = HashSet::new();
        let mut node_connections: HashMap<u32, Vec<u32>> = HashMap::new();

        // collect nodes and connections to update
        for i in 1..nodes.len() {
            let node = &nodes[i];
            if nodes_to_remove.contains(&node.id) {
                continue;
            }
            for j in 1..nodes.len() {
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
                            merge_nodes(node, other, &mut nodes_map, elements, &mut node_connections, &mut nodes_to_remove, debug_info);
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
                *debug_info += format!("Updated node {} connections: {:?}\n", node.id, node.connections).as_str();
            }
        }

        // remove nodes
        nodes.retain(|node| {
            !nodes_to_remove.contains(&node.id)
        });

        // remove dangling nodes
        /*let len_old = nodes.len();
        nodes.retain(|node| {
            node.connections.len() > 1
        });

        if nodes.len() != len_old {
            *debug_info += format!("Removed {} dangling nodes\n", len_old - nodes.len()).as_str();
            changed = true;
        }*/

        // remove elements that are not connected to any nodes
        let len_old = elements.len();
        elements.retain(|id, element| {
            nodes.iter().filter(|node| {
                node.connections.contains(&id)
            }).take(2).count() > 0
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


        if !changed {
            *debug_info += "Nothing to do\n";
        }
    }

    nodes_map
}

fn merge_nodes(node: &Node, other: &Node, nodes_map: &mut BTreeMap<u32, BTreeSet<u32>>, elements: &mut BTreeMap<u32, Box<dyn CircuitElement>>, node_connections: &mut HashMap<u32, Vec<u32>>, nodes_to_remove: &mut HashSet<u32>, debug_info: &mut String) {
    nodes_map.entry(node.id).or_default().insert(other.id);

    if let Some(set) = nodes_map.remove(&other.id) {
        *debug_info += format!("Set: {:?}\n", set).as_str();
        nodes_map.entry(node.id).or_default().extend(set);
    }

    for element in elements.values_mut() {
        element.set_nodes(element.get_nodes().iter().map(|node_id| {
            if *node_id == other.id {
                node.id
            } else {
                *node_id
            }
        }).collect());
    }

    node_connections.entry(node.id)
        .or_default()
        .extend(other.connections.iter().copied());

    nodes_to_remove.insert(other.id);
}
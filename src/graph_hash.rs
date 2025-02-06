use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;


pub fn graph_hash(g: &Graph<(), (), Undirected>) -> u64 {
    let mut labels: HashMap<NodeIndex, u64> = HashMap::new();
    for n in g.node_indices() {
        let degree = g.neighbors(n).count() as u64;
        labels.insert(n, degree);
    }

    for _round in 0..5 {
        let mut new_labels: HashMap<NodeIndex, u64> = HashMap::new();
        for n in g.node_indices() {
            let current_label = labels[&n];
            let mut nbr_labels: Vec<u64> = g.neighbors(n)
                .map(|nbr| labels[&nbr])
                .collect();
            nbr_labels.sort();
            let mut hash_val = 1469598103934665603u64; // FNV offset basis
            hash_val ^= current_label;
            hash_val = hash_val.wrapping_mul(1099511628211u64);
            for &nbr_label in nbr_labels.iter() {
                hash_val ^= nbr_label;
                hash_val = hash_val.wrapping_mul(1099511628211u64);
            }
            new_labels.insert(n, hash_val);
        }
        labels = new_labels;
    }

    let mut all_labels: Vec<u64> = labels.values().cloned().collect();
    all_labels.sort();
    let mut final_hash = 1469598103934665603u64;
    for label in all_labels {
        final_hash ^= label;
        final_hash = final_hash.wrapping_mul(1099511628211u64);
    }
    final_hash
}

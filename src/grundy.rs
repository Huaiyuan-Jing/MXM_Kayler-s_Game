use crate::graph_hash::graph_hash;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};

pub fn grundy(
    g: &Graph<(), (), petgraph::Undirected>,
    grundy_cache: &mut HashMap<u64, u64>,
) -> u64 {
    // println!("nodes: {}, edges: {}", g.node_count(), g.edge_count());
    if g.edge_count() == 0 {
        return 0;
    }
    if g.edge_count() == 1 {
        return 1;
    }
    if grundy_cache.contains_key(&graph_hash(g)) {
        return grundy_cache[&graph_hash(g)];
    }
    let mut rev_mex = HashSet::new();
    for edge in g.edge_indices() {
        let mut tmp = g.clone();
        tmp.remove_edge(edge);
        let nodes_to_remove: Vec<NodeIndex> = tmp
            .node_indices()
            .filter(|&node| tmp.neighbors(node).count() == 0)
            .collect();
        for node in nodes_to_remove {
            tmp.remove_node(node);
        }
        rev_mex.insert(grundy(&tmp, grundy_cache));
    }
    for node in g.node_indices() {
        let mut tmp = g.clone();
        tmp.remove_node(node);
        let nodes_to_remove: Vec<NodeIndex> = tmp
            .node_indices()
            .filter(|&node| tmp.neighbors(node).count() == 0)
            .collect();
        for node in nodes_to_remove {
            tmp.remove_node(node);
        }
        rev_mex.insert(grundy(&tmp, grundy_cache));
    }
    let mut mex = 0;
    while rev_mex.contains(&mex) {
        mex += 1;
    }
    grundy_cache.insert(graph_hash(g), mex);
    mex
}

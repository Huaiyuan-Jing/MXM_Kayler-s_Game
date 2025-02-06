use crate::graph_hash::graph_hash;
use petgraph::graph::Graph;
use std::collections::HashSet;
use rayon::prelude::*;
use dashmap::DashMap;

pub fn grundy(
    g: &Graph<(), (), petgraph::Undirected>,
    grundy_cache: &DashMap<u64, u64>,
) -> u64 {
    if g.edge_count() == 0 {
        return 0;
    }
    if g.edge_count() == 1 {
        return 1;
    }
    let hash = graph_hash(g);
    if let Some(value) = grundy_cache.get(&hash) {
        return *value;
    }

    let mut rev_mex = HashSet::new();

    let edge_results: Vec<u64> = g.edge_indices().par_bridge().map(|edge| {
        let mut tmp = g.clone();
        tmp.remove_edge(edge);
        let nodes_to_remove: Vec<_> = tmp.node_indices()
            .filter(|&node| tmp.neighbors(node).count() == 0)
            .collect();
        for node in nodes_to_remove {
            tmp.remove_node(node);
        }
        grundy(&tmp, grundy_cache)
    }).collect();

    rev_mex.extend(edge_results);

    let node_results: Vec<u64> = g.node_indices().par_bridge().map(|node| {
        let mut tmp = g.clone();
        tmp.remove_node(node);
        let nodes_to_remove: Vec<_> = tmp.node_indices()
            .filter(|&n| tmp.neighbors(n).count() == 0)
            .collect();
        for n in nodes_to_remove {
            tmp.remove_node(n);
        }
        grundy(&tmp, grundy_cache)
    }).collect();

    rev_mex.extend(node_results);

    let mex = (0..).find(|&m| !rev_mex.contains(&m)).unwrap();
    grundy_cache.insert(hash, mex);
    mex
}

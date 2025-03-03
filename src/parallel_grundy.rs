use crate::grundy_cache::GrundyCache;
use petgraph::graph::Graph;
use rayon::prelude::*;
use std::collections::HashSet;

fn remove_isolated_nodes(g: &Graph<(), (), petgraph::Undirected>) -> Graph<(), (), petgraph::Undirected>{ 
    let nodes_to_remove: Vec<_> = g
        .node_indices()
        .filter(|&node| g.neighbors(node).count() == 0)
        .collect();
    let mut tmp = g.clone();
    for node in nodes_to_remove {
        tmp.remove_node(node);
    }
    tmp
}

pub fn grundy(g: &Graph<(), (), petgraph::Undirected>, grundy_cache: &GrundyCache) -> u64 {
    let g = remove_isolated_nodes(g);
    if g.edge_count() == 0 {
        return 0;
    }
    if g.edge_count() == 1 {
        return 1;
    }
    let tmp = grundy_cache.get(&g);
    if tmp != -1 {
        return tmp as u64;
    }

    let mut rev_mex = HashSet::new();

    let edge_results: Vec<u64> = g
        .edge_indices()
        .par_bridge()
        .map(|edge| {
            let mut tmp = g.clone();
            tmp.remove_edge(edge);
            grundy(&tmp, grundy_cache)
        })
        .collect();

    rev_mex.extend(edge_results);

    let node_results: Vec<u64> = g
        .node_indices()
        .par_bridge()
        .map(|node| {
            let mut tmp = g.clone();
            tmp.remove_node(node);
            grundy(&tmp, grundy_cache)
        })
        .collect();

    rev_mex.extend(node_results);

    let mex = (0..).find(|&m| !rev_mex.contains(&m)).unwrap();
    grundy_cache.insert(&g, mex);
    mex
}

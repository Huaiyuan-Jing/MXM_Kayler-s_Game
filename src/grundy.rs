use crate::grundy_cache::GrundyCache;
use petgraph::graph::Graph;
use std::collections::HashSet;

fn remove_isolated_nodes(
    g: &Graph<(), (), petgraph::Undirected>,
) -> Graph<(), (), petgraph::Undirected> {
    let mut tmp = g.clone();
    let nodes_to_remove: Vec<_> = tmp
        .node_indices()
        .filter(|&node| g.neighbors(node).count() == 0)
        .collect();
    for node in nodes_to_remove {
        tmp.remove_node(node);
    }
    tmp
}
pub fn grundy(g: &Graph<(), (), petgraph::Undirected>, grundy_cache: &mut GrundyCache) -> u64 {
    let g = remove_isolated_nodes(g);
    if g.edge_count() == 0 {
        return 0;
    }
    let tmp = grundy_cache.get(&g);
    if tmp != -1 {
        return tmp as u64;
    }
    let mut rev_mex = HashSet::new();
    for edge in g.edge_indices() {
        let mut tmp = g.clone();
        tmp.remove_edge(edge);
        rev_mex.insert(grundy(&tmp, grundy_cache));
    }
    for node in g.node_indices() {
        let mut tmp = g.clone();
        tmp.remove_node(node);
        rev_mex.insert(grundy(&tmp, grundy_cache));
    }
    let mut mex = 0;
    while rev_mex.contains(&mex) {
        mex += 1;
    }
    grundy_cache.insert(&g, mex);
    mex
}

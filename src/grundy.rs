use crate::grundy_cache::GrundyCache;
use petgraph::graph::Graph;
use std::collections::HashSet;

fn remove_isolated_nodes(
    g: &Graph<(), (), petgraph::Undirected>,
) -> Graph<(), (), petgraph::Undirected> {
    let mut tmp = g.clone();
    let mut nodes_to_remove: Vec<_> = tmp
        .node_indices()
        .filter(|&node| tmp.neighbors(node).count() == 0)
        .collect();
    nodes_to_remove.reverse();
    for node in nodes_to_remove {
        tmp.remove_node(node);
    }
    tmp
}
pub fn grundy(
    g: &Graph<(), (), petgraph::Undirected>,
    grundy_cache: &mut GrundyCache,
    show: bool,
    depth: i32,
) -> u64 {
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
        rev_mex.insert(grundy(&tmp, grundy_cache, show, depth + 1));
    }
    for node in g.node_indices() {
        let mut tmp = g.clone();
        if g.neighbors(node).count() == 0 {
            continue;
        }
        tmp.remove_node(node);
        rev_mex.insert(grundy(&tmp, grundy_cache, show, depth + 1));
    }
    let mut mex = 0;
    while rev_mex.contains(&mex) {
        mex += 1;
    }
    grundy_cache.insert(&g, mex);
    if show && mex == 0{
        println!(
            "Depth: {}, Node numbers {}, Edge numbers {}",
            depth,
            g.node_count(),
            g.edge_count()
        );
        for node in g.node_indices() {
            print!("{}'s neighbor: ", node.index());
            for neighbor in g.neighbors(node) {
                print!("{}, ", neighbor.index());
            }
            println!("");
        }
        println!("mex: {}", mex);
    }
    mex
}

#[test]
fn test_remove_isolated_nodes() {
    let mut graph: Graph<(), (), petgraph::Undirected> = Graph::new_undirected();
    for _ in 0..3 {
        graph.add_node(());
    }
    let g = remove_isolated_nodes(&graph);
    for node in g.node_indices() {
        print!("{}: {} neighbors", node.index(), g.neighbors(node).count());
    }
    println!("");
    assert_eq!(g.node_count(), 0);
}

pub mod graph_hash;
pub mod grundy;
pub mod parallel_grundy;
use dashmap::DashMap;
use petgraph::graph::Graph;
use std::collections::HashMap;

fn main() {
    let mut grundy_cache = DashMap::new();
    for j in 1..=12 {
        test_complete_graph(j, &mut grundy_cache);
    }
}

fn test_complete_graph(n: usize, grundy_cache: &mut DashMap<u64, u64>) {
    let mut nodes = Vec::new();
    let mut graph: Graph<(), (), petgraph::Undirected> = Graph::new_undirected();
    for _ in 0..n {
        let node = graph.add_node(());
        nodes.push(node);
    }
    for i in 0..n - 1 {
        for j in i + 1..n {
            graph.add_edge(nodes[i], nodes[j], ());
        }
    }
    println!(
        "grundy of complete graph of size {} is {}",
        n,
        parallel_grundy::grundy(&graph, grundy_cache)
    );
}

fn test_path(n: usize, grundy_cache: &mut HashMap<u64, u64>) {
    let mut nodes = Vec::new();
    let mut graph: Graph<(), (), petgraph::Undirected> = Graph::new_undirected();
    for _ in 0..n {
        let node = graph.add_node(());
        nodes.push(node);
    }
    for i in 0..n - 1 {
        graph.add_edge(nodes[i], nodes[i + 1], ());
    }
    println!(
        "grundy of path of length {} is {}",
        n,
        grundy::grundy(&graph, grundy_cache)
    );
}

fn test_m_n(m: usize, n: usize, grundy_cache: &mut DashMap<u64, u64>) {
    let mut n_nodes = Vec::new();
    let mut m_nodes = Vec::new();
    let mut graph: Graph<(), (), petgraph::Undirected> = Graph::new_undirected();
    for _ in 0..m {
        let node = graph.add_node(());
        m_nodes.push(node);
    }
    for _ in 0..n {
        let node = graph.add_node(());
        n_nodes.push(node);
    }
    for i in 0..m {
        for j in 0..n {
            graph.add_edge(m_nodes[i], n_nodes[j], ());
        }
    }
    println!(
        "grundy of K_{}_{} is {}",
        m,
        n,
        parallel_grundy::grundy(&graph, grundy_cache)
    );
}

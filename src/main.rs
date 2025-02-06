pub mod graph_hash;
pub mod grundy;

use grundy::grundy;
use core::net;
use std::collections::HashMap;
use petgraph::graph::Graph;
fn main() {
    let mut grundy_cache = HashMap::new();
    for i in 1..=10 {
        for j in 1..=10 {
            test_m_n(i, j, &mut grundy_cache);
        }
    }
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
    println!("grundy of path of length {} is {}", n, grundy(&graph, grundy_cache));
}

fn test_m_n(m: usize, n: usize, grundy_cache: &mut HashMap<u64, u64>) {
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
    println!("grundy of K_{}_{} is {}", m, n, grundy(&graph, grundy_cache));
}

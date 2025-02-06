use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;


fn graph_hash(g: &Graph<(), (), Undirected>) -> u64 {
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

fn create_test_graph_1() -> Graph<(), (), Undirected> {
    let mut graph = Graph::<(), (), Undirected>::new_undirected();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());

    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, d, ());
    graph.add_edge(d, a, ());
    graph.add_edge(a, c, ());
    graph
}

fn create_test_graph_2() -> Graph<(), (), Undirected> {
    let mut graph = Graph::<(), (), Undirected>::new_undirected();
    let n0 = graph.add_node(());
    let n1 = graph.add_node(());
    let n2 = graph.add_node(());
    let n3 = graph.add_node(());

    graph.add_edge(n0, n2, ());
    graph.add_edge(n0, n1, ());
    graph.add_edge(n1, n3, ());
    graph.add_edge(n2, n3, ());
    graph.add_edge(n0, n3, ());
    graph
}

fn create_test_graph_3() -> Graph<(), (), Undirected> {
    let mut graph = Graph::<(), (), Undirected>::new_undirected();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());

    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());
    graph
}

fn create_test_graph_4() -> Graph<(), (), Undirected> {
    let mut graph = Graph::<(), (), Undirected>::new_undirected();
    graph.add_node(());
    graph.add_node(());
    graph.add_node(());
    graph.add_node(());
    graph
}

fn main() {
    let graph1 = create_test_graph_1();
    let hash1 = graph_hash(&graph1);
    println!("Test Graph 1 hash: {}", hash1);

    let graph2 = create_test_graph_2();
    let hash2 = graph_hash(&graph2);
    println!("Test Graph 2 (isomorphic to Graph 1) hash: {}", hash2);

    let graph3 = create_test_graph_3();
    let hash3 = graph_hash(&graph3);
    println!("Test Graph 3 (triangle) hash: {}", hash3);

    let graph4 = create_test_graph_4();
    let hash4 = graph_hash(&graph4);
    println!("Test Graph 4 (isolated nodes) hash: {}", hash4);

    if hash1 == hash2 {
        println!("Graph 1 and Graph 2 are likely isomorphic.");
    } else {
        println!("Graph 1 and Graph 2 hash differ, check the hash function.");
    }

    if hash1 == hash3 {
        println!("Graph 1 and Graph 3 are likely isomorphic.");
    }
    else {
        println!("Graph 1 and Graph 3 hash differ, check the hash function.");
    }
}

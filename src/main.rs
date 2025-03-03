pub mod graph_hash;
pub mod grundy;
pub mod grundy_cache;
pub mod parallel_grundy;
use petgraph::graph::Graph;

fn main() {
    let mut grundy_cache = grundy_cache::GrundyCache::new();
    for i in 5..=5 {
        for j in 1..=7 {
            test_m_n(i, j, &mut grundy_cache);
        }
    }
    println!("collision rate is {}%", grundy_cache.collision_rate());
}

// fn test_complete_graph(n: usize, grundy_cache: &mut grundy_cache::GrundyCache) {
//     let mut nodes = Vec::new();
//     let mut graph: Graph<(), (), petgraph::Undirected> = Graph::new_undirected();
//     for _ in 0..n {
//         let node = graph.add_node(());
//         nodes.push(node);
//     }
//     for i in 0..n - 1 {
//         for j in i + 1..n {
//             graph.add_edge(nodes[i], nodes[j], ());
//         }
//     }
//     println!(
//         "grundy of complete graph of size {} is {}",
//         n,
//         grundy::grundy(&graph, grundy_cache)
//     );
// }

fn test_m_n(m: usize, n: usize, grundy_cache: &mut grundy_cache::GrundyCache) {
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
        grundy::grundy(&graph, grundy_cache)
    );
}

pub mod graph_hash;
pub mod grundy;
pub mod grundy_cache;
pub mod parallel_grundy;
use petgraph::graph::Graph;

fn main() {
    let mut grundy_cache = grundy_cache::GrundyCache::new();
    for n in 4..=4 {
        for m in 8..=8 {
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
                n,
                m,
                parallel_grundy::grundy(&graph, &mut grundy_cache)
            );
        }
    }
}

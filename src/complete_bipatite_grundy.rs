use crate::grundy_cache::GrundyCache;
use petgraph::graph::Graph;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;

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
fn separate_group_in_bipartite_graph(g: &Graph<(), (), petgraph::Undirected>) -> Vec<Vec<i32>> {
    let mut color = vec![-1; g.node_count()];
    let mut queue = std::collections::VecDeque::new();
    let mut a = vec![];
    let mut b = vec![];
    for start in g.node_indices() {
        if color[start.index()] == -1 {
            color[start.index()] = 0;
            queue.push_back(start);
            a.push(start);
            while let Some(node) = queue.pop_front() {
                for neighbor in g.neighbors(node) {
                    if color[neighbor.index()] == -1 {
                        color[neighbor.index()] = 1 - color[node.index()];
                        queue.push_back(neighbor);
                        if color[neighbor.index()] == 0 {
                            a.push(neighbor);
                        } else {
                            b.push(neighbor);
                        }
                    } else if color[neighbor.index()] == color[node.index()] {
                        panic!("Graph is not bipartite");
                    }
                }
            }
        }
    }
    let mut matrix = vec![vec![1; b.len()]; a.len()];
    for i in 0..a.len() {
        for j in 0..b.len() {
            if g.edges_connecting(a[i], b[j]).count() == 0 {
                matrix[i][j] = 0;
            }
        }
    }
    matrix
}
pub fn grundy(
    g: &Graph<(), (), petgraph::Undirected>,
    grundy_cache: &mut GrundyCache,
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
        rev_mex.insert(grundy(&tmp, grundy_cache, depth + 1));
    }
    for node in g.node_indices() {
        let mut tmp = g.clone();
        if g.neighbors(node).count() == 0 {
            continue;
        }
        tmp.remove_node(node);
        rev_mex.insert(grundy(&tmp, grundy_cache, depth + 1));
    }
    let mut mex = 0;
    while rev_mex.contains(&mex) {
        mex += 1;
    }
    grundy_cache.insert(&g, mex);
    if mex == 0 {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("src/output.txt")
            .unwrap();
        file.write_all(format!("depth: {depth}, mex: {mex}\n").as_bytes())
            .unwrap();
        for row in &separate_group_in_bipartite_graph(&g) {
            for elem in row {
                file.write_all(format!("{}", elem).as_bytes()).unwrap();
            }
            file.write_all(format!("\n").as_bytes()).unwrap();
        }
        drop(file);
    }
    mex
}

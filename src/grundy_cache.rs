use crate::graph_hash::graph_hash;
use dashmap::DashMap;
use petgraph::graph::Graph;
struct CacheNode {
    g: Graph<(), (), petgraph::Undirected>,
    grundy: u64,
}
pub struct GrundyCache {
    cache: DashMap<u64, Vec<CacheNode>>,
}
impl GrundyCache {
    pub fn new() -> GrundyCache {
        GrundyCache {
            cache: DashMap::new(),
        }
    }
    pub fn insert(&self, g: Graph<(), (), petgraph::Undirected>, grundy: u64) {
        let key = graph_hash(&g);
        if !self.cache.contains_key(&key) {
            self.cache.insert(key, Vec::new());
        }
        for node in self.cache.get_mut(&key).unwrap().iter() {
            if petgraph::algo::is_isomorphic(&node.g, &g) {
                if grundy != node.grundy {
                    panic!(
                        "Problem: Same graph but different grundy. Nodes:  {}, {}, Edges: {}, {}, Grundy: {}, {}",
                        node.g.node_count(),
                        g.node_count(),
                        node.g.edge_count(),
                        g.edge_count(),
                        grundy,
                        node.grundy,
                    );
                }
                return;
            }
        }
        self.cache.get_mut(&key).unwrap().push(CacheNode {
            g: g.clone(),
            grundy,
        });
    }
    pub fn get(&self, g: &Graph<(), (), petgraph::Undirected>) -> i64 {
        let key = graph_hash(g);
        if !self.cache.contains_key(&key) {
            return -1;
        }
        for node in self.cache.get(&key).unwrap().iter() {
            if petgraph::algo::is_isomorphic(&node.g, g) {
                return node.grundy as i64;
            }
        }
        -1
    }
    pub fn collision_rate(&self) -> f64 {
        let mut total = 0;
        let mut collisions = 0;
        for node in self.cache.iter() {
            total += node.value().len();
            collisions += 1;
        }
        (total as f64 / collisions as f64 - 1.0) * 100.0
    }
}

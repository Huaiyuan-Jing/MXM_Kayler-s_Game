use crate::graph_hash::graph_hash;
use petgraph::graph::Graph;
use dashmap::DashMap;
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
        for node in self.cache.get_mut(&key).unwrap().iter_mut() {
            if petgraph::algo::is_isomorphic(&node.g, &g) {
                if grundy != node.grundy {
                    panic!("Problem: Same graph but different grundy");
                }
                return;
            }
        }
        self.cache.get_mut(&key).unwrap().push(CacheNode { g, grundy });
    }
    pub fn get(&self, g: &Graph<(), (), petgraph::Undirected>) -> i64 {
        let key = graph_hash(g);
        if self.cache.contains_key(&key) {
            for node in self.cache.get(&key).unwrap().iter() {
                if petgraph::algo::is_isomorphic(&node.g, g) {
                    return node.grundy as i64;
                }
            }
        }
        -1
    }
}

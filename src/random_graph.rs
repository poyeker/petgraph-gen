use petgraph::algo::*;
use petgraph::prelude::*;

use crate::classic::ToCompleteGraph;
use crate::common::init_graph;
use itertools::{repeat_n, Itertools};
use simple_rand::*;

pub trait ToWattsStrogatzGraph<U: Clone> {
    fn to_watts_strogatz_graph(&mut self, k: usize, p: f64, weight: U);
}

impl<T, U: Clone> ToWattsStrogatzGraph<U> for UnGraph<T, U> {
    fn to_watts_strogatz_graph(&mut self, k: usize, p: f64, weight: U) {
        self.clear_edges();
        let n = self.node_count();
        if k == n {
            self.to_complete_graph(weight);
        } else {
            for i in 0..n {
                for j in 0..(k / 2) {
                    let a = self.node_indices().nth(i).unwrap();
                    let b = self.node_indices().nth((i + 1 + j) % n).unwrap();
                    self.add_edge(a, b, weight.clone());
                }
            }
            let mut rng = Rand::<Pcg64Mcg>::default();
            for j in 1..(k / 2 + 1) {
                let target = self
                    .node_indices()
                    .skip(j)
                    .chain(self.node_indices().take(j));
                for (u, v) in self.node_indices().zip(target) {
                    if rng.rand_float(0., 1.) < p {
                        let mut w = rng.one_of(&self.node_indices().collect::<Vec<_>>());
                        while w == u || self.contains_edge(w, u) {
                            w = rng.one_of(&self.node_indices().collect::<Vec<_>>());
                            if self.neighbors(u).count() > n - 1 {
                                break;
                            }
                        }

                        self.remove_edge(self.find_edge(u, v).unwrap());
                        self.add_edge(u, w, weight.clone());
                    }
                }
            }
        }
    }
}

pub fn watts_strogatz_graph<T: Default, U: Clone + Default>(
    n: usize,
    k: usize,
    p: f64,
) -> UnGraph<T, U> {
    let mut g = init_graph(n);
    g.to_watts_strogatz_graph(k, p, U::default());
    g
}

pub fn connected_watts_strogatz_graph<T: Default, U: Clone + Default>(
    n: usize,
    k: usize,
    p: f64,
    tries: usize,
) -> Result<UnGraph<T, U>, String> {
    for _ in 0..tries {
        let g: UnGraph<T, U> = watts_strogatz_graph(n, k, p);
        if connected_components(&g) == 1 {
            return Ok(g);
        }
    }
    return Err("Construction of graph failed!".into());
}

pub trait ToErdosRenyiGraph<U: Clone> {
    fn to_erdos_renyi_graph(&mut self, p: f64, weight: U);
}

impl<T, U: Clone> ToErdosRenyiGraph<U> for UnGraph<T, U> {
    fn to_erdos_renyi_graph(&mut self, p: f64, weight: U) {
        self.clear_edges();
        let mut rng = Rand::<Pcg64Mcg>::default();
        for pair in self.node_indices().combinations(2) {
            if rng.rand_float(0., 1.) <= p {
                self.add_edge(pair[0], pair[1], weight.clone());
            }
        }
    }
}

pub fn erdos_renyi_graph<T: Default, U: Clone + Default>(n: usize, p: f64) -> UnGraph<T, U> {
    let mut g = init_graph(n);
    g.to_erdos_renyi_graph(p, U::default());
    g
}

pub fn barabasi_albert_graph<T: Default, U: Clone + Default>(n: usize, m: usize) -> UnGraph<T, U> {
    let mut g = init_graph::<T, U>(0);
    let mut targets = (0..m).map(|_| g.add_node(T::default())).collect::<Vec<_>>();
    let mut repeated_nodes: Vec<NodeIndex> = vec![];
    let mut source = g.add_node(T::default());
    let mut rng = Rand::<Pcg64Mcg>::default();
    while g.node_count() <= n {
        for (a, &b) in repeat_n(source, m).zip(targets.iter()) {
            g.add_edge(a, b, U::default());
        }
        repeated_nodes.extend(targets);
        repeated_nodes.extend(repeat_n(source, m));
        targets = rng.n_of(&repeated_nodes, m);
        source = g.add_node(T::default());
    }
    g
}

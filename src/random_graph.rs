use petgraph::prelude::*;
use petgraph::algo::*;
use petgraph::visit::*;
use simple_rand::*;
use crate::classic::ToCompleteGraph;
use crate::common::init_graph;

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
            let mut rng = make_rng!();
            for j in 1..(k / 2 + 1) {
                let target = self.node_indices().skip(j).chain(self.node_indices().take(j));
                for (u, v) in self.node_indices().zip(target) {
                    if rng.rand_float(0., 1.) < p {
                        let mut w = rng.choice(self.node_indices());
                        while w == u || self.contains_edge(w, u) {
                            w = rng.choice(self.node_indices());
                            if self.neighbors(u).count() > n - 1 { break; }
                        }

                        self.remove_edge(self.find_edge(u, v).unwrap());
                        self.add_edge(u, w, weight.clone());
                    }
                }
            }
        }
    }
}

pub fn watts_strogatz_graph<T: Default, U: Clone>(n: usize, k: usize, p: f64, weight: U) -> UnGraph<T, U> {
    let mut g = init_graph(n);
    g.to_watts_strogatz_graph(k, p, weight.clone());
    g
}

pub fn connected_watts_strogatz_graph<T: Default, U: Clone>(n: usize, k: usize, p: f64, weight: U, tries: usize) -> Result<UnGraph<T, U>, String> {
    for _ in 0..tries {
        let mut g: UnGraph<T, U> = watts_strogatz_graph(n, k, p, weight.clone());
        if connected_components(&g) == 1 { return Ok(g); }
    }
    return Err("Construction of graph failed!".into());
}

pub trait ToErdosRenyiGraph<U: Clone> {
    fn to_erdos_renyi_graph(&mut self, p: f64, weight: U);
}

impl<T, U: Clone> ToErdosRenyiGraph<U> for UnGraph<T, U> {
    fn to_erdos_renyi_graph(&mut self, p: f64, weight: U) {
        unimplemented!();
    }
}

pub fn erdos_renyi_graph<T: Default, U: Clone>(n: usize, p: f64, weight: U) -> UnGraph<T, U> { todo!() }

pub fn barabasi_albert_graph<T: Default, U: Clone>(n: usize, m: usize, weight: U) -> UnGraph<T, U> {
    todo!()
}
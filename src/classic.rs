use crate::common;
use petgraph::prelude::*;

pub trait ToEmptyGraph<U: Clone> {
    fn to_empty_graph(&mut self, weight: U);
}

impl<T, U: Clone> ToEmptyGraph<U> for UnGraph<T, U> {
    fn to_empty_graph(&mut self, weight: U) {
        self.clear_edges();
    }
}

pub fn empty_graph<T: Default, U: Clone + Default>(n: usize) -> UnGraph<T, U> {
    let mut g = common::init_graph(n);
    g
}

pub trait ToCompleteGraph<U: Clone> {
    fn to_complete_graph(&mut self, weight: U);
}

impl<T, U: Clone> ToCompleteGraph<U> for UnGraph<T, U> {
    fn to_complete_graph(&mut self, weight: U) {
        self.clear_edges();
        for (i, ni) in self.node_indices().rev().skip(1).rev().enumerate() {
            for nj in self.node_indices().skip(i + 1) {
                self.add_edge(ni, nj, weight.clone());
            }
        }
    }
}

pub fn complete_graph<T: Default, U: Clone + Default>(n: usize) -> UnGraph<T, U> {
    let mut g = common::init_graph(n);
    g.to_complete_graph(U::default());
    g
}

pub trait ToRing<U: Clone> {
    fn to_ring(&mut self, weight: U);
}

impl<T, U: Clone> ToRing<U> for UnGraph<T, U> {
    fn to_ring(&mut self, weight: U) {
        self.clear_edges();
        let mut right_node = self.node_indices().skip(1);
        for i in self.node_indices() {
            if let Some(r) = right_node.next() {
                self.add_edge(i, r, weight.clone());
            } else {
                self.add_edge(i, self.node_indices().next().unwrap(), weight.clone());
            }
        }
    }
}

pub fn ring<T: Default, U: Clone + Default>(n: usize) -> UnGraph<T, U> {
    let mut g = common::init_graph(n);
    g.to_ring(U::default());
    g
}

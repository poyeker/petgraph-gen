use petgraph::prelude::*;
use crate::common::*;

pub trait ToGrid2DGraph<U: Clone> {
    fn to_grid2d_graph(&mut self, m: usize, n: usize, periodic: bool, weight: U);
}

impl<T, U: Clone> ToGrid2DGraph<U> for UnGraph<T, U> {
    fn to_grid2d_graph(&mut self, m: usize, n: usize, periodic: bool, weight: U) {
        assert_eq!(
            m * n,
            self.node_count(),
            "m*n={},node_count={}",
            m * n,
            self.node_count()
        );
        self.clear_edges();
        for i in 0..m - 1 {
            for j in 0..n {
                let a = self.node_indices().nth(i * n + j).unwrap();
                let b = self.node_indices().nth((i + 1) * n + j).unwrap();
                self.add_edge(a, b, weight.clone());
            }
        }
        for i in 0..m {
            for j in 0..n - 1 {
                let a = self.node_indices().nth(i * n + j).unwrap();
                let b = self.node_indices().nth(i * n + j + 1).unwrap();
                self.add_edge(a, b, weight.clone());
            }
        }
        if periodic {
            for i in 0..m {
                let a = self.node_indices().nth(i * n).unwrap();
                let b = self.node_indices().nth(i * n + n - 1).unwrap();
                self.add_edge(a, b, weight.clone());
            }
            for j in 0..n {
                let a = self.node_indices().nth(j).unwrap();
                let b = self.node_indices().nth((m-1) * n + j).unwrap();
                self.add_edge(a, b, weight.clone());
            }
        }
    }
}

pub fn grid_2d_graph<T: Default, U: Clone>(m: usize, n: usize, periodic: bool, weight: U) -> UnGraph<T, U> {
    let mut g = init_graph(m * n);
    g.to_grid2d_graph(m, n, periodic, weight);
    g
}





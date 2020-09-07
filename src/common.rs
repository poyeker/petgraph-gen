use petgraph::prelude::*;

trait ClearEdges {
    fn clear_edges(&mut self);
}

impl<T, U> ClearEdges for Graph<T, U> {
    fn clear_edges(&mut self) {
        self.edge_indices().for_each(|e| {
            self.remove_edge(e);
        });
    }
}

pub(crate) fn init_graph<T: Default, U: Clone>(n: usize) -> Graph<T, U, Undirected, u32> {
    let mut g = UnGraph::<T, U>::new_undirected();
    for _ in 0..n {
        g.add_node(Default::default());
    }
    g
}

pub(crate) fn init_digraph<T: Default, U: Clone>(n: usize) -> DiGraph<T, U, u32> {
    let mut g = DiGraph::<T, U, u32>::new();
    for _ in 0..n {
        g.add_node(Default::default());
    }
    g
}

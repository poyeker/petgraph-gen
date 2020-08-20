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


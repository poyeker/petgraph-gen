use petgraph_gen::lattice::grid_2d_graph;
use petgraph_gen::classic::complete_graph;
use petgraph_gen::random_graph::{ToWattsStrogatzGraph, watts_strogatz_graph, connected_watts_strogatz_graph};

#[test]
fn it_works() {
    let mut g = connected_watts_strogatz_graph::<u32,()>(1000,2,0.2,(),100);
    eprintln!("g = {:#?}", g);
}
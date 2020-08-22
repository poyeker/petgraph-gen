#[test]
fn it_works() {
    let g = petgraph_gen::random_graph::barabasi_albert_graph::<(), ()>(400000, 2);
}

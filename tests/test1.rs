#[test]
fn it_works() {
    let g = petgraph_gen::random_graph::barabasi_albert_graph::<(),()>(4,1,());
    eprintln!("g = {:#?}", g);
}
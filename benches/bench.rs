#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn bench_ring(b: &mut Bencher) {
    b.iter(|| petgraph_gen::random_graph::barabasi_albert_graph::<(),()>(10000,2));
}

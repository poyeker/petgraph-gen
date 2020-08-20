#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn bench_ring(b: &mut Bencher) {
    b.iter(|| petgraph_gen::classic::ring::<u32, _>(10000, ()));
}

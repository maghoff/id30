use std::io::Write;

use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use id30::Id30;

fn display_u32(b: &mut Bencher) {
    let mut buf = Vec::with_capacity(8);
    let candidates: Vec<u32> = (0..100).map(|_| rand08::random::<Id30>().into()).collect();

    b.iter(|| {
        for candidate in &candidates {
            buf.clear();
            write!(&mut buf, "{}", candidate).unwrap();
            black_box(&buf);
        }
    });
}

fn display(b: &mut Bencher) {
    let mut buf = Vec::with_capacity(8);
    let candidates: Vec<Id30> = (0..100).map(|_| rand08::random()).collect();

    b.iter(|| {
        for candidate in &candidates {
            buf.clear();
            write!(&mut buf, "{}", candidate).unwrap();
            black_box(&buf);
        }
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("display_u32", display_u32);
    c.bench_function("display", display);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

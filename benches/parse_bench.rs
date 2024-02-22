use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use id30::{Id30, Id30Parse};

fn parse_id30(b: &mut Bencher) {
    let candidates: Vec<_> = (0..100)
        .map(|_| rand08::random::<Id30>().to_string())
        .collect();

    b.iter(|| {
        for candidate in &candidates {
            let id30: Id30 = candidate.parse().unwrap();
            black_box(id30);
        }
    });
}

fn parse_id30parse(b: &mut Bencher) {
    let candidates: Vec<_> = (0..100)
        .map(|_| rand08::random::<Id30>().to_string())
        .collect();

    b.iter(|| {
        for candidate in &candidates {
            let id30: Id30Parse = candidate.parse().unwrap();
            black_box(id30);
        }
    });
}

fn parse_u32(b: &mut Bencher) {
    let candidates: Vec<_> = (0..100)
        .map(|_| u32::from(rand08::random::<Id30>()).to_string())
        .collect();

    b.iter(|| {
        for candidate in &candidates {
            let i: u32 = candidate.parse().unwrap();
            black_box(i);
        }
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_u32", parse_u32);
    c.bench_function("parse_id30", parse_id30);
    c.bench_function("parse_id30parse", parse_id30parse);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

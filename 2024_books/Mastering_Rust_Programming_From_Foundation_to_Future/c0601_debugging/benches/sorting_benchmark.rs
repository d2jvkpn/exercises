use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};

use c0601_debugging::sorting::*;

fn sort_array(c: &mut Criterion) {
    let mut arr = black_box([6, 2, 4, 1, 9, -2, 5]);

    c.bench_function("sorting bubble algorithm", |b: &mut Bencher| b.iter(|| bubble(&mut arr)));

    c.bench_function("sorting selection algorithm", |b: &mut Bencher| {
        b.iter(|| selection(&mut arr))
    });

    c.bench_function("sorting insertion algorithm", |b: &mut Bencher| {
        b.iter(|| insertion(&mut arr))
    });
}

criterion_group!(benches, sort_array);
criterion_main!(benches);

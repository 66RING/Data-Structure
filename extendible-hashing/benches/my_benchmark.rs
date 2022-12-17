use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use extendible_hashing::{ExtendiableHash, Mode};

type K = u64;
type V = u64;
const SET_SIZE: usize = 1 << 10;
fn create_workload() -> Vec<(K, V)> {
    let mut v = vec![];
    v.resize(SET_SIZE, (0, 0));
    for i in 1..SET_SIZE {
        v[i] = (i as K, i as V);
    }
    v
}

fn stress_crate_no(workload: &Vec<(K, V)>) {
    let mut m = ExtendiableHash::new(2, 32);
    for (k, v) in workload {
        m.insert(k, v);
    }
    for (k, _) in workload {
        m.remove(&k, Mode::No);
    }
}
fn stress_crate_merge(workload: &Vec<(K, V)>) {
    let mut m = ExtendiableHash::new(2, 32);
    for (k, v) in workload {
        m.insert(k, v);
    }
    for (k, _) in workload {
        m.remove(&k, Mode::Merge);
    }
}
fn stress_crate_shrink(workload: &Vec<(K, V)>) {
    let mut m = ExtendiableHash::new(2, 32);
    for (k, v) in workload {
        m.insert(k, v);
    }
    for (k, _) in workload {
        m.remove(&k, Mode::Shrink);
    }
}
fn stress_std(workload: &Vec<(K, V)>) {
    let mut m = HashMap::new();
    for (k, v) in workload {
        m.insert(k, v);
    }
    for (k, _) in workload {
        m.remove(&k);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let workload = create_workload();
    c.bench_function("crate: no mode", |b| b.iter(|| stress_crate_no(black_box(&workload))));
    c.bench_function("crate: merge mode", |b| b.iter(|| stress_crate_merge(black_box(&workload))));
    c.bench_function("crate: shrink mode", |b| b.iter(|| stress_crate_shrink(black_box(&workload))));
    c.bench_function("std", |b| b.iter(|| stress_std(black_box(&workload))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

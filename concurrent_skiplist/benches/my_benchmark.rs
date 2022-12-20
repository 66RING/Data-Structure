use criterion::{black_box, criterion_group, criterion_main, Criterion};
use concurrent_skiplist::safe_array_rcrefcell_skiplist::Skiplist as SafeARRSkiplist;
use concurrent_skiplist::safe_array_rc_skiplist::Skiplist as SafeARSkiplist;
use concurrent_skiplist::unsafe_array_skiplist::Skiplist as UnsafeASkiplst;
use concurrent_skiplist::safe_list_skiplist::Skiplist as SafeLSkiplst;

// use concurrent_skiplist::skiplist_jj::Skiplist as SkiplistJ;

type K = i32;
type V = i32;
const SET_SIZE: usize = 1 << 10;
const BUKCET_SIZE: usize = 1000;

fn create_workload() -> Vec<(K, V)> {
    let mut v = vec![];
    v.resize(SET_SIZE, (0, 0));
    for i in 1..SET_SIZE {
        v[i] = (i as K, i as V);
    }
    v
}

fn safe_list_base(workload: &Vec<(K, V)>) {
    let mut s = SafeLSkiplst::new();
    for (k, _) in workload {
        s.add(*k);
    }
    for (k, _) in workload {
        s.search(*k);
    }
    for (k, _) in workload {
        s.erase(*k);
    }
}

fn safe_array_rcrefcell_base(workload: &Vec<(K, V)>) {
    let mut s = SafeARRSkiplist::new();
    for (k, _) in workload {
        s.add(*k);
    }
    for (k, _) in workload {
        s.search(*k);
    }
    for (k, _) in workload {
        s.erase(*k);
    }
}

fn safe_array_rc_base(workload: &Vec<(K, V)>) {
    let mut s = SafeARSkiplist::new();
    for (k, _) in workload {
        s.add(*k);
    }
    for (k, _) in workload {
        s.search(*k);
    }
    for (k, _) in workload {
        s.erase(*k);
    }
}

fn unsafe_array__base(workload: &Vec<(K, V)>) {
    let mut s = UnsafeASkiplst::new();
    for (k, _) in workload {
        s.add(*k);
    }
    for (k, _) in workload {
        s.search(*k);
    }
    for (k, _) in workload {
        s.erase(*k);
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    let workload = create_workload();
    c.bench_function("safe rc refcell array base", |b| b.iter(|| safe_array_rcrefcell_base(black_box(&workload))));
    c.bench_function("safe rc array base", |b| b.iter(|| safe_array_rc_base(black_box(&workload))));
    c.bench_function("unsafe array base", |b| b.iter(|| unsafe_array__base(black_box(&workload))));
    c.bench_function("safe list base", |b| b.iter(|| safe_list_base(black_box(&workload))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


use extendible_hashing::{Mode,ExtendiableHash};

const SET_SIZE: usize = 1 << 10;
type K = u64;
type V = u64;

#[test]
fn stress_crate_no() {
    let mut workload = vec![];
    workload.resize(SET_SIZE, (0, 0));
    for i in 1..SET_SIZE {
        workload[i] = (i as K, i as V);
    }

    let mut m = ExtendiableHash::new(2, 32);
    for (k, v) in &workload {
        m.insert(k, v);
    }
    for (k, _) in &workload {
        m.remove(&k, Mode::No);
    }
}


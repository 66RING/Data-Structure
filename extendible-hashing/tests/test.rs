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
        m.insert(k.clone(), v.clone());
    }
    for (k, v) in &workload {
        assert_eq!(&*m.get(k).unwrap(), v);
    }
    for (k, _) in &workload {
        m.remove(k, Mode::No);
    }
    for (k, _) in &workload {
        assert!(m.get(k).is_none());
    }
}

#[test]
fn stress_crate_merge() {
    let mut workload = vec![];
    workload.resize(SET_SIZE, (0, 0));
    for i in 1..SET_SIZE {
        workload[i] = (i as K, i as V);
    }

    let mut m = ExtendiableHash::new(2, 32);
    for (k, v) in &workload {
        m.insert(k.clone(), v.clone());
    }
    for (k, v) in &workload {
        assert_eq!(&*m.get(k).unwrap(), v);
    }
    for (k, _) in &workload {
        m.remove(k, Mode::Merge);
    }
    for (k, _) in &workload {
        assert!(m.get(k).is_none());
    }
}

#[test]
fn stress_crate_shrink() {
    let mut workload = vec![];
    workload.resize(SET_SIZE, (0, 0));
    for i in 1..SET_SIZE {
        workload[i] = (i as K, i as V);
    }

    let mut m = ExtendiableHash::new(2, 32);
    for (k, v) in &workload {
        m.insert(k.clone(), v.clone());
    }
    for (k, v) in &workload {
        assert_eq!(&*m.get(k).unwrap(), v);
    }
    for (k, _) in &workload {
        m.remove(k, Mode::Shrink);
    }
    for (k, _) in &workload {
        assert!(m.get(k).is_none());
    }
}

use concurrent_skiplist::unsafe_skiplist::Skiplist as ArraySkiplist;
use concurrent_skiplist::safe_list_skiplist::Skiplist as ListSkiplist;

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

#[test]
fn list_base() {
    let workload = &create_workload();
    let mut s = ListSkiplist::new();
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

#[test]
fn array_base() {
    let workload = &create_workload();
    let mut s = ArraySkiplist::new();
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





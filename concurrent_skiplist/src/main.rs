use concurrent_skiplist::safe_array_rcrefcell_skiplist::Skiplist;
fn main() {
    // ["Skiplist", "add", "add", "add", "search", "add", "search", "erase", "erase", "search"]
    // [[], [1], [2], [3], [0], [4], [1], [0], [1], [1]]

    let mut s = Skiplist::new();
    s.add(4);
    s.add(3);
    s.add(2);
    println!("{}", s.search(0));
    s.add(1);
    println!("{}", s.search(1));
    println!("{}", s.erase(0));
    println!("f {}", s.search(5));
    println!("f {}", s.erase(5));
    println!("{}", s.search(4));
    println!("{}", s.erase(4));
    println!("{}", s.erase(1));
    println!("{}", s.search(1));
    s.display();
}

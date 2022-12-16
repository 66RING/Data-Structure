use std::io;
use extendible_hashing::{Directory, Mode};


fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("error input\n");
    let bucket_cap = line.trim().parse::<usize>().expect("not a num");

    line.clear();
    io::stdin().read_line(&mut line).expect("error input\n");
    let global_depth = line.trim().parse::<usize>().expect("not a num");
    // println!("bucket_size: {} , global_depth: {}", bucket_cap, global_depth);

    let mut dir = Directory::new(global_depth, bucket_cap);

    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("error input\n");
        let mut words = line.trim().split_whitespace();
        let cmd = words.next().unwrap();
        match cmd {
            "insert" => {
                let key = words.next().unwrap().parse::<i64>().unwrap();
                let value = words.next().unwrap().parse::<i64>().unwrap();
                dir.insert(key, value);
            },
            "delete" => {
                let key = words.next().unwrap().parse::<i64>().unwrap();
                dir.remove(&key, Mode::Shrink);
            },
            "update" => {
                let key = words.next().unwrap().parse::<i64>().unwrap();
                let value = words.next().unwrap().parse::<i64>().unwrap();
                dir.update(key, value);
            },
            "search" => {
                let key = words.next().unwrap().parse::<i64>().unwrap();
                // let mut value = String::new();
                let mut value: i64 = -1;
                // dir.search(&key, &mut value);
                println!("{}", value);
            },
            "display" => {
                dir.display();
                // println!("display");
            },
            "exit" => {
                break;
            },
            _ => {
                println!("invalid command: {}", cmd);
            },
        }
    }
}

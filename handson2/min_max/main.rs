use core::panic;
use std::fs;

mod min_max_tree;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments")
    }

    // This simple harness reads input from a file and parses it
    // No error checking is done since I didn't think it was necessary to overcomplicate the harness

    let contents = fs::read_to_string(&args[1]).expect("Failed reading file");

    let mut lines = contents.lines();
    let (_n, _m) = lines.next().unwrap().split_once(" ").unwrap();
    let _n: usize = _n.parse().unwrap();
    let _m: usize = _m.parse().unwrap();

    // Here we build the starting data vector
    let arr: Vec<u32> = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|a| a.parse().unwrap())
        .collect();

    // And use it to initialize our segment tree
    let mut tree = min_max_tree::MinMaxTree::from_vec(arr);

    // Now we parse the queries
    for l in lines {
        let cur: Vec<u32> = l.split(" ").map(|a| a.parse().unwrap()).collect();
        let cmd = cur[0];
        //println!("{cur:?}");
        match cmd {
            0 => {
                tree.update(
                    cur[1].try_into().unwrap(),
                    cur[2].try_into().unwrap(),
                    cur[3],
                );
            }
            1 => println!(
                "{}",
                tree.max(cur[1].try_into().unwrap(), cur[2].try_into().unwrap())
            ),
            _ => {
                panic!("Unexpected command!")
            }
        }
    }
}

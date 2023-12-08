use crate::is_there_tree::IsThereTree;
use std::fs;

mod is_there_tree;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments")
    }
    // This simple harness reads input from a file and parses it
    // No error checking is done since I didn't think it was necessary to overcomplicate the harness

    let contents = fs::read_to_string(&args[1]).expect("Failed reading file");

    let n: &str;
    let m: &str;
    let mut lines = contents.lines();
    (n, m) = lines.next().unwrap().split_once(" ").unwrap();
    let n: usize = n.parse().unwrap();
    let _m: usize = m.parse().unwrap();

    // Here we read all the ranges
    let arr: Vec<(usize, usize)> = lines
        .take(n)
        .map(|a| {
            let mut it = a.split(" ");
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    // Once we have the ranges we initialize the segment tree
    let tree = IsThereTree::from_vec(arr, n);

    // And start parsing the querys
    for l in contents.lines().skip(n + 1) {
        let cur: Vec<usize> = l.split(" ").map(|a| a.parse().unwrap()).collect();
        println!(
            "{}",
            if tree.is_there(cur[0], cur[1], cur[2].try_into().unwrap()) {
                1
            } else {
                0
            }
        );
    }
}

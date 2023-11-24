use core::panic;
use std::fs;

mod min_max_tree;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments")
    }
    let contents = fs::read_to_string(&args[1]).expect("Failed reading file");

    let n: &str;
    let m: &str;
    let mut lines = contents.lines();
    (n, m) = lines.next().unwrap().split_once(" ").unwrap();
    let n: usize = n.parse().unwrap();
    let m: usize = m.parse().unwrap();

    let arr: Vec<u32> = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|a| a.parse().unwrap())
        .collect();
    //println!("{arr:?}");
    let mut tree = min_max_tree::MinMaxTree::from_vec(arr);
    //println!("{tree:?}");
    for l in lines {
        let cur: Vec<u32> = l.split(" ").map(|a| a.parse().unwrap()).collect();
        let cmd = cur[0];
        //println!("{cur:?}");
        match cmd {
            0 => tree.update(
                cur[1].try_into().unwrap(),
                cur[2].try_into().unwrap(),
                cur[3],
            ),
            1 => println!(
                "{}",
                tree.max(cur[1].try_into().unwrap(), cur[2].try_into().unwrap())
            ),
            _ => {
                panic!("Unexpected command!")
            }
        }
    }
    //println!("{contents}");
}

use std::io;

fn main() {
    let mut n = String::new();
    println!("Enter the number of elements in the array: ");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please insert a number");
    let mut test_input: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = input.trim().parse().expect("Please insert a number");
        test_input.push(input);
    }

    println!("Input: {:?} Leaders: {:?}", test_input, leader(&test_input));
}

fn leader(a: &[i32]) -> Vec<i32> {
    let mut max = a[a.len() - 1];
    let mut leaders: Vec<i32> = Vec::new();
    leaders.push(max);
    for el in a.iter().rev() {
        if *el > max {
            max = *el;
            leaders.push(max);
        }
    }
    return leaders;
}

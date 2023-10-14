fn main() {
    println!("Hello, world!");
    println!("{}", missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
}

fn missing_number(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let n = nums.len() as i32 + 1;

    return ((n * (n - 1)) / 2) - sum;
}

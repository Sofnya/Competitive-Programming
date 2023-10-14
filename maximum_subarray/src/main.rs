fn main() {
    let a = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum sum: {}", maximum_subarray(&a));
}

fn maximum_subarray(nums: &Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut cur_sum = 0;
    for el in nums {
        cur_sum += *el;
        if cur_sum > max {
            max = cur_sum;
        }
        if cur_sum < 0 {
            cur_sum = 0;
        }
    }

    return max;
}

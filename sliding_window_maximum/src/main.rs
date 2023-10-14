fn main() {
    let mut perm: Vec<usize> = Vec::new();
    let mut test_input = vec![1, 3, -1, -3, 5, 3, 6, 7];
    /**
       println!("{:?}", merge_sort(&test_input));
       for (a, b) in merge_sort(&test_input) {
           perm.push(b);
           println!("{} {}", a, b);
       }
       println!("{:?}", perm);
       println!("{:?}", permutate(&mut test_input, &mut perm));
    */
    println!(
        "{:?}",
        max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
    );
}

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut window_sorted: Vec<i32> = Vec::new();
    let mut permutation: Vec<usize> = Vec::new();

    let tmp = merge_sort(&nums[0..k as usize].to_vec());
    for (a, b) in tmp {
        permutation.push(b);
        window_sorted.push(a);
    }
    println!("Window: {:?}", &nums[0..k as usize]);
    println!("Perm: {permutation:?}\nSort:{window_sorted:?}");
    for i in 0..nums.len() {
        if i + k as usize > nums.len() {
            break;
        }
        let max = window_sorted[window_sorted.len() - 1];
        result.push(max);
        window_sorted.remove(permutation[0] - i);
        permutation.remove(0);
        println!("Max:{max} Window: {:?}", &nums[i..i + k as usize]);
        println!("Perm: {permutation:?}\nSort:{window_sorted:?}");

        let j = find_sorted_pos(&window_sorted, nums[i + k as usize]);
        window_sorted.insert(j, nums[i + k as usize]);
        permutation.push(j + i);

        println!("Perm: {permutation:?}\nSort:{window_sorted:?}");
    }

    return result;
}

fn merge_sort(arr: &Vec<i32>) -> Vec<(i32, usize)> {
    let mut a: Vec<(i32, usize)> = Vec::new();
    let mut b: Vec<(i32, usize)> = Vec::new();

    let mut i = 0;
    for el in arr {
        b.push((*el, i));
        a.push((*el, i));
        i += 1;
    }
    split(&mut a, 0, arr.len(), &mut b);
    return b;
}

fn merge(a: &mut Vec<(i32, usize)>, begin: usize, end: usize, b: &mut Vec<(i32, usize)>) {
    let middle = (begin + end) / 2;
    let mut i = begin;
    let mut j = middle;
    for k in begin..end {
        if i < middle && (j >= end || a[i].0 <= a[j].0) {
            b[k] = a[i];
            i += 1;
        } else {
            b[k] = a[j];
            j += 1;
        }
    }
}

fn split(a: &mut Vec<(i32, usize)>, begin: usize, end: usize, b: &mut Vec<(i32, usize)>) {
    if end - begin <= 1 {
        return;
    }
    let middle = (begin + end) / 2;
    split(b, begin, middle, a);
    split(b, middle, end, a);
    merge(a, begin, end, b);
}

fn permutate(a: &mut Vec<i32>, perm: &mut Vec<usize>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for _ in 0..a.len() {
        result.push(0);
    }
    for i in 0..a.len() {
        result[i] = a[perm[i]];
    }

    return result;
}

fn find_sorted_pos(a: &Vec<i32>, el: i32) -> usize {
    let mut i = 0;
    while i < a.len() && a[i] < el {
        i += 1;
    }
    return i;
}

use std::vec;

fn main() {
    println!("Hello, world!");
    let res = Solution::trap(vec![4, 2, 0, 3, 2, 5]);
    println!("{}", res);
}

struct Solution {}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut sum = 0;

        let mut l_edges = Self::leaders(height.clone());
        let mut r_edges: Vec<(usize, i32)> =
            Self::leaders(height.clone().into_iter().rev().collect())
                .iter()
                .map(|(i, h)| (height.len() - i - 1, *h))
                .rev()
                .collect();
        let mut l_ind = 0;
        let mut r_ind = 0;

        l_edges.push((height.len(), 0));
        r_edges.push((height.len(), 0));

        //println!("{:?}", height);
        //println!("{:?}", l_edges);
        //println!("{:?}", r_edges);

        for (i, h) in height.iter().enumerate() {
            //println!("{} {}", i, h);
            //println!("l_edges{:?} r_edges{:?}", l_edges[l_ind], r_edges[r_ind]);
            if l_ind >= l_edges.len() - 1 {
                break;
            }
            if i >= l_edges[l_ind + 1].0 {
                l_ind += 1;
            }
            if i >= r_edges[r_ind].0 {
                r_ind += 1;
            }
            if i < l_edges[l_ind].0 {
                //println!("continue");
                continue;
            }
            sum += std::cmp::max(std::cmp::min(l_edges[l_ind].1, r_edges[r_ind].1) - h, 0);
            //println!("sum:{sum}");
        }

        sum
    }
    fn leaders(height: Vec<i32>) -> Vec<(usize, i32)> {
        let mut leaders = Vec::new();
        let mut max = 0;

        for (i, el) in height.iter().enumerate() {
            if *el >= max {
                max = *el;
                leaders.push((i, *el));
            }
        }
        leaders
    }
}

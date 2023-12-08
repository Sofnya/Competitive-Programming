use std::collections::HashSet;

#[derive(Debug)]
struct Node {
    values: Option<HashSet<u32>>,
}

impl Node {
    fn new(values: Option<HashSet<u32>>) -> Self {
        Self { values }
    }
}

#[derive(Debug)]
pub struct IsThereTree {
    nodes: Vec<Node>,
    capacity: usize,
}
impl IsThereTree {
    fn left_child(id: usize) -> usize {
        id * 2 + 1
    }
    fn right_child(id: usize) -> usize {
        id * 2 + 2
    }

    // Returns the range of ids covered by a node with given id
    fn range(&self, id: usize) -> (usize, usize) {
        let max_height = self.capacity.ilog2();
        let height = max_height - (id + 1).ilog2();
        let pow = 1 << height;
        (id * pow + (pow - 1), id * pow + (pow - 1) * 2)
    }

    // Initializes the segment tree, by building the intermediate answers for every node
    fn initialize(&mut self, id: usize) -> Option<HashSet<u32>> {
        if id > self.capacity {
            return None;
        }
        match &self.nodes[id].values {
            None => {
                let left = self.initialize(Self::left_child(id));
                let right = self.initialize(Self::right_child(id));
                let res = match (left, right) {
                    // Since the tree is bigger than the actual underlying data, some nodes may remain uninitialized
                    (None, None) => None,
                    (None, Some(r)) => Some(r),
                    (Some(l), None) => Some(l),
                    (Some(l), Some(r)) => Some(l.union(&r).map(|&a| a).collect()),
                };
                self.nodes[id].values = res.clone();
                res
            }
            // The base case for leaves.
            Some(a) => Some(a.clone()),
        }
    }

    pub fn is_there(&self, i: usize, j: usize, k: u32) -> bool {
        return self.rec_is_there(0, (i + self.capacity / 2, j + self.capacity / 2), k);
    }

    fn rec_is_there(&self, id: usize, query_range: (usize, usize), k: u32) -> bool {
        let covered_range = self.range(id);
        let mid = (covered_range.1 - covered_range.0) / 2 + covered_range.0;

        if query_range == covered_range {
            match self.nodes[id].values.as_ref() {
                None => false,
                Some(h) => h.contains(&k),
            }
        } else if query_range.0 > covered_range.1 || query_range.1 < covered_range.0 {
            // If the query is outside of the node covered range just return the neutral false
            false
        } else {
            self.rec_is_there(
                Self::left_child(id),
                (query_range.0, usize::min(query_range.1, mid)),
                k,
            ) || self.rec_is_there(
                Self::right_child(id),
                (usize::max(query_range.0, mid + 1), query_range.1),
                k,
            )
        }
    }

    fn with_size(size: usize) -> Self {
        // We only support complete binary trees at the moment
        let capacity = size.next_power_of_two() * 2 - 1;
        let mut tree = Self {
            nodes: Vec::with_capacity(capacity),
            capacity,
        };
        for _ in 0..capacity {
            tree.nodes.push(Node::new(None))
        }
        tree
    }

    pub fn from_vec(ranges: Vec<(usize, usize)>, n: usize) -> Self {
        // We turn the vector of ranges in a vector of start/end points, indicated as (index,is_start) tuples
        let mut ranges: Vec<(usize, bool)> = ranges
            .iter()
            .map(|(l, r)| [(l, true), (r, false)])
            .flatten()
            .map(|(i, is_start)| (*i, is_start))
            .collect();
        // Sort it
        ranges.sort_unstable_by(|(i, is_start1), (j, is_start2)| {
            if i.cmp(j).is_eq() {
                is_start2.cmp(is_start1)
            } else {
                i.cmp(j)
            }
        });

        // Now we can build a vector going from 0 to n, with each point containing the number of overlapping segments
        let mut points: Vec<u32> = vec![0; n];
        let mut counter = 0;
        let mut last = 0;
        for (i, is_start) in ranges {
            if i != last {
                for j in last..i {
                    points[j] += counter;
                }
                last = i;
            }
            if is_start {
                counter += 1
            } else {
                // We need to increment the endpoints as we want the ranges to be inclusive
                points[i] += 1;
                counter -= 1
            };
        }

        // Once we have the underlying points vector, we can initialize the tree
        let mut tree = Self::with_size(points.len());
        for (i, val) in points.iter().enumerate() {
            tree.nodes[i + (tree.capacity / 2)].values = Some(HashSet::from([*val]));
        }
        tree.initialize(0);
        tree
    }
}

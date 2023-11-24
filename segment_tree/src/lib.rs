#[derive(Debug)]
struct Node {
    value: Option<i32>,
    id: usize,
}

impl Node {
    fn new(value: Option<i32>, id: usize) -> Self {
        Self { value, id }
    }
}

#[derive(Debug)]
struct SegmentTree {
    nodes: Vec<Node>,
    size: usize,
    capacity: usize,
}
impl SegmentTree {
    fn left_child(id: usize) -> usize {
        id * 2 + 1
    }
    fn right_child(id: usize) -> usize {
        id * 2 + 2
    }

    fn parent(id: usize) -> usize {
        (id - 1) / 2
    }

    fn range(&self, id: usize) -> (usize, usize) {
        let max_height = self.capacity.ilog2();
        let height = max_height - (id + 1).ilog2();
        let pow = (1 << height);
        (id * pow + (pow - 1), id * pow + (pow - 1) * 2)
    }

    fn access(&self, id: usize) -> Option<i32> {
        self.nodes[self.capacity / 2 + id].value
    }

    pub fn add(&mut self, id: usize, value: i32) {
        let mut cur = self.capacity / 2 + id;
        loop {
            match self.nodes[cur].value {
                None => self.nodes[cur].value = Some(value),
                Some(a) => self.nodes[cur].value = Some(a + value),
            };
            if cur == 0 {
                break;
            };

            cur = Self::parent(cur);
        }
    }

    pub fn sum(&self, i: usize, j: usize) -> i32 {
        return self.rec_sum(0, (i + self.capacity / 2, j + self.capacity / 2));
    }
    fn rec_sum(&self, id: usize, query_range: (usize, usize)) -> i32 {
        let covered_range = self.range(id);
        let mut res = 0;
        let mid = (covered_range.1 - covered_range.0) / 2 + covered_range.0;
        println!("Calculating sum{id}: {query_range:?} {covered_range:?} mid.{mid}");

        if query_range == covered_range {
            res = self.nodes[id].value.unwrap_or(0);
        } else if query_range.0 > covered_range.1 || query_range.1 < covered_range.0 {
            res = 0;
        } else {
            res = self.rec_sum(
                Self::left_child(id),
                (query_range.0, usize::min(query_range.1, mid)),
            ) + self.rec_sum(
                Self::right_child(id),
                (usize::max(query_range.0, mid + 1), query_range.1),
            );
        }
        println!("Returning sum{id}: {res}");
        res
    }

    pub fn with_size(size: usize) -> Self {
        let capacity = size.next_power_of_two() * 2 - 1;
        let mut tree = Self {
            nodes: Vec::with_capacity(capacity),
            size,
            capacity,
        };
        for i in 0..capacity {
            tree.nodes.push(Node::new(None, i))
        }
        tree
    }

    pub fn from_vec(vec: Vec<i32>) -> Self {
        let mut tmp = Self::with_size(vec.len());
        for (i, val) in vec.iter().enumerate() {
            tmp.add(i, *val);
        }

        tmp
    }

    fn printRanges(&self) {
        for i in 0..self.capacity {
            println!("{i}:{:?}", self.range(i));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emh() {
        let mut tree = SegmentTree::with_size(5);
        println!("{tree:?}");
        tree.add(0, 3);
        tree.add(1, 5);
        tree.add(4, 4);
        tree.add(3, 2);
        println!("{tree:?}");
        tree.printRanges();
        println!("Sum:{}", tree.sum(1, 3));
        println!("Sum:{}", tree.sum(0, 4));
        println!("Sum:{}", tree.sum(0, 0));
        println!("Sum:{}", tree.sum(3, 4));
    }
}

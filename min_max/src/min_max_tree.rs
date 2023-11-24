#[derive(Debug)]
struct Node {
    value: Option<u32>,
    min: u32,
    id: usize,
}

impl Node {
    fn new(value: Option<u32>, id: usize) -> Self {
        Self {
            value,
            min: u32::MAX,
            id,
        }
    }
}

#[derive(Debug)]
pub struct MinMaxTree {
    nodes: Vec<Node>,
    size: usize,
    capacity: usize,
}
impl MinMaxTree {
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
        let pow = 1 << height;
        (id * pow + (pow - 1), id * pow + (pow - 1) * 2)
    }

    fn access(&self, id: usize) -> usize {
        self.capacity / 2 + id
    }

    pub fn initialize(&mut self, id: usize) -> u32 {
        if id > self.capacity {
            return 0;
        }
        match self.nodes[id].value {
            None => {
                self.nodes[id].value = Some(u32::max(
                    self.initialize(Self::left_child(id)),
                    self.initialize(Self::right_child(id)),
                ));
                self.nodes[id].value.unwrap()
            }
            Some(a) => a,
        }
    }

    pub fn max(&self, i: usize, j: usize) -> u32 {
        //println!("Max: {i} {j}");
        return self.rec_max(0, (i + self.capacity / 2 - 1, j + self.capacity / 2 - 1));
    }
    fn rec_max(&self, id: usize, query_range: (usize, usize)) -> u32 {
        let covered_range = self.range(id);
        let res;
        let mid = (covered_range.1 - covered_range.0) / 2 + covered_range.0;
        //println!("Calculating max{id}: {query_range:?} {covered_range:?} mid.{mid}");

        if query_range == covered_range {
            res = u32::min(self.nodes[id].value.unwrap_or(0), self.nodes[id].min);
        } else if query_range.0 > covered_range.1 || query_range.1 < covered_range.0 {
            res = u32::MIN;
        } else {
            res = u32::max(
                self.rec_max(
                    Self::left_child(id),
                    (query_range.0, usize::min(query_range.1, mid)),
                ),
                self.rec_max(
                    Self::right_child(id),
                    (usize::max(query_range.0, mid + 1), query_range.1),
                ),
            );
        }
        //println!("Returning max{id}: {res}");
        res
    }

    pub fn update(&mut self, i: usize, j: usize, t: u32) {
        self.rec_update(0, (i + self.capacity / 2 - 1, j + self.capacity / 2 - 1), t);
    }
    fn rec_update(&mut self, id: usize, query_range: (usize, usize), t: u32) {
        let covered_range = self.range(id);
        let mid = (covered_range.1 - covered_range.0) / 2 + covered_range.0;
        //println!("Updating min{id}: {query_range:?} {covered_range:?} mid.{mid}");

        if query_range == covered_range {
            self.nodes[id].min = u32::min(self.nodes[id].min, t);
        } else if query_range.0 > covered_range.1 || query_range.1 < covered_range.0 {
            return;
        } else {
            self.rec_update(
                Self::left_child(id),
                (query_range.0, usize::min(query_range.1, mid)),
                t,
            );
            self.rec_update(
                Self::right_child(id),
                (usize::max(query_range.0, mid + 1), query_range.1),
                t,
            );
        }
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

    pub fn from_vec(vec: Vec<u32>) -> Self {
        let mut tmp = Self::with_size(vec.len());
        for (i, val) in vec.iter().enumerate() {
            tmp.nodes[i + (tmp.capacity / 2)].value = Some(*val);
        }
        tmp.initialize(0);
        tmp
    }

    fn print_ranges(&self) {
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
        let mut tree = MinMaxTree::from_vec(vec![3, 5, 0, 2, 4]);
        println!("{tree:?}");
        tree.print_ranges();
        assert_eq!(tree.max(1, 3), 5);
        assert_eq!(tree.max(0, 4), 5);
        assert_eq!(tree.max(0, 0), 3);
        assert_eq!(tree.max(3, 4), 4);
        let mut tree = MinMaxTree::from_vec(vec![5, 1, 4, 3, 2]);
        tree.update(1, 2, 2);
        println!("{tree:?}");
        assert_eq!(tree.max(2, 4), 4);
        assert_eq!(tree.max(1, 2), 2);
    }
}

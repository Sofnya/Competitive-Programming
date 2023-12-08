#[derive(Debug)]
struct Node {
    // Each node stores the local maximum of its covered nodes.
    // We use bigger than necessary complete binary trees for simplicity
    // which means that some nodes are actually not needed, and are therefore left with None as their value.
    maximum: Option<u32>,
    minimum: u32,
}

impl Node {
    fn new(maximum: Option<u32>) -> Self {
        Self {
            maximum,
            minimum: u32::MAX,
        }
    }
}

#[derive(Debug)]
pub struct MinMaxTree {
    nodes: Vec<Node>,
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

    // Here we initialize the maximums of the tree
    fn initialize(&mut self, id: usize) -> u32 {
        if id > self.capacity {
            return 0;
        }
        match self.nodes[id].maximum {
            None => {
                self.nodes[id].maximum = Some(u32::max(
                    self.initialize(Self::left_child(id)),
                    self.initialize(Self::right_child(id)),
                ));
                self.nodes[id].maximum.unwrap()
            }
            Some(a) => a,
        }
    }

    pub fn max(&self, i: usize, j: usize) -> u32 {
        return self.rec_max(0, (i + self.capacity / 2 - 1, j + self.capacity / 2 - 1));
    }

    fn rec_max(&self, id: usize, query_range: (usize, usize)) -> u32 {
        let covered_range = self.range(id);
        let mid = (covered_range.1 - covered_range.0) / 2 + covered_range.0;

        if query_range == covered_range {
            // Total match
            u32::min(self.nodes[id].maximum.unwrap_or(0), self.nodes[id].minimum)
        } else if query_range.0 > covered_range.1 || query_range.1 < covered_range.0 {
            // No match
            // We answer with 0 as its a neutral value.
            u32::MIN
        } else {
            // Partial match
            // Here we combine the two subresults, correcting the result with the minimum if necessary
            u32::min(
                u32::max(
                    self.rec_max(
                        Self::left_child(id),
                        (query_range.0, usize::min(query_range.1, mid)),
                    ),
                    self.rec_max(
                        Self::right_child(id),
                        (usize::max(query_range.0, mid + 1), query_range.1),
                    ),
                ),
                self.nodes[id].minimum,
            )
        }
    }

    pub fn update(&mut self, i: usize, j: usize, t: u32) {
        self.rec_update(0, (i + self.capacity / 2 - 1, j + self.capacity / 2 - 1), t);
    }

    // We handle the update lazily, by inserting minimum in the highest matching nodes of the tree
    // This function also returns the new maximum of visited nodes, in order to update the values on the path back up to the root
    fn rec_update(&mut self, id: usize, query_range: (usize, usize), t: u32) -> u32 {
        let covered_range = self.range(id);
        let mid = (covered_range.1 - covered_range.0) / 2 + covered_range.0;
        if query_range == covered_range {
            // Total match
            self.nodes[id].minimum = u32::min(t, self.nodes[id].minimum);
            u32::min(self.nodes[id].minimum, self.nodes[id].maximum.unwrap_or(0))
        } else if query_range.0 > covered_range.1 || query_range.1 < covered_range.0 {
            // No match
            u32::min(self.nodes[id].minimum, self.nodes[id].maximum.unwrap_or(0))
        } else {
            // Partial match

            let tmp = u32::min(
                self.nodes[id].minimum,
                u32::max(
                    self.rec_update(
                        Self::left_child(id),
                        (query_range.0, usize::min(query_range.1, mid)),
                        t,
                    ),
                    self.rec_update(
                        Self::right_child(id),
                        (usize::max(query_range.0, mid + 1), query_range.1),
                        t,
                    ),
                ),
            );
            // We need to update our maximum, as it may have changed
            self.nodes[id].maximum = Some(tmp);
            tmp
        }
    }

    pub fn with_size(size: usize) -> Self {
        // We only work with complete binary trees for simplicity
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

    pub fn from_vec(vec: Vec<u32>) -> Self {
        let mut tmp = Self::with_size(vec.len());
        for (i, val) in vec.iter().enumerate() {
            tmp.nodes[i + (tmp.capacity / 2)].maximum = Some(*val);
        }
        tmp.initialize(0);
        tmp
    }

    fn print_ranges(&self) {
        for i in 0..self.capacity {
            println!("{i}:{:?}", self.range(i));
        }
    }

    pub fn pretty_print(&self) {
        let mut last_mid = 0;
        for id in 0..self.capacity {
            let range = self.range(id);
            let cur_mid = range.1 - range.0;
            if cur_mid < last_mid {
                println!("\n");
            };
            print!("{}", "-".repeat(cur_mid));
            print!("|{}", self.nodes[id].maximum.unwrap_or(0),);
            if self.nodes[id].minimum != u32::MAX {
                print!(".{}", self.nodes[id].minimum);
            } else {
                print!(".0");
            }

            print!("{}", "-".repeat(cur_mid));
            last_mid = cur_mid;
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tree = MinMaxTree::from_vec(vec![3, 5, 0, 2, 4]);
        tree.print_ranges();
        assert_eq!(tree.max(1, 3), 5);
        assert_eq!(tree.max(0, 4), 5);
        assert_eq!(tree.max(0, 0), 3);
        assert_eq!(tree.max(3, 4), 4);
        let mut tree = MinMaxTree::from_vec(vec![5, 1, 4, 3, 2]);
        tree.update(1, 2, 2);
        assert_eq!(tree.max(2, 4), 4);
        assert_eq!(tree.max(1, 2), 2);
    }
}

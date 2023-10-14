#[derive(Debug)]
struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
}

/// This a representation of a tree.
/// Every node has an implicity id, which is its position on the vector `nodes`.
/// Every node has a key and at most two children. The ids of the children are
/// stored in `id_left` and `id_right`. These ids are `None` iff the child does not exit.
impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left child of the node `parent_id`
    /// iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    // Returns whether the tree is a binary search tree or not
    pub fn is_bst(&self) -> bool {
        self.rec_is_bst(Some(0)).0
    }

    // A private recursive function that returns whether the subtree rooted at `node_id` is a bst or not,
    // the maximum key in the subtree, and the minimum key in the subtree
    fn rec_is_bst(&self, node_id: Option<usize>) -> (bool, u32, u32) {
        match node_id {
            None => (true, u32::MIN, u32::MAX),
            Some(node_id) => {
                assert!(node_id < self.nodes.len(), "Node id is out of range");
                let node = &self.nodes[node_id];

                let (res_l, max_l, min_l) = self.rec_is_bst(node.id_left);
                let (res_r, max_r, min_r) = self.rec_is_bst(node.id_right);

                let res = res_l && res_r && (max_l <= node.key) && (min_r > node.key);
                let max = std::cmp::max(node.key, std::cmp::max(max_l, max_r));
                let min = std::cmp::min(node.key, std::cmp::min(min_l, min_r));

                (res, max, min)
            }
        }
    }

    // Returns whether the tree is balanced or not
    pub fn is_balanced(&self) -> bool {
        self.rec_is_balanced(Some(0)).0
    }

    // A private recursive function that returns whether the subtree rooted at `node_id` is balanced or not,
    // and the maximum height of the subtree
    fn rec_is_balanced(&self, node_id: Option<usize>) -> (bool, u32) {
        match node_id {
            None => (true, 0),
            Some(node_id) => {
                assert!(node_id < self.nodes.len(), "Node id is out of range");
                let node = &self.nodes[node_id];

                let (res_l, height_l) = self.rec_is_balanced(node.id_left);
                let (res_r, height_r) = self.rec_is_balanced(node.id_right);

                let res = res_l
                    && res_r
                    // We use comparisons to avoid underflow as heights are unsigned
                    && (std::cmp::max(height_l, height_r) - std::cmp::min(height_l, height_r)) <= 1;
                let height = std::cmp::max(height_l, height_r) + 1;

                (res, height)
            }
        }
    }

    // Returns whether the tree is a max heap or not
    pub fn is_max_heap(&self) -> bool {
        // First we check if the tree is complete
        if !self.rec_is_complete(Some(0)).0 {
            false
        } else {
            // If it is complete, we check if all nodes satisfy the max heap property
            self.rec_is_max_heap(Some(0)).0
        }
    }

    // A private recursive function that returns whether the subtree rooted at `node_id` is complete or not,
    // whether it is perfect or not, and the height of the subtree
    fn rec_is_complete(&self, node_id: Option<usize>) -> (bool, bool, u32) {
        match node_id {
            None => (true, true, 0),
            Some(node_id) => {
                assert!(node_id < self.nodes.len(), "Node id is out of range");
                let node = &self.nodes[node_id];

                let (complete_l, perfect_l, height_l) = self.rec_is_complete(node.id_left);
                let (complete_r, perfect_r, height_r) = self.rec_is_complete(node.id_right);

                // A tree is perfect if both its subtrees are perfect and of the same height
                let perfect = perfect_l && perfect_r && height_l == height_r;

                // A tree is complete if it is perfect
                let complete = perfect
                    // Otherwise, it can still be perfect if its left subtree is perfect and the right subtree is complete
                    || (perfect_l && complete_r) && height_r == height_l
                    // Or if its left subtree is complete and the right subtree is perfect and of height one less
                    || (complete_l && perfect_r) && height_r + 1 == height_l;

                let height = std::cmp::max(height_l, height_r) + 1;
                (complete, perfect, height)
            }
        }
    }

    // A private recursive function that returns whether the subtree rooted at `node_id` is a max heap or not,
    // and the key of the subtree root
    fn rec_is_max_heap(&self, node_id: Option<usize>) -> (bool, u32) {
        match node_id {
            None => (true, 0),
            Some(node_id) => {
                assert!(node_id < self.nodes.len(), "Node id is out of range");
                let node = &self.nodes[node_id];

                let (res_l, key_l) = self.rec_is_max_heap(node.id_left);
                let (res_r, key_r) = self.rec_is_max_heap(node.id_right);

                let res = res_l && res_r && (key_l <= node.key) && (key_r <= node.key);
                (res, node.key)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
    }

    #[test]
    fn test_is_bst() {
        let mut tree = Tree::with_root(10);
        assert!(tree.is_bst());

        tree.add_node(0, 5, true);
        assert!(tree.is_bst());
        tree.add_node(0, 22, false);
        assert!(tree.is_bst());
        tree.add_node(1, 7, false);
        assert!(tree.is_bst());
        tree.add_node(3, 20, false);
        assert!(!tree.is_bst());
    }

    #[test]
    fn test_is_balanced() {
        let mut tree = Tree::with_root(10);
        assert!(tree.is_balanced());

        tree.add_node(0, 5, true);
        assert!(tree.is_balanced());
        tree.add_node(0, 22, false);
        assert!(tree.is_balanced());
        tree.add_node(1, 7, false);
        assert!(tree.is_balanced());
        tree.add_node(3, 20, false);
        assert!(!tree.is_balanced());
    }

    #[test]
    fn test_is_max_heap() {
        // First some completeness tests
        let mut tree = Tree::with_root(100);
        assert!(tree.is_max_heap());

        tree.add_node(0, 50, true); // id 1
        assert!(tree.is_max_heap());
        tree.add_node(0, 52, false); // id 2
        assert!(tree.is_max_heap());
        tree.add_node(1, 27, false); // id 3
        assert!(!tree.is_max_heap());
        tree.add_node(1, 20, true); // id 4
        assert!(tree.is_max_heap());

        tree.add_node(2, 10, false); // id 5
        assert!(!tree.is_max_heap());
        tree.add_node(2, 11, true); // id 6
        assert!(tree.is_max_heap());
        tree.add_node(6, 9, false); // id 7
        assert!(!tree.is_max_heap());

        // Now some max heap tests
        let mut tree = Tree::with_root(100);
        assert!(tree.is_max_heap());

        tree.add_node(0, 50, true); // id 1
        assert!(tree.is_max_heap());
        tree.add_node(0, 52, false); // id 2
        assert!(tree.is_max_heap());
        tree.add_node(1, 27, true); // id 3
        println!("{:?}", tree);
        assert!(tree.is_max_heap());
        tree.add_node(1, 50, false); // id 4
        assert!(tree.is_max_heap());
        tree.add_node(2, 53, true); // id 5
        assert!(!tree.is_max_heap());
    }
}

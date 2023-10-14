struct Heap {
    heap: Vec<i32>,
    size: usize,
}

impl Heap {
    fn new() -> Heap {
        Heap {
            heap: Vec::new(),
            size: 0,
        }
    }

    fn push(&mut self, el: i32) {
        self.heap.push(el);
        self.size += 1;
        self.sift_up(self.size - 1);
    }

    fn left(&self, i: usize) -> usize {
        return 2 * i;
    }

    fn right(&self, i: usize) -> usize {
        return 2 * i + 1;
    }

    fn parent(&self, i: usize) -> usize {
        return i / 2;
    }
}

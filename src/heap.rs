#[derive(Debug)]
pub struct MinHeap {
    length: usize,
    harr: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        Self {
            length: 0,
            harr: vec![],
        }
    }

    fn get_left_child_idx(&self, i: usize) -> usize {
        2 * i + 1
    }

    fn get_right_child_idx(&self, i: usize) -> usize {
        2 * i + 2
    }

    fn get_parent_idx(&self, i: usize) -> usize {
        (i - 1) / 2
    }

    fn swap_up(&mut self, idx: usize) {
        let item = self.harr[idx];

        let parent_idx = self.get_parent_idx(idx);

        if item > self.harr[parent_idx] {
            return;
        }

        self.harr[idx] = self.harr[parent_idx];
        self.harr[parent_idx] = item;
        if parent_idx != 0 {
            self.swap_up(parent_idx)
        }
    }

    pub fn insert(&mut self, item: i32) {
        self.harr.push(item);

        if self.harr.len() > 1 {
            self.swap_up(self.harr.len() - 1);
        }
    }
    pub fn delete(&mut self) {
        self.harr[0] = self.harr[self.harr.len() - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::MinHeap;

    #[test]
    fn test_heap() {
        let mut heap = MinHeap::new();

        heap.insert(20);

        assert_eq!(heap.harr, vec![20]);
        heap.insert(21);
        heap.insert(15);
        heap.insert(18);
        heap.insert(1);
        assert_eq!(heap.harr, vec![20]);
    }
}

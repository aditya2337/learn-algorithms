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

    fn swap_down(&mut self, idx: usize) {
        let item = self.harr[idx];
        let mut smallest = idx;

        let left = self.get_left_child_idx(idx);
        let right = self.get_left_child_idx(idx);

        if left < self.harr.len() && self.harr[left] < item {
            smallest = left;
        } else if right < self.harr.len() && self.harr[right] < item {
            smallest = right;
        }

        if smallest != idx {
            self.harr[idx] = self.harr[smallest];
            self.harr[smallest] = item;
            self.swap_down(smallest);
        }
    }

    pub fn insert(&mut self, item: i32) {
        self.harr.push(item);

        if self.harr.len() > 1 {
            self.swap_up(self.harr.len() - 1);
        }
    }

    pub fn delete_min(&mut self) {
        let last_elem = self.harr.pop();
        if let Some(last_elem) = last_elem {
            self.harr[0] = last_elem;
            self.swap_down(0);
        }
    }

    pub fn delete_element(&mut self, item: i32) {
        let idx = self.harr.iter().position(|x| x == &item).unwrap();
        // our assumption is that the heap won't store 0,
        // This is not ideal, but hey you get the idea 😉
        const MIN_VAL: i32 = 0;
        self.harr[idx] = MIN_VAL;
        self.swap_up(idx);
        self.delete_min();
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
        assert_eq!(heap.harr, vec![1, 15, 20, 21, 18]);
        heap.delete_min();
        assert_eq!(heap.harr, vec![15, 18, 20, 21]);
        heap.delete_element(21);
        assert_eq!(heap.harr, vec![15, 18, 20]);
    }
}

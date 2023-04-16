#[derive(Debug)]
struct Heap {
    heap: Vec<i32>,
    size: usize,
}

impl Heap {
    pub fn new(arr: Vec<i32>) -> Self {
        let size = arr.len();
        let mut heap = Self { heap: arr, size };

        let mut i = heap.size;

        while i > 0 {
            i -= 1;
            heap.heapify(i);
        }

        heap
    }
    fn heapify(&mut self, idx: usize) {
        let mut largest = idx;
        let l = self.left_idx(idx);
        let r = self.right_idx(idx);

        if l < self.size && self.heap[l] > self.heap[idx] {
            largest = l;
        }

        if r < self.size && self.heap[r] > self.heap[largest] {
            largest = r;
        }

        if largest != idx {
            let temp = self.heap[largest];
            self.heap[largest] = self.heap[idx];
            self.heap[idx] = temp;
            self.heapify(largest);
        }
    }

    pub fn sort(&mut self) {
        let mut i = self.size;

        let initial_size = self.size;

        while i > 0 {
            i -= 1;
            let largest = self.delete();
            self.heap[self.size] = largest;
        }

        self.size = initial_size;
    }

    fn delete(&mut self) -> i32 {
        let largest = self.heap[0];
        let last_idx = self.size - 1;
        self.heap[0] = self.heap[last_idx];
        self.heap[last_idx] = 0;
        self.heapify(0);
        self.size -= 1;
        largest
    }

    fn left_idx(&self, i: usize) -> usize {
        2 * i + 1
    }

    fn right_idx(&self, i: usize) -> usize {
        2 * i + 2
    }
}

#[cfg(test)]
mod tests {
    use super::Heap;

    #[test]
    fn test_heap() {
        let mut heap = Heap::new(vec![1, 14, 3, 5, 6, 60]);

        assert_eq!(heap.heap, vec![60, 14, 3, 5, 6, 1]);

        heap.sort();

        assert_eq!(heap.heap, vec![1, 3, 5, 6, 14, 60]);
    }
}

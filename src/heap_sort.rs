#[derive(Debug)]
struct Heap {
    heap: Vec<i32>,
    size: usize,
}

impl Heap {
    pub fn new(arr: Vec<i32>) -> Self {
        let size = arr.len();
        let mut heap = Self { heap: arr, size };

        for i in 0..heap.heap.len() {
            heap.heapify(i);
        }

        heap
    }
    pub fn heapify(&mut self, idx: usize) {
        let mut largest = idx;
        let l = self.left_idx(idx);
        let r = self.right_idx(idx);

        if l < self.size && self.heap[l] > self.heap[idx] {
            largest = l;
        }

        if r < self.size && self.heap[r] > self.heap[idx] {
            largest = r;
        }

        if largest != idx {
            let temp = self.heap[largest];
            self.heap[largest] = self.heap[idx];
            self.heap[idx] = temp;
            self.heapify(largest);
        }
    }

    fn left_idx(&self, i: usize) -> usize {
        2 * i
    }

    fn right_idx(&self, i: usize) -> usize {
        2 * i + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Heap;

    #[test]
    fn test_heap() {
        let heap = Heap::new(vec![1, 14, 3, 5, 6, 60]);

        println!("{:?}", heap.heap);
    }
}

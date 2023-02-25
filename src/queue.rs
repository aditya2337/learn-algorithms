type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

#[derive(Debug)]
pub struct Queue<T> {
    length: i32,
    head: Link<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }

    fn enqueue(&mut self, item: T) {
        // create a new node
        let mut new_node = Box::new(Node::new(item));
        // get the node in head
        let current_node = self.head.take();

        match current_node {
            Some(current_node) => {
                // point the new node's next to the current node
                new_node.next = Some(current_node);
                // point the head to the new node
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }

        self.length += 1;
    }

    fn dequeue(&mut self) {
        let current_node = self.head.take();
        match current_node {
           Some(mut current_node) => {
               // take the next node from head
                let next_node = current_node.next.take();
                // point the head to the next node
                self.head = next_node;
           }
           None => {
               panic!("Nothing in the queue");
           }
        }

        self.length -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_queue() {
        let mut q = Queue::<i32>::new();

        q.enqueue(1);
        assert_eq!(q.length, 1);

        q.enqueue(3);
        assert_eq!(q.length, 2);

        q.dequeue();
        assert_eq!(q.length, 1);

        q.dequeue();
        assert_eq!(q.length, 0);

        println!("The queue: {:?}", q);
    }
}

use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}
impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, val: T) {
        let new_node = Node::new(val);

        match self.tail.take() {
            Some(tail_node) => {
                tail_node.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(tail_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }

        self.tail = Some(new_node);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.tail.take().map(|tail_node| {
            match tail_node.borrow_mut().prev.take() {
                Some(prev_node) => {
                    prev_node.borrow_mut().next = None;
                    self.tail = Some(prev_node);
                }
                None => {
                    self.head.take();
                }
            }

            self.length -= 1;
            Rc::try_unwrap(tail_node).ok().unwrap().into_inner().val
        })
    }
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}
impl<T> Node<T> {
    pub fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            prev: None,
            next: None,
        }))
    }
}




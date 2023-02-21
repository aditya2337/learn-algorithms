use std::{fmt::Debug, ptr};

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

impl<T> LinkedList<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn get_length(&self) -> i32 {
        match &self.head {
            Some(head) => {
                let mut counter = 1;

                let mut this_ref = &head.next;

                while let Some(i) = this_ref {
                    counter = counter + 1;
                    this_ref = &i.next;
                }
                counter
            }
            None => 0,
        }
    }

    fn check_idx_validity(&self, &idx: &i32) {
        if idx > self.get_length() - 1 {
            panic!("out of bound");
        }
    }

    pub fn insert_at(&mut self, value: T, idx: i32) {
        self.check_idx_validity(&idx);

        let mut this_ref = &self.head;

        for i in 0..idx {
            this_ref = match this_ref {
                Some(a) => {
                    &a.next
                }
                None => break
            };
        }


         match this_ref {
            Some(node_at_idx) => {
                let next_node = &node_at_idx.next;
                match next_node {
                    Some(existing_next_node) => {
                        let mut new_node = Box::new(Node { value, next: next_node });
                        existing_next_node.next = Some(new_node);
                    }
                    None => self.append(value)
                }
            }
            None => (),
         }
    }

    pub fn append(&mut self, value: T) {
        let mut new_tail = Box::new(Node::new(value));
        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    fn prepend(&self) {}
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

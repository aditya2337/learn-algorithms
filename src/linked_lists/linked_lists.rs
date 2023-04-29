use std::{fmt::Debug, mem, ptr, ops::Index};

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

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

    pub fn insert_at(&mut self, value: T, idx: i32) -> bool {
        self.check_idx_validity(&idx);

        let mut current_node = self.head.as_mut();
        let mut current_index = 0;

        while let Some(node) = current_node {
            if current_index == idx {
                let new_node = Box::new(Node {
                    value,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                return true;
            }
            current_node = node.next.as_mut();
            current_index += 1;
        }

        false
    }

    pub fn delete_at(&mut self, idx: i32) -> bool {
        let idx_to_check = idx - 1;
        self.check_idx_validity(&idx_to_check);

        let mut current_node = self.head.as_mut();
        let mut current_index = 0;

        while let Some(node) = current_node {

            if idx_to_check == current_index - 1 {
                let node_to_delete = node.next.take();
                match node_to_delete {
                    Some(node_exists) => {
                        println!("do we get here tho");
                        node.next = node_exists.next;
                    }
                    // TODO: Delete the head 😱
                    None => ()
                }
                return true;
            }
            current_node = node.next.as_mut();
            current_index += 1;
        }
        false
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

pub fn test_ll() {
    let mut ll = LinkedList::<i32>::new();
    println!("Linked list length is: {:?}", ll.get_length());
    ll.append(2);
    ll.append(2);
    println!("After Append: Linked list length is: {:?}", ll);
    println!("After Append: Linked list length is: {:?}", ll.get_length());
    ll.insert_at(3, 0);
    println!("After Insert: Linked list length is: {:?}", ll);
    println!("After Insert: Linked list length is: {:?}", ll.get_length());
    ll.delete_at(1);
    println!("After Delete: Linked list length is: {:?}", ll);
    println!("After Delete: Linked list length is: {:?}", ll.get_length());
}

use std::fmt::Debug;

pub mod types;

#[derive(Debug)]
pub struct BinaryTree<T> {
    height: usize,
    root: Link<T>,
}

impl<T> BinaryTree<T>
where
    T: Debug,
{
    pub fn new(height: usize, root: Link<T>) -> Self {
        Self { height, root }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left_node: Link<T>,
    right_node: Link<T>,
}

impl<T> Node<T> where T: Debug {}

pub type Link<T> = Option<Box<Node<T>>>;

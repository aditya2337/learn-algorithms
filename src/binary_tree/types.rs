use std::fmt::Debug;

#[derive(Debug)]
pub struct BinaryTree<T> {
    pub height: usize,
    pub root: Link<T>,
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
    pub value: T,
    pub left_node: Link<T>,
    pub right_node: Link<T>,
}

impl<T> Node<T> where T: Debug {}

pub type Link<T> = Option<Box<Node<T>>>;

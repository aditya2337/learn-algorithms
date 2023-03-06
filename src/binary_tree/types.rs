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
    pub fn new() -> Self {
        Self { height: 0, root: None }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left_node: Link<T>,
    pub right_node: Link<T>,
}

impl<T> Node<T>
where
    T: Debug,
{
    pub fn new(value: T) -> Self {
        Self {
            value,
            left_node: None,
            right_node: None,
        }
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            left_node: self.left_node.clone(),
            right_node: self.right_node.clone(),
        }
    }
}

pub type Link<T> = Option<Box<Node<T>>>;

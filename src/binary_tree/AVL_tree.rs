use std::cmp::max;

type Link = Option<Box<AVLNode>>;

#[derive(Debug, Clone)]
struct AVLNode {
    val: i32,
    left_child: Link,
    right_child: Link,
    height: usize,
}

impl AVLNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left_child: None,
            right_child: None,
            height: 1,
        }
    }

    fn height(node: &Link) -> usize {
        match node {
            Some(node) => node.height,
            None => 0,
        }
    }

    fn get_left_height(&self) -> usize {
        Self::height(&self.left_child)
    }

    fn get_right_height(&self) -> usize {
        Self::height(&self.right_child)
    }

    fn balance_factor(&self) -> i8 {
        self.get_right_height() as i8 - self.get_left_height() as i8
    }

    fn update_height(&mut self) {
        let left_child = self.get_left_height();
        let right_child = self.get_right_height();

        let height = self.height;

        println!("{height}");

        self.height = 1 + max(left_child, right_child);
    }

    fn rotate_left(&mut self) -> Link {
        let mut node = self.right_child.take().unwrap();

        self.right_child = node.as_mut().left_child.take();
        node.left_child = Some(Box::new(self.clone()));
        self.update_height();
        node.update_height();
        Some(node)
    }

    fn rotate_right(&mut self) -> Link {
        let mut node = self.left_child.take().unwrap();

        self.left_child = node.as_mut().right_child.take();
        node.right_child = Some(Box::new(self.clone()));
        self.update_height();
        node.update_height();
        Some(node)
    }

    fn balance(&mut self) {
        let bf = self.balance_factor();

        if bf > 1 {
            // rotate left
            if self.right_child.as_ref().unwrap().balance_factor() < 0 {
                self.right_child = self.right_child.as_mut().unwrap().rotate_right();
            }
            self.rotate_left();
        } else if bf < -1 {
            // rotate right
            if self.left_child.as_ref().unwrap().balance_factor() > 0 {
                // Perform right-left rotations
                self.left_child = self.left_child.as_mut().unwrap().rotate_left();
            }
            self.rotate_right();
        }
    }

    pub fn insert(&mut self, val: i32) {
        let new_node = Self::new(val);
        walk_and_insert(self, new_node);
        self.update_height();
        self.balance();
    }
}

#[derive(Debug)]
struct AVLTree {
    root: Link,
}

impl AVLTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, val: i32) {
        let new_node = AVLNode::new(val);

        if self.root.is_none() {
            self.root = Some(Box::new(new_node));
        } else {
            self.root.as_mut().unwrap().insert(val);
        }
    }
}

fn walk_and_insert(node: &mut AVLNode, new_node: AVLNode) {
    if node.val > new_node.val {
        if node.left_child.is_none() {
            node.left_child = Some(Box::new(new_node));
            return;
        } else {
            return walk_and_insert(node.left_child.as_mut().unwrap(), new_node);
        }
    } else {
        if node.right_child.is_none() {
            node.right_child = Some(Box::new(new_node));
            return;
        } else {
            return walk_and_insert(node.right_child.as_mut().unwrap(), new_node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AVLTree;

    #[test]
    fn test_avl() {
        let mut tree = AVLTree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(3);
        tree.insert(5);

        println!("tree -------> {:?}", tree);
    }
}

type Link = Option<Box<AVLNode>>;

#[derive(Debug)]
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

    pub fn balance_factor(&self) -> i8 {
        (self.right_child.as_ref().unwrap().get_left_height()
            - self.left_child.as_ref().unwrap().get_right_height())
        .try_into()
        .unwrap_or(0)
    }

    pub fn insert(&mut self, val: i32) {
        let new_node = Self::new(val);
        walk_and_insert(self, new_node);
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

        println!("tree -------> {:?}", tree);
    }
}

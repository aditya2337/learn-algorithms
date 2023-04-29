type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    val: i32,
    left_child: Link,
    right_child: Link,
    balance_factor: i8,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left_child: None,
            right_child: None,
            balance_factor: 0,
        }
    }

    pub fn is_unbalanced(&self) -> bool {
        self.balance_factor < -1 || self.balance_factor > 1
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
        let new_node = Node::new(val);

        if self.root.is_none() {
            self.root = Some(Box::new(new_node));
        } else {
            walk_and_insert(self.root.as_mut().unwrap(), new_node);
        }
    }
}

fn walk_and_insert(node: &mut Node, new_node: Node) {
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


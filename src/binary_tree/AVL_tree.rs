type Link = Option<Box<Node>>;

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

struct AVLTree {
    root: Link,
}

impl AVLTree {
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

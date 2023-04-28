type Link<T> = Option<Box<Node<T>>>;

enum BalanceFactor {}

struct Node<T> {
    val: T,
    left_child: Link<T>,
    right_child: Link<T>,
    balance_factor: i8,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
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

struct AVL_Tree<T> {
    root: Link<T>,
}

impl<T> AVL_Tree<T> {}

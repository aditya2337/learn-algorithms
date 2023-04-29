#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut ptr_1 = head.as_ref();
    let mut ptr_2 = head.as_ref();

    while ptr_2.is_some() && ptr_2.as_ref().unwrap().next.is_some() {
        ptr_1 = ptr_1.unwrap().next.as_ref();
        ptr_2 = ptr_2.unwrap().next.as_ref().unwrap().next.as_ref();
    }

    ptr_1.cloned()
}

fn create_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &num in nums.iter().rev() {
        let node = ListNode {
            val: num,
            next: head,
        };
        head = Some(Box::new(node));
    }
    head
}

#[cfg(test)]
mod tests {
    use crate::reverse_linked_list::{create_linked_list, reverse_list};

    #[test]
    fn test_merge() {
        let list_a = create_linked_list(vec![1, 2, 4]);

        let meged_linked_list = reverse_list(list_a);

        print!("{:?}", meged_linked_list);
    }
}

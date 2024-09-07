#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    fn new_node(val: i32, node: ListNode) -> Self {
        ListNode { next: Some(Box::new(node)), val }
    }
}

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let Some(mut l1) = list1 else { return list2 };
    let Some(mut l2) = list2 else { return Some(l1) };
    
    return if l1.val < l2.val {
        l1.next = merge_two_lists(l1.next, Some(l2));
        Some(l1)
    } else {
        l2.next = merge_two_lists(Some(l1), l2.next);
        Some(l2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let list1 = Some(Box::new(ListNode::new_node(1, ListNode::new_node(2, ListNode::new(4)))));
        let list2 = Some(Box::new(ListNode::new_node(1, ListNode::new_node(3, ListNode::new(4)))));
        let merged = Some(Box::new(ListNode::new_node(1, ListNode::new_node(1, ListNode::new_node(2, ListNode::new_node(3, ListNode::new_node(4, ListNode::new(4))))))));
        assert_eq!(merge_two_lists(list1, list2), merged);
    }

    #[test]
    fn test2() {
        let list1 = None;
        let list2 = None;
        let merged = None;
        assert_eq!(merge_two_lists(list1, list2), merged);
    }

    #[test]
    fn test3() {
        let list1 = Some(Box::new(ListNode::new(0)));
        let list2 = None;
        let merged = Some(Box::new(ListNode::new(0)));
        assert_eq!(merge_two_lists(list1, list2), merged);
    }

    #[test]
    fn test4() {
        let list1 = Some(Box::new(ListNode::new_node(1, ListNode::new_node(2, ListNode::new(4)))));
        let list2 = Some(Box::new(ListNode::new(5)));
        let merged = Some(Box::new(ListNode::new_node(1, ListNode::new_node(2, ListNode::new_node(4, ListNode::new(5))))));
        assert_eq!(merge_two_lists(list1, list2), merged);
    }
}
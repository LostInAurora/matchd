// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub(crate) struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut current = dummy_head.as_mut();
        let (mut p1, mut p2, mut carry) = (l1, l2, 0);
        while p1.is_some() || p2.is_some() {
            let sum = p1.as_ref().map_or(0, |node| node.val)
                + p2.as_ref().map_or(0, |node| node.val)
                + carry;
            carry = sum / 10;
            if let Some(cur) = current {
                cur.next = Some(Box::new(ListNode::new(sum % 10)));
                current = cur.next.as_mut();
            }
            p1 = p1.and_then(|node| node.next);
            p2 = p2.and_then(|node| node.next);
        }
        if carry != 0 {
            let node = ListNode::new(carry);
            if let Some(cur) = current {
                cur.next = Some(Box::new(node));
            }
        }
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 辅助函数，用于创建链表
    fn create_list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;
        for &v in vals {
            *current = Some(Box::new(ListNode::new(v)));
            if let Some(current_node) = current {
                current = &mut current_node.next;
            }
        }
        head
    }

    #[test]
    fn test_add_two_numbers_1() {
        let l1 = create_list(&[1, 4, 3]);
        let l2 = create_list(&[5, 6, 4]);
        let expected = create_list(&[6, 0, 8]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_add_two_numbers_2() {
        let l1 = create_list(&[0]);
        let l2 = create_list(&[0]);
        let expected = create_list(&[0]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_add_two_numbers_3() {
        let l1 = create_list(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_list(&[9, 9, 9, 9]);
        let expected = create_list(&[8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }
}
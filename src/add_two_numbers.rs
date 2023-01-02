// https://leetcode.cn/problems/add-two-numbers/

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 节点数在 [0, 100] 的范围内，故不能转换成整数再加
        let mut result = None;
        let mut current_node = &mut result;
        let mut need_carry = false;
        let mut l1 = &l1;
        let mut l2 = &l2;

        loop {
            if l1.is_none() && l2.is_none() {
                break;
            }

            let mut v1 = 0;
            let mut v2 = 0;

            if let Some(node) = l1 {
                v1 = node.val;
                l1 = &node.next;
            }
            if let Some(node) = l2 {
                v2 = node.val;
                l2 = &node.next;
            }

            let mut digit = v1 + v2;
            if need_carry {
                digit += 1;
            }

            if digit < 10 {
                need_carry = false;
            } else {
                digit -= 10;
                need_carry = true;
            }

            *current_node = Some(Box::new(ListNode::new(digit)));
            if let Some(node) = current_node {
                current_node = &mut node.next;
            }
        }

        if need_carry {
            *current_node = Some(Box::new(ListNode::new(1)));
        }

        result
    }
}

pub struct Solution;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let mut l1 = Some(Box::new(ListNode::new(2)));
        let mut l1_ref = &mut l1;
        if let Some(node) = l1_ref {
            node.next = Some(Box::new(ListNode::new(4)));
            l1_ref = &mut node.next;
        }
        if let Some(node) = l1_ref {
            node.next = Some(Box::new(ListNode::new(3)));
        }

        let mut l2 = Some(Box::new(ListNode::new(5)));
        let mut l2_ref = &mut l2;
        if let Some(node) = l2_ref {
            node.next = Some(Box::new(ListNode::new(6)));
            l2_ref = &mut node.next;
        }
        if let Some(node) = l2_ref {
            node.next = Some(Box::new(ListNode::new(4)));
        }

        let mut result = Some(Box::new(ListNode::new(7)));
        let mut result_ref = &mut result;
        if let Some(node) = result_ref {
            node.next = Some(Box::new(ListNode::new(0)));
            result_ref = &mut node.next;
        }
        if let Some(node) = result_ref {
            node.next = Some(Box::new(ListNode::new(8)));
        }
        assert_eq!(result, Solution::add_two_numbers(l1, l2));

        l1 = Some(Box::new(ListNode::new(0)));
        l2 = Some(Box::new(ListNode::new(0)));
        result = Some(Box::new(ListNode::new(0)));
        assert_eq!(result, Solution::add_two_numbers(l1, l2));

        let mut l1 = Some(Box::new(ListNode::new(9)));
        let mut l1_ref = &mut l1;
        if let Some(node) = l1_ref {
            node.next = Some(Box::new(ListNode::new(9)));
            l1_ref = &mut node.next;
        }
        if let Some(node) = l1_ref {
            node.next = Some(Box::new(ListNode::new(9)));
            l1_ref = &mut node.next;
        }
        if let Some(node) = l1_ref {
            node.next = Some(Box::new(ListNode::new(9)));
            l1_ref = &mut node.next;
        }
        if let Some(node) = l1_ref {
            node.next = Some(Box::new(ListNode::new(9)));
            l1_ref = &mut node.next;
        }
        if let Some(node) = l1_ref {
            node.next = Some(Box::new(ListNode::new(9)));
            l1_ref = &mut node.next;
        }
        if let Some(node) = l1_ref {
            node.next = Some(Box::new(ListNode::new(9)));
        }

        let mut l2 = Some(Box::new(ListNode::new(9)));
        let mut l2_ref = &mut l2;
        if let Some(node) = l2_ref {
            node.next = Some(Box::new(ListNode::new(9)));
            l2_ref = &mut node.next;
        }
        if let Some(node) = l2_ref {
            node.next = Some(Box::new(ListNode::new(9)));
            l2_ref = &mut node.next;
        }
        if let Some(node) = l2_ref {
            node.next = Some(Box::new(ListNode::new(9)));
        }

        let mut result = Some(Box::new(ListNode::new(8)));
        let mut result_ref = &mut result;
        if let Some(node) = result_ref {
            node.next = Some(Box::new(ListNode::new(9)));
            result_ref = &mut node.next;
        }
        if let Some(node) = result_ref {
            node.next = Some(Box::new(ListNode::new(9)));
            result_ref = &mut node.next;
        }
        if let Some(node) = result_ref {
            node.next = Some(Box::new(ListNode::new(9)));
            result_ref = &mut node.next;
        }
        if let Some(node) = result_ref {
            node.next = Some(Box::new(ListNode::new(0)));
            result_ref = &mut node.next;
        }
        if let Some(node) = result_ref {
            node.next = Some(Box::new(ListNode::new(0)));
            result_ref = &mut node.next;
        }
        if let Some(node) = result_ref {
            node.next = Some(Box::new(ListNode::new(0)));
            result_ref = &mut node.next;
        }
        if let Some(node) = result_ref {
            node.next = Some(Box::new(ListNode::new(1)));
        }
        assert_eq!(result, Solution::add_two_numbers(l1, l2));
    }
}

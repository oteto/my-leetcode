/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

use std::borrow::Borrow;

struct Solution {}

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
// @lc code=start
// Definition for singly-linked list.
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tmp = head.as_mut().unwrap();
        let mut l1 = l1;
        let mut l2 = l2;

        loop {
            match (l1.as_ref(), l2.as_ref()) {
                (Some(n1), Some(n2)) => {
                    if n1.val <= n2.val {
                        tmp.next = Some(Box::new(ListNode::new(n1.val)));
                        l1 = l1.unwrap().next;
                    } else {
                        tmp.next = Some(Box::new(ListNode::new(n2.val)));
                        l2 = l2.unwrap().next;
                    }
                }
                _ => break,
            }
            tmp = tmp.next.as_mut().unwrap();
        }
        if l1.is_some() {
            tmp.next = l1;
        } else if l2.is_some() {
            tmp.next = l2;
        }
        head.unwrap().next
    }
}
// @lc code=end

#[test]
fn test_21() {
    assert_eq!(
        Solution::merge_two_lists(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None }))
                        }))
                    }))
                }))
            }))
        }))
    )
}

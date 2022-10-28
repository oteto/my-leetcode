/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

use std::ops::DerefMut;

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut up = 0;
        let mut v1 = 0;
        let mut v2 = 0;

        while l1.is_some() || l2.is_some() || up != 0 {
            v1 = 0;
            v2 = 0;

            if let Some(n) = l1 {
                v1 = n.val;
                l1 = n.next;
            }
            if let Some(n) = l2 {
                v2 = n.val;
                l2 = n.next;
            }

            let sum = v1 + v2 + up;

            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
            up = sum / 10;
        }

        dummy_head.next
    }
}
// @lc code=end

#[test]
fn test2() {
    let l = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));
    let r = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    assert_eq!(
        Solution::add_two_numbers(l, r),
        Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            }))
        }))
    );
}

/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let s = x.to_string();
        let mut chars = s.chars().into_iter();
        loop {
            match (chars.next(), chars.next_back()) {
                (Some(l), Some(r)) => {
                    if l != r {
                        return false;
                    }
                }
                _ => return true,
            }
        }
    }
}
// @lc code=end

#[test]
fn test9() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
}

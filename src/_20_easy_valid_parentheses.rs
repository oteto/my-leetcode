/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

struct Solution {}

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = VecDeque::new();
        for c in s.chars() {
            match c {
                '(' => v.push_back(')'),
                '{' => v.push_back('}'),
                '[' => v.push_back(']'),
                rc => {
                    if let Some(c) = v.pop_back() {
                        if c == rc {
                            continue;
                        }
                    }
                    return false;
                }
            }
        }
        v.is_empty()
    }
}
// @lc code=end

#[test]
fn test_20() {
    assert_eq!(Solution::is_valid(String::from("()")), true);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("(]")), false);
    assert_eq!(Solution::is_valid(String::from("(])")), false);
}

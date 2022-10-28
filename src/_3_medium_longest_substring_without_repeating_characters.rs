/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

struct Solution {}

// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut curr = Vec::new();
        let mut max_len = 0;

        for c in s.chars() {
            if let Some(i) = curr.iter().position(|&x| x == c) {
                curr.drain(0..i + 1);
            }
            curr.push(c);
            max_len = max(curr.len(), max_len);
        }
        max_len as i32
    }
}
// @lc code=end

#[test]
fn test3() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abcabcbb")),
        3
    );
}

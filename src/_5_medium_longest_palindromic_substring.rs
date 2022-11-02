/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let mut start = 0;
        let mut max_len = 1;
        let mut i = 0;

        let chars = s.chars().collect::<Vec<char>>();

        while i < s.len() {
            let mut left = i;
            let mut right = i;

            while right < s.len() - 1 && chars.get(right) == chars.get(right + 1) {
                right += 1;
            }

            i = right + 1;

            while right < s.len() - 1 && left > 0 && chars.get(right + 1) == chars.get(left - 1) {
                left -= 1;
                right += 1;
            }

            if max_len < right - left + 1 {
                start = left;
                max_len = right - left + 1;
            }
        }

        String::from(&s[start..start + max_len])
    }
}
// @lc code=end

#[test]
fn test5() {
    assert_eq!(
        Solution::longest_palindrome(String::from("babad")),
        String::from("bab")
    );
    assert_eq!(
        Solution::longest_palindrome(String::from("cbbd")),
        String::from("bb")
    );
}

/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result: i32 = 0;

        if x == i32::MIN {
            return 0;
        }

        while x.abs() > 0 {
            if let Some(sum) = result.checked_mul(10).and_then(|r| r.checked_add(x % 10)) {
                result = sum;
            } else {
                return 0;
            }
            x /= 10;
        }

        result
    }
}
// @lc code=end

#[test]
fn test7() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(1), 1);
    assert_eq!(Solution::reverse(i32::MIN), 0);
    assert_eq!(Solution::reverse(i32::MAX), 0);
}

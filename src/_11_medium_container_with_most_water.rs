/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

use std::borrow::Borrow;

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;

        let mut l = 0;
        let mut r = height.len() - 1;

        while l < r {
            max = max.max(height[l].min(height[r]) as usize * (r - l));
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max as i32
    }
}
// @lc code=end

#[test]
fn test11() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}

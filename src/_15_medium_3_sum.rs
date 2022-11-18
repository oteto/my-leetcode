/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

struct Solution {}

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = vec![];

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                if left > i + 1 && nums[left] == nums[left - 1] {
                    left += 1;
                    continue;
                }
                if right < nums.len() - 1 && nums[right] == nums[right + 1] {
                    right -= 1;
                    continue;
                }

                let sum = nums[i] + nums[left] + nums[right];
                if sum == 0 {
                    res.push([nums[i], nums[left], nums[right]].to_vec());
                    left += 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        res
    }
}
// @lc code=end

#[test]
fn test15() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, 0, 1], vec![-1, -1, 2]]
    );
}

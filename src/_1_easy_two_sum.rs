/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ret = Vec::with_capacity(2);
        let mut tmp = Vec::with_capacity(nums.len());
        for (i, &num1) in nums.iter().enumerate() {
            for (j, &num2) in tmp.iter().enumerate() {
                if num1 + num2 == target {
                    ret.push(j as i32);
                    ret.push(i as i32);
                    break;
                }
            }
            tmp.push(num1);
        }
        ret
    }
}
// @lc code=end

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if target < nums[0] {
            return 0;
        }

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        l as i32
    }
}
// @lc code=end

#[test]
fn test_35() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
}

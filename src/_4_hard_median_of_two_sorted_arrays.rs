/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while nums1.len() > i && nums2.len() > j {
            if nums1[i] < nums2[j] {
                nums.push(nums1[i]);
                i += 1;
            } else {
                nums.push(nums2[j]);
                j += 1;
            }
        }
        while nums1.len() > i {
            nums.push(nums1[i]);
            i += 1;
        }
        while nums2.len() > j {
            nums.push(nums2[j]);
            j += 1;
        }

        let len = i + j;
        if len % 2 == 0 {
            return (nums[len / 2] + nums[len / 2 - 1]) as f64 / 2 as f64;
        }
        nums[len / 2] as f64
    }
}
// @lc code=end

#[test]
fn test4() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
}

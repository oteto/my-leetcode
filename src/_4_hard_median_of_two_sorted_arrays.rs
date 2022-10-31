/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut i = 0;
        let mut j = 0;
        let mid = ((nums1.len() + nums2.len()) / 2) as i32;
        let mut ret = 0.0;

        if nums1.len() == 0 && nums2.len() == 1 {
            return nums2[0] as f64;
        }
        if nums2.len() == 0 && nums1.len() == 1 {
            return nums1[0] as f64;
        }

        let is_even = (nums1.len() + nums2.len()) % 2 == 0;
        let mut f = false;

        while nums1.len() > i || nums2.len() > j {
            let n1 = nums1.get(i).map_or(i32::MAX, |v| *v);
            let n2 = nums2.get(j).map_or(i32::MAX, |v| *v);
            if mid == (i + j) as i32 {
                ret += std::cmp::min(n1, n2) as f64;
                break;
            }
            if is_even && mid - 1 == (i + j) as i32 {
                ret += std::cmp::min(n1, n2) as f64;
                f = true;
            }
            if n1 == 1001 {
                j += 1;
                continue;
            }
            if n2 == 1001 {
                i += 1;
                continue;
            }
            if n1 > n2 {
                j += 1;
            } else {
                i += 1;
            }
        }

        match f {
            true => ret / 2.0,
            false => ret,
        }
    }
}
// @lc code=end

#[test]
fn test4() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![2, 3]), 2.5);
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![100000], vec![100001]),
        100000.5
    );
}

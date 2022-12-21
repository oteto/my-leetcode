/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}
// @lc code=end

#[test]
fn test_27() {
    let mut nums = vec![3, 2, 2, 3];
    assert_eq!(Solution::remove_element(&mut nums, 3), 2);
    let (got, _) = nums.split_at(2);
    assert_eq!(got.iter().filter(|x| **x == 2).collect::<Vec<_>>().len(), 2);

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(Solution::remove_element(&mut nums, 2), 5);
    let (got, _) = nums.split_at(5);
    assert_eq!(got.iter().filter(|x| **x == 0).collect::<Vec<_>>().len(), 2);
    assert_eq!(got.iter().filter(|x| **x == 1).collect::<Vec<_>>().len(), 1);
    assert_eq!(got.iter().filter(|x| **x == 3).collect::<Vec<_>>().len(), 1);
    assert_eq!(got.iter().filter(|x| **x == 4).collect::<Vec<_>>().len(), 1);

    let mut nums = vec![1, 1];
    assert_eq!(Solution::remove_element(&mut nums, 1), 0);

    let mut nums = vec![2, 2, 3];
    assert_eq!(Solution::remove_element(&mut nums, 2), 1);
    let (got, _) = nums.split_at(1);
    assert_eq!(got[0], 3);

    let mut nums = vec![0, 4, 4, 0, 4, 4, 4, 0, 2];
    assert_eq!(Solution::remove_element(&mut nums, 4), 4);
    let (got, _) = nums.split_at(4);
    assert_eq!(got.iter().filter(|x| **x == 0).collect::<Vec<_>>().len(), 3);
    assert_eq!(got.iter().filter(|x| **x == 2).collect::<Vec<_>>().len(), 1);
}

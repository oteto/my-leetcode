/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.clone()
            .iter()
            .fold((i32::MIN, 0), |(v, a), c| {
                if v != *c {
                    nums[a as usize] = *c;
                    return (*c, a + 1);
                }
                (*c, a)
            })
            .1
    }
}
// @lc code=end

#[test]
fn test_26() {
    let mut v = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut v), 2);
    let v = v.iter().take(2).map(|a| *a).collect::<Vec<_>>();
    assert_eq!(v, vec![1, 2]);

    let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut v), 5);
    let v = v.iter().take(5).map(|a| *a).collect::<Vec<_>>();
    assert_eq!(v, vec![0, 1, 2, 3, 4]);
}

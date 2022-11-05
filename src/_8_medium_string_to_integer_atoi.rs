/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let chars = s.trim().chars().collect::<Vec<_>>();
        let mut ret: i32 = 0;

        match chars.get(0) {
            Some('-') => {
                let mut chars = chars.iter().skip(1);
                while let Some(c) = chars.next() {
                    if let Some(i) = c.to_digit(10) {
                        if let Some(r) = ret.checked_mul(10).and_then(|r| r.checked_add(i as i32)) {
                            ret = r;
                        } else {
                            return i32::MIN;
                        }
                    } else {
                        break;
                    }
                }
                -ret
            }
            Some(_) => {
                let mut chars = if chars[0] == '+' {
                    chars.iter().skip(1)
                } else {
                    chars.iter().skip(0)
                };
                while let Some(c) = chars.next() {
                    if let Some(i) = c.to_digit(10) {
                        if let Some(r) = ret.checked_mul(10).and_then(|r| r.checked_add(i as i32)) {
                            ret = r;
                        } else {
                            return i32::MAX;
                        }
                    } else {
                        break;
                    }
                }
                ret
            }
            None => ret,
        }
    }
}
// @lc code=end

#[test]
fn test8() {
    assert_eq!(Solution::my_atoi(String::from("  -423")), -423);
    assert_eq!(Solution::my_atoi(String::from("42")), 42);
    assert_eq!(Solution::my_atoi(String::from("")), 0);
    assert_eq!(Solution::my_atoi(String::from(" ++1")), 0);
}

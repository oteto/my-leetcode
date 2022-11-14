/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            let s = strs.get(0).unwrap();
            return s.to_string();
        }
        let mut i = 0;
        let mut char = ' ';
        let mut ret = String::new();
        let len = strs.len();
        loop {
            for (j, s) in strs.iter().enumerate() {
                if let Some(c) = s.chars().nth(i) {
                    if j == 0 {
                        char = c;
                    } else {
                        if char != c {
                            return ret;
                        }
                        if j == len - 1 {
                            ret.push(c)
                        }
                    }
                } else {
                    return ret;
                }
            }

            i += 1;
        }
        ret
    }
}
// @lc code=end

#[test]
fn test14() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")
        ]),
        String::from("fl")
    );

    assert_eq!(
        Solution::longest_common_prefix(vec![
            String::from("dog"),
            String::from("cat"),
            String::from("hoge")
        ]),
        String::from("")
    );
}

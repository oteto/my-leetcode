/*
 * @lc app=leetcode id=10 lang=rust
 *
 * [10] Regular Expression Matching
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        fn is_match_str(s: &str, p: &str) -> bool {
            let (s_len, p_len) = (s.len(), p.len());
            if p_len == 0 {
                return s_len == 0;
            }

            let m = { s_len > 0 && (s.as_bytes()[0] == p.as_bytes()[0] || p.as_bytes()[0] == 46) };
            if p_len >= 2 && p.as_bytes()[1] == 42 {
                return is_match_str(s, &p[2..]) || (m && is_match_str(&s[1..], p));
            }

            m && is_match_str(&s[1..], &p[1..])
        }
        is_match_str(&s, &p)
    }
}
// @lc code=end

#[test]
fn test10() {
    assert_eq!(
        Solution::is_match(String::from("aa"), String::from("a")),
        false
    );
    assert_eq!(
        Solution::is_match(String::from("aa"), String::from("a.")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("abcd"), String::from(".*")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("abcd"), String::from(".*c")),
        false
    );
    assert_eq!(
        Solution::is_match(String::from("abcd"), String::from(".*cd")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("aab"), String::from("c*a*b")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("aa"), String::from("a*c*a")),
        true
    );
    assert_eq!(
        Solution::is_match(String::from("aaba"), String::from("ab*a*c*a")),
        false
    );
}

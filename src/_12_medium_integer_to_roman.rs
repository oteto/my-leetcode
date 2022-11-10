/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

struct Solution {}

// @lc code=start
use std::iter::FromIterator;
use std::ops::Add;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut s = num.to_string();
        s.chars()
            .rev()
            .enumerate()
            .fold(String::new(), |acc, (d, n)| match d {
                0 => Solution::conv(n.to_digit(10).unwrap(), 'I', 'V', 'X').add(&acc),
                1 => Solution::conv(n.to_digit(10).unwrap(), 'X', 'L', 'C').add(&acc),
                2 => Solution::conv(n.to_digit(10).unwrap(), 'C', 'D', 'M').add(&acc),
                3 => Solution::conv(n.to_digit(10).unwrap(), 'M', '_', '_').add(&acc),
                _ => panic!(),
            })
    }

    fn conv(c: u32, one: char, five: char, ten: char) -> String {
        match c {
            4 => String::from_iter(vec![one, five]),
            9 => String::from_iter(vec![one, ten]),
            _ => {
                let mut s = vec![];
                let mut c = c;
                if 5 <= c {
                    s.push(five);
                    c -= 5;
                }
                while 0 < c {
                    s.push(one);
                    c -= 1;
                }
                String::from_iter(s)
            }
        }
    }
}
// @lc code=end

#[test]
fn test12() {
    assert_eq!(Solution::int_to_roman(3), String::from("III"));
    assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
    assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
}

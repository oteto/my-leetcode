/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

struct Solution {}

// @lc code=start
use std::{iter::Peekable, str::Chars};

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut r = 0;
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                'I' => r += Solution::f('I', 'V', 'X', 1, &mut chars),
                'X' => r += Solution::f('X', 'L', 'C', 10, &mut chars),
                'C' => r += Solution::f('C', 'D', 'M', 100, &mut chars),
                'M' => r += Solution::f('M', '\0', '\0', 1000, &mut chars),
                'V' => r += 5,
                'L' => r += 50,
                'D' => r += 500,
                _ => panic!(),
            }
        }
        r
    }

    fn f(one: char, five: char, ten: char, digit: i32, chars: &mut Peekable<Chars>) -> i32 {
        let mut r = digit;
        if let Some(&p) = chars.peek() {
            if p == five {
                r = digit * 4;
                chars.next();
            } else if p == ten {
                r = digit * 9;
                chars.next();
            } else {
                loop {
                    if let Some(&p) = chars.peek() {
                        if p == one {
                            r += digit;
                            chars.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        r
    }
}
// @lc code=end

#[test]
fn test13() {
    assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    assert_eq!(Solution::roman_to_int(String::from("DCXXI")), 621);
}

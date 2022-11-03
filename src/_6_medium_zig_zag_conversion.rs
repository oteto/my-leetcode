use core::num;

/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] ZigZag Conversion
 */
struct Solution {}

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let chars = s.chars().collect::<Vec<char>>();
        let mut ret = String::with_capacity(chars.len());

        let m = 2 * (num_rows - 1);
        let mut p = 2 * (num_rows - 1);
        let mut index = 0;
        let mut row = 0;
        let mut f = true;

        while 0 <= p {
            match chars.get(index) {
                Some(&c) => {
                    ret.push(c);
                    if m == p || p == 0 {
                        index += m as usize;
                    } else {
                        match f {
                            true => {
                                index += p as usize;
                                f = !f;
                            }
                            false => {
                                index += (m - p) as usize;
                                f = !f;
                            }
                        }
                    }
                }
                None => {
                    p -= 2;
                    row += 1;
                    index = row;
                    f = true;
                }
            }
        }

        ret
    }
}
// @lc code=end

#[test]
fn test6() {
    assert_eq!(
        con(String::from("PAYPALISHIRING"), 4),
        String::from("PINALSIGYAHRPI")
    );

    assert_eq!(con(String::from("A"), 1), String::from("A"));
}

fn con(s: String, row: i32) -> String {
    let mut v: Vec<_> = (0..row)
        .chain((1..row - 1).rev())
        .cycle()
        .zip(s.chars())
        .collect();
    v.sort_by_key(|&(r, _)| r);
    v.iter().map(|(_, c)| c).collect()
}

// https://leetcode.cn/problems/zigzag-conversion/

use std::convert::TryInto;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let s: Vec<_> = s.chars().collect();
        let num_rows: usize = num_rows.try_into().unwrap();
        let mut result = String::with_capacity(s.len());
        let duration = 2 * (num_rows - 1);

        for row in 0..num_rows {
            match row {
                0 => s.iter().step_by(duration).for_each(|ch| result.push(*ch)),
                _ if row == num_rows - 1 => {
                    if s.len() >= num_rows {
                        s[num_rows - 1..]
                            .iter()
                            .step_by(duration)
                            .for_each(|ch| result.push(*ch))
                    }
                }
                _ => (row..s.len()).step_by(duration).for_each(|i| {
                    result.push(s[i]);
                    let r = i % duration;
                    let i = i + duration - 2 * r;
                    if i < s.len() {
                        result.push(s[i])
                    }
                }),
            }
        }

        result
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zigzag_conversion() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        )
    }
}

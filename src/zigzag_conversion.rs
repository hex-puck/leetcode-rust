// https://leetcode-cn.com/problems/zigzag-conversion/
#![allow(dead_code)]

use std::{convert::TryInto, hint};

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }

        let s: Vec<_> = s.chars().collect();
        let num_rows: usize = num_rows.try_into().unwrap();
        let mut rows = Vec::<Vec<char>>::with_capacity(num_rows);
        for _ in 0..num_rows {
            rows.push(Vec::with_capacity(s.len()));
        }
        let duration = 2 * (num_rows - 1);

        for i in 0..s.len() {
            let remainder = i % duration;
            match remainder {
                _ if remainder < num_rows => rows[remainder].push(s[i]),
                _ if remainder >= num_rows => rows[duration - remainder].push(s[i]),
                _ => unsafe { hint::unreachable_unchecked() },
            }
        }

        let mut result = String::with_capacity(s.len());
        for row in rows.iter() {
            for ch in row.iter() {
                result.push(*ch);
            }
        }

        result
    }
}

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

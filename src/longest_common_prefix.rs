// https://leetcode-cn.com/problems/longest-common-prefix/
#![allow(dead_code)]

use std::cmp;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::from("");
        }

        let mut iter = strs.iter();
        let mut result = &iter.next().unwrap()[..];
        if result.is_empty() {
            return String::new();
        }

        for s in iter {
            let mut length = 0;
            for i in 1..cmp::min(s.len(), result.len()) + 1 {
                if s.starts_with(&result[..i]) {
                    length = i;
                } else {
                    break;
                }
            }
            result = &result[0..length];
            if result.is_empty() {
                return String::new();
            }
        }

        String::from(result)
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            String::new()
        );
    }
}

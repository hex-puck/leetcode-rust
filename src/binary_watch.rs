// https://leetcode-cn.com/problems/binary-watch/
#![allow(dead_code)]

use std::{collections::HashMap, convert::TryInto};

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut result = vec![];
        // number of ones vs. list of hours
        let mut hash = HashMap::<u8, Vec<u8>>::with_capacity(4);

        for i in 0..12u8 {
            if let Some(vec) = hash.get_mut(&(i.count_ones() as u8)) {
                vec.push(i);
            } else {
                hash.insert(i.count_ones() as u8, vec![i]);
            }
        }

        for i in 0..60u8 {
            let j = turned_on - i.count_ones() as i32;
            if let Err(_) = <i32 as TryInto<u8>>::try_into(j) {
                continue;
            }
            let j = j as u8;
            if j == 0 {
                result.push(format!("0:{:02}", i))
            } else if let Some(vec) = hash.get(&j) {
                vec.iter().for_each(|h| {
                    result.push(format!("{}:{:02}", h, i));
                });
            }
        }

        result
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_binary_watch() {
        let mut vec = Solution::read_binary_watch(1);
        vec.sort();
        assert_eq!(
            vec,
            vec![
                "0:01".to_string(),
                "0:02".to_string(),
                "0:04".to_string(),
                "0:08".to_string(),
                "0:16".to_string(),
                "0:32".to_string(),
                "1:00".to_string(),
                "2:00".to_string(),
                "4:00".to_string(),
                "8:00".to_string()
            ]
        );
        assert_eq!(Solution::read_binary_watch(9), Vec::<String>::new());
    }
}

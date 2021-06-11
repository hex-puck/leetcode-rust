// https://leetcode-cn.com/problems/two-sum/
#![allow(dead_code)]

use std::{collections::HashMap, hint};

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache = HashMap::new();

        for (index, val) in nums.iter().enumerate() {
            let v = target - *val;
            if cache.contains_key(&v) {
                return vec![cache[&v] as i32, index as i32];
            } else {
                cache.insert(*val, index);
            }
        }

        unsafe {
            hint::unreachable_unchecked();
        }
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1])
    }
}

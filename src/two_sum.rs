// https://leetcode-cn.com/problems/two-sum/
#![allow(dead_code)]

use std::{convert::TryInto, hint};

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i.try_into().unwrap(), j.try_into().unwrap()];
                }
            }
        }

        unsafe {
            hint::unreachable_unchecked();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1])
    }
}

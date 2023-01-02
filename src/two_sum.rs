// https://leetcode.cn/problems/two-sum/

use std::{
    collections::{hash_map::Entry, HashMap},
    hint,
};

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut cache = HashMap::new();

        for (index, value) in nums.into_iter().enumerate() {
            match cache.entry(target - value) {
                Entry::Occupied(oe) => {
                    return vec![*oe.get() as i32, index as i32];
                }
                Entry::Vacant(_) => {
                    cache.insert(value, index);
                }
            }
        }

        unsafe {
            hint::unreachable_unchecked();
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1])
    }
}

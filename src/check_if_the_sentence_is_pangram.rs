// https://leetcode.cn/problems/check-if-the-sentence-is-pangram/

use std::collections::HashSet;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        const LETTER_CNT: usize = 26;
        let mut set = HashSet::with_capacity(LETTER_CNT);
        for ch in sentence.chars() {
            set.insert(ch);
            if set.len() == LETTER_CNT {
                return true;
            }
        }

        false
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_if_pangram() {
        assert!(Solution::check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_string()
        ));
        assert!(!Solution::check_if_pangram("leetcode".to_string()));
    }
}

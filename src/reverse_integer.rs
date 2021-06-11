// https://leetcode-cn.com/problems/reverse-integer/
#![allow(dead_code)]

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let signum = x.signum();
        let mut x = x.abs();

        let base = 10;
        let mut rem = x % base;
        x /= base;
        let mut result: i32 = rem;

        while x != 0 {
            rem = x % base;
            x /= base;
            result = match result.checked_mul(base) {
                Some(result) => result,
                None => return 0,
            };
            result = match result.checked_add(rem) {
                Some(result) => result,
                None => return 0,
            };
        }

        match result.checked_mul(signum) {
            Some(result) => result,
            None => 0,
        }
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reverse_test() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
    }
}

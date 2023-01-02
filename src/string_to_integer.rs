// https://leetcode.cn/problems/string-to-integer-atoi/

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s.trim_start();
        if s.is_empty() {
            return 0;
        }

        let first_char = s.chars().next().unwrap();
        let is_negtive = match first_char {
            '+' => {
                s = &s[1..];
                false
            }
            '-' => {
                s = &s[1..];
                true
            }
            _ => false,
        };

        let mut result: i32 = 0;
        let get_limits = || {
            if is_negtive {
                i32::MIN
            } else {
                i32::MAX
            }
        };

        for ch in s.chars() {
            if !ch.is_digit(10) {
                break;
            }

            result = match result.checked_mul(10) {
                Some(v) => v,
                None => return get_limits(),
            };
            result = match result.checked_add(ch.to_digit(10).unwrap() as i32) {
                Some(v) => v,
                None => return get_limits(),
            };
        }

        if is_negtive {
            // 不可能溢出
            result *= -1;
        }

        result
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }
}

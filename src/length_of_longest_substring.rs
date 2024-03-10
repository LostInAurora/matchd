use std::cmp::max;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut max_length, mut left) = (0, 0);
        let mut recorded_map = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        for (index, c) in chars.iter().enumerate() {
            if let Some(saved_index) = recorded_map.get(c) {
                left = max(left, saved_index + 1)
            }
            recorded_map.insert(*c, index);
            max_length = max(max_length, index - left + 1);
        }
        max_length as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring_1() {
        let r = Solution::length_of_longest_substring(String::from("abcabcbb"));
        assert_eq!(r, 3);
    }

    #[test]
    fn test_length_of_longest_substring_2() {
        let r = Solution::length_of_longest_substring(String::from("bbbbb"));
        assert_eq!(r, 1);
    }

    #[test]
    fn test_length_of_longest_substring_3() {
        let r = Solution::length_of_longest_substring(String::from("pwwkew"));
        assert_eq!(r, 3);
    }
}
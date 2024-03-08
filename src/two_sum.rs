use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut recorded_map = HashMap::new();
        for (index, v) in nums.iter().enumerate() {
            match recorded_map.get(&(target - v)) {
                Some(&saved_index) => return vec![saved_index as i32, index as i32],
                None => {
                    recorded_map.insert(v, index);
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}

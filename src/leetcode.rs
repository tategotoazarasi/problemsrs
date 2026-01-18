// src/leetcode.rs

use std::collections::HashMap;

/// Struct to hold LeetCode solutions, mimicking the namespace style in C++.
pub struct Solution;

impl Solution {
    /// # 1. Two Sum
    ///
    /// Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.
    /// You may assume that each input would have exactly one solution, and you may not use the same element twice.
    /// You can return the answer in any order.
    ///
    /// ## Arguments
    ///
    /// * `nums` - A vector of integers.
    /// * `target` - The target sum.
    ///
    /// ## Returns
    ///
    /// A vector containing the indices of the two numbers.
    ///
    /// ## Example
    ///
    /// ```
    /// use problemsrs::leetcode::Solution;
    /// let nums = vec![2, 7, 11, 15];
    /// let target = 9;
    /// assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    /// ```
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Use a HashMap to store the complement value and its index.
        // Key: target - current_value, Value: index of current_value
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            // Check if the current number is the complement we are looking for
            if let Some(&complement_index) = map.get(&num) {
                return vec![complement_index, i as i32];
            }
            // Store the required complement for the current number
            map.insert(target - num, i as i32);
        }

        // Should not be reached based on problem constraints
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_case1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        // The problem allows any order, but our implementation guarantees strictly increasing indices [0, 1]
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_case2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_two_sum_case3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}

// src/leetcode.rs

use std::cmp::min;
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

pub struct Solution2976;

impl Solution2976 {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut min_cost: [[i64; 26]; 26] = [[i64::MAX / 2 - 1; 26]; 26];
        for i in 0..original.len() {
            min_cost[original[i] as usize - 'a' as usize][changed[i] as usize - 'a' as usize] = min(
                min_cost[original[i] as usize - 'a' as usize][changed[i] as usize - 'a' as usize],
                cost[i] as i64,
            );
        }
        for (i, row) in min_cost.iter_mut().enumerate() {
            row[i] = 0;
        }
        for j in 'a'..='z' {
            for i in 'a'..='z' {
                for k in 'a'..='z' {
                    min_cost[i as usize - 'a' as usize][k as usize - 'a' as usize] = min(
                        min_cost[i as usize - 'a' as usize][k as usize - 'a' as usize],
                        min_cost[i as usize - 'a' as usize][j as usize - 'a' as usize]
                            + min_cost[j as usize - 'a' as usize][k as usize - 'a' as usize],
                    );
                }
            }
        }
        let mut ans: i64 = 0;
        let s_bytes = source.as_bytes();
        let t_bytes = target.as_bytes();
        for i in 0..s_bytes.len() {
            let s = s_bytes[i];
            let t = t_bytes[i];
            if s == t {
                continue;
            }
            if min_cost[s as usize - 'a' as usize][t as usize - 'a' as usize] == i64::MAX / 2 - 1 {
                return -1;
            }
            ans += min_cost[s as usize - 'a' as usize][t as usize - 'a' as usize];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_cost_case1() {
        let result = Solution2976::minimum_cost(
            "abcd".parse().unwrap(),
            "acbe".parse().unwrap(),
            vec!['a', 'b', 'c', 'c', 'e', 'd'],
            vec!['b', 'c', 'b', 'e', 'b', 'e'],
            vec![2, 5, 5, 1, 2, 20],
        );
        assert_eq!(result, 28);
    }

    #[test]
    fn test_minimum_cost_case2() {
        let result = Solution2976::minimum_cost(
            "aaaa".parse().unwrap(),
            "bbbb".parse().unwrap(),
            vec!['a', 'c'],
            vec!['c', 'b'],
            vec![1, 2],
        );
        assert_eq!(result, 12);
    }

    #[test]
    fn test_minimum_cost_case3() {
        let result = Solution2976::minimum_cost(
            "abcd".parse().unwrap(),
            "abce".parse().unwrap(),
            vec!['a'],
            vec!['e'],
            vec![10000],
        );
        assert_eq!(result, -1);
    }

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

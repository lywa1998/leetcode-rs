#![allow(unused)]
struct Solution;

// Given an array `nums` of size `n`, return *the majority element*.
//
// The majority element is the element that appears more than `⌊n / 2⌋` times. You may assume that the majority element always exists in the array.
//
// **Example 1:**
//
// ```
// Input: nums = [3,2,3]
// Output: 3
//
// ```
//
// **Example 2:**
//
// ```
// Input: nums = [2,2,1,1,1,2,2]
// Output: 2
//
// ```
//
// **Constraints:**
//
// * `n == nums.length`
// * `1 <= n <= 5 * 10<sup>4</sup>`
// * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
//
// **Follow-up:** Could you solve the problem in linear time and in `O(1)` space?

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 2, 3];
        let ans = 3;
        let res = Solution::majority_element(nums);
        assert_eq!(res, ans);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let ans = 2;
        let res = Solution::majority_element(nums);
        assert_eq!(res, ans);
    }
}

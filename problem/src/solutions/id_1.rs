struct Solution;

// Given an array of integers `nums` and an integer `target`, return *indices of the two numbers such that they add up to `target`*.
//
// You may assume that each input would have ***exactly* one solution**, and you may not use the *same* element twice.
//
// You can return the answer in any order.
//
// **Example 1:**
//
// ```
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// ```
//
// **Example 2:**
//
// ```
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// ```
//
// **Example 3:**
//
// ```
// Input: nums = [3,3], target = 6
// Output: [0,1]
// ```
//
// **Constraints:**
//
// * `2 <= nums.length <= 10<sup>4</sup>`
// * `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`
// * `-10<sup>9</sup> <= target <= 10<sup>9</sup>`
// * **Only one valid answer exists.**
//
// **Follow-up:** Can you come up with an algorithm that is less than `O(n<sup>2</sup>)` time complexity?

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        unimplemented!()
    }
}

#[cfg(test)]
mod id_1_tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let ans = vec![0, 1];
        let res = Solution::two_sum(nums, target);
        assert_eq!(res, ans);
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let ans = vec![1, 2];
        let res = Solution::two_sum(nums, target);
        assert_eq!(res, ans);
    }

    #[test]
    fn example3() {
        let nums = vec![3, 3];
        let target = 6;
        let ans = vec![0, 1];
        let res = Solution::two_sum(nums, target);
        assert_eq!(res, ans);
    }
}

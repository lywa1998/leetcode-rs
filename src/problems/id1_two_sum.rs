use crate::Solution;

/**
 * 1. 两数之和
 * 
 * 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
 * 
 * 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
 * 
 * 你可以按任意顺序返回答案。
 * 
 * 
 * 示例 1：
 * 
 * > 输入：nums = [2,7,11,15], target = 9
 * > 输出：[0,1]
 * > 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
 *
 * 示例 2：
 * 
 * > 输入：nums = [3,2,4], target = 6
 * > 输出：[1,2]
 * 
 * 示例 3：
 * 
 * > 输入：nums = [3,3], target = 6
 * > 输出：[0,1]
 */

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let res = Solution::two_sum(nums, target);
        assert_eq!(res, vec![0, 1]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        let res = Solution::two_sum(nums, target);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![3, 3];
        let target = 6;
        
        let res = Solution::two_sum(nums, target);
        assert_eq!(res, vec![0, 1]);
    }
}
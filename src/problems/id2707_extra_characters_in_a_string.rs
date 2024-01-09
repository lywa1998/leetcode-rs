use crate::Solution;

/**
 * 2707. 字符串中的额外字符
 * 
 * 给你一个下标从 0 开始的字符串 s 和一个单词字典 dictionary 。
 * 你需要将 s 分割成若干个 互不重叠 的子字符串，每个子字符串都在 dictionary 中出现过。
 * s 中可能会有一些 额外的字符 不在任何子字符串中。
 * 
 * 请你采取最优策略分割 s ，使剩下的字符 最少 。
 * 
 * 
 * 示例 1：
 *
 * > 输入：s = "leetscode", dictionary = ["leet","code","leetcode"]
 * > 输出：1
 * > 解释：将 s 分成两个子字符串：下标从 0 到 3 的 "leet" 和下标从 5 到 8 的 "code" 。
 * > 只有 1 个字符没有使用（下标为 4），所以我们返回 1 。
 *
 * 示例 2：
 * 
 * > 输入：s = "sayhelloworld", dictionary = ["hello","world"]
 * > 输出：3
 * > 解释：将 s 分成两个子字符串：下标从 3 到 7 的 "hello" 和下标从 8 到 12 的 "world" 。
 * > 下标为 0 ，1 和 2 的字符没有使用，所以我们返回 3 。
 */

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "leetscode".to_string();
        let dictionary = vec!["leet".to_string(), "code".to_string(), "leetcode".to_string()];
        let ans = Solution::min_extra_char(s, dictionary);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test_example_2() {
        let s = "sayhelloworld".to_string();
        let dictionary = vec!["hello".to_string(), "world".to_string()];
        let ans = Solution::min_extra_char(s, dictionary);
        assert_eq!(ans, 3);
    }
}

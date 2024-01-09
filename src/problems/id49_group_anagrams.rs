use crate::Solution;

/**
 * 49. 字母异位词分组
 *
 * 给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
 * 
 * 字母异位词 是由重新排列源单词的所有字母得到的一个新单词。
 * 
 * 
 * 示例 1:
 * 
 * > 输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
 * > 输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
 *
 * 示例 2:
 * 
 * > 输入: strs = [""]
 * > 输出: [[""]]
 * 
 * 示例 3:
 * 
 * > 输入: strs = ["a"]
 * > 输出: [["a"]]
 */


impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let strs = ["eat","tea","tan","ate","nat","bat"]
            .iter()
            .map(|&str| str.to_owned())
            .collect::<Vec<String>>();
        let ans = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];

        let res = Solution::group_anagrams(strs);
        assert_eq!(res, ans);
    }

    #[test]
    fn test_example_2() {
        let strs = vec!["".to_string()];
        let ans = vec![vec!["".to_string()]];

        let res = Solution::group_anagrams(strs);
        assert_eq!(res, ans);
    }

    #[test]
    fn test_example_3() {
        let strs = vec!["a".to_string()];
        let ans = vec![vec!["a".to_string()]];

        let res = Solution::group_anagrams(strs);
        assert_eq!(res, ans);
    }
}
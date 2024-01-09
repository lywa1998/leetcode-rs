use crate::Solution;

/**
 * 2707. Extra Characters in a String
 * 
 * You are given a 0-indexed string s and a dictionary of words dictionary. 
 * You have to break s into one or more non-overlapping substrings such that each substring is present in dictionary. 
 * There may be some extra characters in s which are not present in any of the substrings.
 * 
 * Return the minimum number of extra characters left over if you break up s optimally.
 * 
 * 
 * Example 1:
 * 
 * Input: s = "leetscode", dictionary = ["leet","code","leetcode"]
 * Output: 1
 * Explanation: We can break s in two substrings: "leet" from index 0 to 3 and "code" from index 5 to 8. 
 * There is only 1 unused character (at index 4), so we return 1.
 * 
 * Example 2:
 * 
 * Input: s = "sayhelloworld", dictionary = ["hello","world"]
 * Output: 3
 * Explanation: We can break s in two substrings: "hello" from index 3 to 7 and "world" from index 8 to 12. 
 * The characters at indices 0, 1, 2 are not used in any substring and thus are considered as extra characters. 
 * Hence, we return 3.
 */

use std::collections::HashSet;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let word_set = dictionary.iter()
            .map(|word| word.to_owned())
            .collect::<HashSet<_>>();

        let n = s.len();
        let mut f: Vec<i32> = vec![0i32; n+1];
        for i in 1..=n {
            f[i] = f[i-1] + 1;
            for j in 0..i {
                if word_set.contains(&s[j..i]) {
                    f[i] = f[i].min(f[j]);
                }
            }
        }
        f[n]
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

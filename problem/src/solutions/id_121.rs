#![allow(unused)]
struct Solution;

// You are given an array `prices` where `prices[i]` is the price of a given stock on the `i<sup>th</sup>` day.
// 
// You want to maximize your profit by choosing a **single day** to buy one stock and choosing a **different day in the future** to sell that stock.
// 
// Return *the maximum profit you can achieve from this transaction*. If you cannot achieve any profit, return `0`.
// 
// **Example 1:**
// 
// ```
// Input: prices = [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
// 
// ```
// 
// **Example 2:**
// 
// ```
// Input: prices = [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transactions are done and the max profit = 0.
// 
// ```
// 
// **Constraints:**
// 
// * `1 <= prices.length <= 10<sup>5</sup>`
// * `0 <= prices[i] <= 10<sup>4</sup>`

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = i32::MAX;
        prices.iter()
            .fold(0, |profit, price| {
                buy = buy.min(*price);
                profit.max(*price - buy)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let output = 5;
        assert_eq!(Solution::max_profit(prices), output);
    }

    #[test]
    fn test_example_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let output = 0;
        assert_eq!(Solution::max_profit(prices), output);
    }
}
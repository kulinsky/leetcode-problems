// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/564/

// You are given an array prices where prices[i] is the price of a given stock on the ith day.
// Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).
// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
//
// Example 1:
// Input: prices = [7,1,5,3,6,4]
// Output: 7
// Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
// Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
//
// Constraints:
// 1 <= prices.length <= 3 * 104
// 0 <= prices[i] <= 104

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total_profit = 0;

        match prices.len() {
            0..=1 => total_profit,
            2 => if prices[1] - prices[0] > 0 { prices[1] - prices[0] } else { 0 },
            _ => {
                let mut min_price = -1;
                for i in 0..prices.len() {
                    if i == 0 && prices[i] <= prices[i + 1] {
                        min_price = prices[i];
                    } else if i == prices.len() - 1 && prices[i] >= prices[i - 1] && min_price != -1 {
                        total_profit += prices[i] - min_price;
                        min_price = -1;
                    } else if i > 0 && i < prices.len() - 1 {
                        if prices[i] <= prices[i - 1] && prices[i] <= prices[i + 1] {
                            min_price = prices[i];
                        } else if prices[i] >= prices[i - 1] && prices[i] >= prices[i + 1] && min_price != -1 {
                            total_profit += prices[i] - min_price;
                            min_price = -1;
                        }
                    }

                }

                total_profit
            }
        }
    }
}

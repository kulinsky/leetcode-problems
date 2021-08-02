// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/880/

// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
//
// Example:
// Input: x = 123
// Output: 321

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x < 0 {
            return match format!("{}", x * -1).chars().rev().collect::<String>().parse::<i32>() {
                Ok(val) => val * -1,
                Err(_) => 0
            };
        }

        match format!("{}", x).chars().rev().collect::<String>().parse::<i32>() {
            Ok(val) => val,
            Err(_) => 0
        }
    }
}

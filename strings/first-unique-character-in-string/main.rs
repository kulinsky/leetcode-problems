// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/881/

// Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.
//
// Example:
// Input: s = "leetcode"
// Output: 0

use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::new();

        for char in s.chars() {
            let count = map.entry(char).or_insert(0);
            *count += 1;
        }

        for (i, char) in s.chars().enumerate() {
            if let Some(c) = map.get(&char) {
                if *c == 1 {
                    return i as i32;
                }
            }
        }

        -1
    }
}

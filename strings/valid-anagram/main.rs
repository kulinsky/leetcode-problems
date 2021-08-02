// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/882/

// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
//
// Example:
// Input: s = "anagram", t = "nagaram"
// Output: true

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }

        let mut s_map = HashMap::new();
        let mut t_map = HashMap::new();

        for x in s.chars() {
            let mut count = s_map.entry(x).or_insert(0);
            *count += 1;
        }

        for x in t.chars() {
            let mut count = t_map.entry(x).or_insert(0);
            *count += 1;
        }

        t_map == s_map
    }
}

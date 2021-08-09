// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/885/discuss

// Implement strStr().
// Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
//
// Clarification:
// What should we return when needle is an empty string? This is a great question to ask during an interview.
// For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr() and Java's indexOf().
//
// Example 1:
// Input: haystack = "hello", needle = "ll"
// Output: 2

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let result = -1;

        if haystack == needle {
            return 0;
        }

        if needle == "" {
            return 0;
        }

        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();

        for i in 0..haystack.len() {
            let mut tmp = i;

            for j in 0..needle.len() {
                if haystack[tmp] == needle[j] {
                    if j == needle.len() - 1 {
                        return i as i32;
                    }

                    if tmp == haystack.len() - 1 {
                        return -1;
                    }

                    tmp += 1;

                    continue;
                } else {
                    break;
                }
            }
        }

        result
    }
}

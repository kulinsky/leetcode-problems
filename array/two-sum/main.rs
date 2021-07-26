// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/546/

// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.
//
// Example 1:
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Output: Because nums[0] + nums[1] == 9, we return [0, 1].
//
// Example 2:
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
//
// Example 3:
// Input: nums = [3,3], target = 6
// Output: [0,1]
//
// Constraints:
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.


use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut res = Vec::new();

        for (i, item) in nums.iter().enumerate() {
            match map.get(&(target - item)) {
                Some(value) => {
                    res.push(i as i32);
                    res.push(*value);

                    return res;
                },
                None => {
                    map.insert(*item, i as i32);
                }
            }
        }

        panic!("solution not found")
    }
}

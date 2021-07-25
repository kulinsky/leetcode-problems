// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/567/

// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
// Note that you must do this in-place without making a copy of the array.
//
// Example 1:
// Input: nums = [0,1,0,3,12]
// Output: [1,3,12,0,0]
//
// Example 2:
// Input: nums = [0]
// Output: [0]
//
// Constraints:
// 1 <= nums.length <= 104
// -231 <= nums[i] <= 231 - 1

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_index = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[zero_index] = nums[i];
                zero_index += 1;
            }
        }

        for i in zero_index..nums.len() {
            nums[i] = 0;
        }
    }
}

// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/646/

// Given an array, rotate the array to the right by k steps, where k is non-negative.
//
// Example 1:
// Input: nums = [1,2,3,4,5,6,7], k = 3
// Output: [5,6,7,1,2,3,4]
// Explanation:
// rotate 1 steps to the right: [7,1,2,3,4,5,6]
// rotate 2 steps to the right: [6,7,1,2,3,4,5]
// rotate 3 steps to the right: [5,6,7,1,2,3,4]

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let kf = (k as usize) % nums.len();
        reverse(nums, 0, nums.len() - 1);

        match kf {
            0 => reverse(nums, 0, 0),
            _ => reverse(nums, 0, kf - 1),
        }

        reverse(nums, kf, nums.len() - 1);

        fn reverse(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
            while start < end {
                let tmp = nums[start];
                nums[start] = nums[end];
                nums[end] = tmp;
                start += 1;
                end -= 1;
            }
        }
    }
}

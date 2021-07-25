// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/727/

// Given an integer array nums sorted in non-decreasing order,
// remove the duplicates in-place such that each unique element appears only once.
// The relative order of the elements should be kept the same.
// Since it is impossible to change the length of the array in some languages,
// you must instead have the result be placed in the first part of the array nums.
// More formally, if there are k elements after removing the duplicates,
// then the first k elements of nums should hold the final result.
// It does not matter what you leave beyond the first k elements.
// Return k after placing the final result in the first k slots of nums.
// Do not allocate extra space for another array.
// You must do this by modifying the input array in-place with O(1) extra memory.

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut tmp_idx = 0;

        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                if i + 1 == nums.len() - 1 {
                    nums[tmp_idx] = nums[i];
                    tmp_idx += 1;
                }
            } else {
                nums[tmp_idx] = nums[i];
                tmp_idx += 1;

                if i + 1 == nums.len() - 1 {
                    nums[tmp_idx] = nums[i + 1];
                    tmp_idx += 1;
                }
            }
        }

        for _ in tmp_idx..nums.len() {
            nums.pop();
        };

        nums.len() as i32
    }
}
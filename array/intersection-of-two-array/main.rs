// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/674/

// Given two integer arrays nums1 and nums2, return an array of their intersection.
// Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.
//
// Example 1:
// Input: nums1 = [1,2,2,1], nums2 = [2,2]
// Output: [2,2]
//
// Example 2:
// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// Output: [4,9]
// Explanation: [9,4] is also accepted.
//
// Constraints:
// 1 <= nums1.length, nums2.length <= 1000
// 0 <= nums1[i], nums2[i] <= 1000

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort();
        nums2.sort();
        let mut res = Vec::new();

        let mut first = 0;
        let mut second = 0;
        let mut blocked_to_add = false;

        for _ in 0..nums1.len() + nums2.len() {
            if first == nums1.len() - 1 && second == nums2.len() - 1 {
                if nums1[first] == nums2[second] {
                    if !blocked_to_add {
                        res.push(nums1[first]);
                    }
                }
                break;
            }

            if nums1[first] == nums2[second] {
                if !blocked_to_add {
                    res.push(nums1[first]);
                }

                if first < nums1.len() - 1 {
                    first += 1;
                } else {
                    blocked_to_add = true
                }

                if second < nums2.len() - 1 {
                    second += 1;
                } else {
                    blocked_to_add = true
                }

            } else {
                if nums1[first] > nums2[second] {
                    if second < nums2.len() - 1 {
                        second += 1;
                    } else {
                        blocked_to_add = true
                    }
                } else {
                    if first < nums1.len() - 1 {
                        first += 1;
                    } else {
                        blocked_to_add = true
                    }
                }
            }
        }

        res
    }
}

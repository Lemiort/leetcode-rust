/**
 * [283] Move Zeroes
 *
 * Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
 * Note that you must do this in-place without making a copy of the array.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [0,1,0,3,12]
 * Output: [1,3,12,0,0]
 * <strong class="example">Example 2:
 * Input: nums = [0]
 * Output: [0]
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 *
 *  
 * Follow up: Could you minimize the total number of operations done?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/move-zeroes/
// discuss: https://leetcode.com/problems/move-zeroes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
// 0,1,0,3,12
// i - nums - non_zero_index
// 0 - 0 1 0 3 12 - 1
// 1 - 1 0 0 3 12 - 3
// 2 - 1 3 0 0 12 - 4
// 3 - 1 3 12 0 0 - 5

// 0 - 1 0 - 0
// 1 - 0 1 - 0

// 0 - 1 0 0
// 1 - 0 1 0
//

// 0 - 1 0 1 - 2
// 1 - 1 1 0 - 3

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // Find the index of the first zero element.
        if let Some(mut non_zero_index) = nums.iter().position(|&x| x != 0) {
            let mut i = 0;
            while i < nums.len() && non_zero_index < nums.len() {
                if nums[i] == 0 && non_zero_index > i {
                    nums.swap(i, non_zero_index);
                    // find next non-zero
                    while non_zero_index < nums.len() && nums[non_zero_index] == 0 {
                        non_zero_index += 1;
                    }
                } else if non_zero_index <= i {
                    // we skipped non-zero, find next non-zero
                    non_zero_index += 1;
                    while non_zero_index < nums.len() && nums[non_zero_index] == 0 {
                        non_zero_index += 1;
                    }
                }
                i += 1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_283() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums2 = vec![0];
        Solution::move_zeroes(&mut nums2);
        assert_eq!(nums2, vec![0]);

        let mut nums3 = vec![1, 0];
        Solution::move_zeroes(&mut nums3);
        assert_eq!(nums3, vec![1, 0]);

        let mut nums4 = vec![1, 0, 0];
        Solution::move_zeroes(&mut nums4);
        assert_eq!(nums4, vec![1, 0, 0]);

        let mut nums5 = vec![1, 0, 1];
        Solution::move_zeroes(&mut nums5);
        assert_eq!(nums5, vec![1, 1, 0]);
    }
}

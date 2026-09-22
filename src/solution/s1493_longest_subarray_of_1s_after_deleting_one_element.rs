/**
 * [1493] Longest Subarray of 1's After Deleting One Element
 *
 * Given a binary array nums, you should delete one element from it.
 * Return the size of the longest non-empty subarray containing only 1's in the resulting array. Return 0 if there is no such subarray.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,1,0,1]
 * Output: 3
 * Explanation: After deleting the number in position 2, [1,1,1] contains 3 numbers with value of 1's.
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0,1,1,1,0,1,1,0,1]
 * Output: 5
 * Explanation: After deleting the number in position 4, [0,1,1,1,1,1,0,1] longest subarray with value of 1's is [1,1,1,1,1].
 *
 * <strong class="example">Example 3:
 *
 * Input: nums = [1,1,1]
 * Output: 2
 * Explanation: You must delete one element.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	nums[i] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/
// discuss: https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut zero_count = 0;
        let mut max_len = 0;

        for right in 0..nums.len() {
            if nums[right] == 0 {
                zero_count += 1;
            }

            // while window has more than one 0, move left border
            while zero_count > 1 {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }

            // basic window len: right - left + 1.
            // witout last element: (right - left + 1) - 1 = right - left.
            max_len = max_len.max(right - left);
        }

        max_len as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1493() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
        assert_eq!(
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
            5
        );
        assert_eq!(Solution::longest_subarray(vec![1, 1, 1]), 2);

        assert_eq!(
            Solution::longest_subarray(vec![1, 1, 0, 0, 1, 1, 1, 0, 1]),
            4
        );
    }
}

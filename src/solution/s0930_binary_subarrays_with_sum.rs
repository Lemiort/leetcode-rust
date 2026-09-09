/**
 * [930] Binary Subarrays With Sum
 *
 * Given a binary array nums and an integer goal, return the number of non-empty subarrays with a sum goal.
 * A subarray is a contiguous part of the array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,0,1,0,1], goal = 2
 * Output: 4
 * Explanation: The 4 subarrays are bolded and underlined below:
 * [<u>1,0,1</u>,0,1]
 * [<u>1,0,1,0</u>,1]
 * [1,<u>0,1,0,1</u>]
 * [1,0,<u>1,0,1</u>]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0,0,0,0,0], goal = 0
 * Output: 15
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 3 * 10^4
 * 	nums[i] is either 0 or 1.
 * 	0 <= goal <= nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-subarrays-with-sum/
// discuss: https://leetcode.com/problems/binary-subarrays-with-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut sum = 0;
        let mut result = 0;
        let mut counts = std::collections::HashMap::new();
        counts.insert(0, 1);

        for num in nums {
            sum += num;
            if let Some(&count) = counts.get(&(sum - goal)) {
                result += count;
            }
            *counts.entry(sum).or_insert(0) += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_930() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
    }
}

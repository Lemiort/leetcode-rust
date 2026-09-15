use core::num;

/**
 * [992] Subarrays with K Different Integers
 *
 * Given an integer array nums and an integer k, return the number of good subarrays of nums.
 * A good array is an array where the number of different integers in that array is exactly k.
 *
 * 	For example, [1,2,3,1,2] has 3 different integers: 1, 2, and 3.
 *
 * A subarray is a contiguous part of an array.
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [1,2,1,2,3], k = 2
 * Output: 7
 * Explanation: Subarrays formed with exactly 2 different integers: [1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2]
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [1,2,1,3,4], k = 3
 * Output: 3
 * Explanation: Subarrays formed with exactly 3 different integers: [1,2,1,3], [2,1,3], [1,3,4].
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	1 <= nums[i], k <= nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subarrays-with-k-different-integers/
// discuss: https://leetcode.com/problems/subarrays-with-k-different-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarrays_up_to_k(&nums, k) - Self::subarrays_up_to_k(&nums, k - 1)
    }

    pub fn subarrays_up_to_k(nums: &Vec<i32>, k: i32) -> i32 {
        let mut left: usize = 0;
        let mut memo = std::collections::HashMap::new();
        let mut counter = 0i32;
        for right in 0..nums.len() {
            *memo.entry(nums[right]).or_insert(0) += 1;
            while memo.len() > k as usize {
                memo.entry(nums[left]).and_modify(|e| *e -= 1);
                if *memo.get(&nums[left]).unwrap() == 0 {
                    memo.remove(&nums[left]);
                }
                left += 1;
            }
            counter += right as i32 - left as i32 + 1;
        }
        return counter;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_992() {
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2),
            7
        );

        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3),
            3
        );
    }
}

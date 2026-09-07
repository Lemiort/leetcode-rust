/**
 * [560] Subarray Sum Equals K
 *
 * Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * <strong class="example">Example 1:
 * Input: nums = [1,1,1], k = 2
 * Output: 2
 * <strong class="example">Example 2:
 * Input: nums = [1,2,3], k = 3
 * Output: 2
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	-1000 <= nums[i] <= 1000
 * 	-10^7 <= k <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subarray-sum-equals-k/
// discuss: https://leetcode.com/problems/subarray-sum-equals-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut prefix_counts = std::collections::HashMap::with_capacity(nums.len() + 1);
        prefix_counts.insert(0, 1);

        for num in nums {
            sum += num;
            // find prefix sum that equals to sum - k
            if let Some(&c) = prefix_counts.get(&(sum - k)) {
                count += c;
            }

            // record the current prefix sum
            *prefix_counts.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_560() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
        assert_eq!(Solution::subarray_sum(vec![1], 0), 0);
        assert_eq!(Solution::subarray_sum(vec![-1, -1, 1], 0), 1);
    }
}

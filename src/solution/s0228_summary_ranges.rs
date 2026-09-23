/**
 * [228] Summary Ranges
 *
 * You are given a sorted unique integer array nums.
 * A range [a,b] is the set of all integers from a to b (inclusive).
 * Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.
 * Each range [a,b] in the list should be output as:
 *
 * 	"a->b" if a != b
 * 	"a" if a == b
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: nums = [0,1,2,4,5,7]
 * Output: ["0->2","4->5","7"]
 * Explanation: The ranges are:
 * [0,2] --> "0->2"
 * [4,5] --> "4->5"
 * [7,7] --> "7"
 *
 * <strong class="example">Example 2:
 *
 * Input: nums = [0,2,3,4,6,8,9]
 * Output: ["0","2->4","6","8->9"]
 * Explanation: The ranges are:
 * [0,0] --> "0"
 * [2,4] --> "2->4"
 * [6,6] --> "6"
 * [8,9] --> "8->9"
 *
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 20
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	All the values of nums are unique.
 * 	nums is sorted in ascending order.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/summary-ranges/
// discuss: https://leetcode.com/problems/summary-ranges/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        let mut result = Vec::new();
        let mut left = nums[0];
        let mut prev = left;
        for num in nums {
            if (num as i64 - prev as i64) > 1 {
                if prev == left {
                    result.push(left.to_string());
                } else {
                    result.push(std::format!("{}->{}", left, prev));
                }
                left = num;
            }
            prev = num
        }

        // last range
        if prev == left {
            result.push(left.to_string());
        } else {
            result.push(std::format!("{}->{}", left, prev));
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_228() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );

        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );

        assert_eq!(
            Solution::summary_ranges(vec![-2147483648, 0, 2, 3, 4, 6, 8, 9]),
            vec!["-2147483648", "0", "2->4", "6", "8->9"]
        );
    }
}

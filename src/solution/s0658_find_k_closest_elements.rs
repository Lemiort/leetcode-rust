use std::i32;

/**
 * [658] Find K Closest Elements
 *
 * Given a sorted integer array arr, two integers k and x, return the k closest integers to x in the array. The result should also be sorted in ascending order.
 * An integer a is closer to x than an integer b if:
 *
 * 	|a - x| < |b - x|, or
 * 	|a - x| == |b - x| and a < b
 *
 *  
 * <strong class="example">Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">arr = [1,2,3,4,5], k = 4, x = 3</span>
 * Output: <span class="example-io">[1,2,3,4]</span>
 * </div>
 * <strong class="example">Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">arr = [1,1,2,3,4,5], k = 4, x = -1</span>
 * Output: <span class="example-io">[1,1,2,3]</span>
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= k <= arr.length
 * 	1 <= arr.length <= 10^4
 * 	arr is sorted in ascending order.
 * 	-10^4 <= arr[i], x <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-k-closest-elements/
// discuss: https://leetcode.com/problems/find-k-closest-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut left = 0;
        let mut sum_distance = 0;
        let mut best_left = 0;
        for right in 0..arr.len() {
            let right_distance = (x - arr[right]).abs();
            sum_distance += right_distance;
            // cannot expand window anymore
            if right - left >= k as usize {
                let left_distance = (x - arr[left]).abs();
                let old_sum_distance = sum_distance - right_distance;
                let new_sum_distance = sum_distance - left_distance;
                if old_sum_distance > new_sum_distance {
                    // regular shift of window
                    left += 1;
                    best_left = left;
                    sum_distance = new_sum_distance;
                } else if old_sum_distance == new_sum_distance {
                    // prefer old one, but shift anyway
                    left += 1;
                    // best_left = left;
                    sum_distance = new_sum_distance;
                } else {
                    // refuse to shift
                    return arr[best_left..best_left + k as usize].to_vec();
                }
            }
        }
        arr[best_left..best_left + k as usize].to_vec()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_658() {
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
            vec![1, 2, 3, 4]
        );

        assert_eq!(
            Solution::find_closest_elements(vec![1, 1, 2, 3, 4, 5], 4, -1),
            vec![1, 1, 2, 3]
        );

        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 1, 3),
            vec![3]
        );

        assert_eq!(
            Solution::find_closest_elements(vec![1, 1, 1, 10, 10, 10], 1, 9),
            vec![10]
        );

        assert_eq!(
            Solution::find_closest_elements(vec![0, 0, 1, 2, 3, 3, 4, 7, 7, 8], 3, 5),
            vec![3, 3, 4]
        );
    }
}

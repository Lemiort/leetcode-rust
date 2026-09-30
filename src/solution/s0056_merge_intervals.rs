/**
 * [56] Merge Intervals
 *
 * Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
 *  
 * <strong class="example">Example 1:
 *
 * Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
 *
 * <strong class="example">Example 2:
 *
 * Input: intervals = [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 *
 * <strong class="example">Example 3:
 *
 * Input: intervals = [[4,7],[1,4]]
 * Output: [[1,7]]
 * Explanation: Intervals [1,4] and [4,7] are considered overlapping.
 *
 *  
 * Constraints:
 *
 * 	1 <= intervals.length <= 10^4
 * 	intervals[i].length == 2
 * 	0 <= starti <= endi <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-intervals/
// discuss: https://leetcode.com/problems/merge-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
// [[1,3],[2,6],[8,10],[15,18]]
// 1 2 3 6 8 10 15 18
// 1 2 1 2 3 3   4  4

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        intervals.sort_unstable_by_key(|interval| interval[0]);
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        for interval in intervals {
            if res.is_empty() || res.last().unwrap()[1] < interval[0] {
                res.push(interval);
            } else {
                // update last inserted value if it has overlaps
                let last = res.last_mut().unwrap();
                last[1] = last[1].max(interval[1]);
            }
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_56() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );

        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );

        assert_eq!(
            Solution::merge(vec![vec![4, 7], vec![1, 4]]),
            vec![vec![1, 7]]
        );
    }
}

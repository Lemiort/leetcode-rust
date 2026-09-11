/**
 * [849] Maximize Distance to Closest Person
 *
 * You are given an array representing a row of seats where seats[i] = 1 represents a person sitting in the i^th seat, and seats[i] = 0 represents that the i^th seat is empty (0-indexed).
 * There is at least one empty seat, and at least one person sitting.
 * Alex wants to sit in the seat such that the distance between him and the closest person to him is maximized.
 * Return that maximum distance to the closest person.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/10/distance.jpg" style="width: 650px; height: 257px;" />
 * Input: seats = [1,0,0,0,1,0,1]
 * Output: 2
 * Explanation:
 * If Alex sits in the second open seat (i.e. seats[2]), then the closest person has distance 2.
 * If Alex sits in any other open seat, the closest person has distance 1.
 * Thus, the maximum distance to the closest person is 2.
 *
 * <strong class="example">Example 2:
 *
 * Input: seats = [1,0,0,0]
 * Output: 3
 * Explanation:
 * If Alex sits in the last seat (i.e. seats[3]), the closest person is 3 seats away.
 * This is the maximum distance possible, so the answer is 3.
 *
 * <strong class="example">Example 3:
 *
 * Input: seats = [0,1]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	2 <= seats.length <= 2 * 10^4
 * 	seats[i] is 0 or 1.
 * 	At least one seat is empty.
 * 	At least one seat is occupied.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-distance-to-closest-person/
// discuss: https://leetcode.com/problems/maximize-distance-to-closest-person/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut corner_max = 0;
        let mut ltr_max = 0;
        let mut distance = 0;
        // left to right pass
        for seat in &seats {
            if *seat == 1 {
                ltr_max = ltr_max.max(distance);
                distance = 0;
            } else {
                distance += 1;
            }
        }
        corner_max = distance;
        distance = 0;
        let mut rtl_max = 0;
        // right to left distance
        for seat in seats.iter().rev() {
            if *seat == 1 {
                rtl_max = rtl_max.max(distance);
                distance = 0;
            } else {
                distance += 1;
            }
        }
        corner_max = distance.max(corner_max);
        let mid_max = (rtl_max.max(rtl_max) + 1) / 2;
        return mid_max.max(corner_max);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_849() {
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0]), 3);
        assert_eq!(Solution::max_dist_to_closest(vec![0, 1]), 1);
    }
}

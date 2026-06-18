/**
 * [1344] Angle Between Hands of a Clock
 *
 * Given two numbers, hour and minutes, return the smaller angle (in degrees) formed between the hour and the minute hand.
 * Answers within 10^-5 of the actual value will be accepted as correct.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_1_1673.png" style="width: 300px; height: 296px;" />
 * Input: hour = 12, minutes = 30
 * Output: 165
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_2_1673.png" style="width: 300px; height: 301px;" />
 * Input: hour = 3, minutes = 30
 * Output: 75
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/26/sample_3_1673.png" style="width: 300px; height: 301px;" />
 * Input: hour = 3, minutes = 15
 * Output: 7.5
 *
 *  
 * Constraints:
 *
 * 	1 <= hour <= 12
 * 	0 <= minutes <= 59
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/angle-between-hands-of-a-clock/
// discuss: https://leetcode.com/problems/angle-between-hands-of-a-clock/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let hour_angle =
            (hour % 12) as f64 * 30.0f64 + (minutes as f64 * 360.0f64 / 60.0f64) / 12.0f64;
        let minute_angle = minutes as f64 * 360.0f64 / 60.0f64;
        let angle = hour_angle - minute_angle;
        return angle.abs().min(360.0f64 - angle.abs());
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1344() {
        assert_eq!(Solution::angle_clock(12, 30), 165.0);
        assert_eq!(Solution::angle_clock(3, 30), 75.0);
        assert_eq!(Solution::angle_clock(3, 15), 7.5);
    }
}

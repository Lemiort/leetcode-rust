/**
 * [149] Max Points on a Line
 *
 * Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane, return the maximum number of points that lie on the same straight line.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/plane1.jpg" style="width: 300px; height: 294px;" />
 * Input: points = [[1,1],[2,2],[3,3]]
 * Output: 3
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/plane2.jpg" style="width: 300px; height: 294px;" />
 * Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= points.length <= 300
 * 	points[i].length == 2
 * 	-10^4 <= xi, yi <= 10^4
 * 	All the points are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-points-on-a-line/
// discuss: https://leetcode.com/problems/max-points-on-a-line/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut global_max = 1;
        for i in 0..points.len() {
            let mut angles = std::collections::HashMap::new();
            let mut current_max = 1;
            for j in (i + 1)..points.len() {
                let mut dx = points[i][0] - points[j][0];
                let mut dy = points[i][1] - points[j][1];
                let gcd = Self::gcd(dx, dy);
                dx /= gcd;
                dy /= gcd;
                if dx < 0 || (dx == 0 && dy < 0) {
                    dx = -dx;
                    dy = -dy;
                }
                let current_count = angles.entry((dx, dy)).or_insert(0);
                *current_count += 1;
                current_max = current_max.max(*current_count + 1);
            }
            global_max = global_max.max(current_max);
        }
        global_max
    }

    pub fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a.abs() } else { Self::gcd(b, a % b) }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_149() {
        assert_eq!(
            Solution::max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ]),
            4
        );
    }
}

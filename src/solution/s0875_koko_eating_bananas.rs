/**
 * [875] Koko Eating Bananas
 *
 * Koko loves to eat bananas. There are n piles of bananas, the i^th pile has piles[i] bananas. The guards have gone and will come back in h hours.
 * Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.
 * Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
 * Return the minimum integer k such that she can eat all the bananas within h hours.
 *  
 * <strong class="example">Example 1:
 *
 * Input: piles = [3,6,7,11], h = 8
 * Output: 4
 *
 * <strong class="example">Example 2:
 *
 * Input: piles = [30,11,23,4,20], h = 5
 * Output: 30
 *
 * <strong class="example">Example 3:
 *
 * Input: piles = [30,11,23,4,20], h = 6
 * Output: 23
 *
 *  
 * Constraints:
 *
 * 	1 <= piles.length <= 10^4
 * 	piles.length <= h <= 10^9
 * 	1 <= piles[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/koko-eating-bananas/
// discuss: https://leetcode.com/problems/koko-eating-bananas/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        assert!((piles.len() as i32) <= h);

        let mut max_speed = *piles.iter().max().unwrap_or(&0);
        let mut min_speed = (max_speed + h - 1) / h; // div_ceil
        let mut ans = max_speed;

        while min_speed <= max_speed {
            let speed = min_speed + ((max_speed - min_speed) / 2);
            let hours = Solution::eat(&piles, speed);
            // some time left - speed up
            if hours > h as i64 {
                min_speed = speed + 1;
            } else {
                max_speed = speed - 1;
                ans = speed;
            }
        }
        ans
    }

    //try to eat with speed, return required time
    fn eat(piles: &Vec<i32>, speed: i32) -> i64 {
        let mut hours = 0i64;
        for pile in piles {
            hours += (*pile as i64 + speed as i64 - 1) / speed as i64; // div_ceil
        }
        return hours;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_875() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
        assert_eq!(
            Solution::min_eating_speed(
                vec![
                    873375536, 395271806, 617254718, 970525912, 634754347, 824202576, 694181619,
                    20191396, 886462834, 442389139, 572655464, 438946009, 791566709, 776244944,
                    694340852, 419438893, 784015530, 588954527, 282060288, 269101141, 499386849,
                    846936808, 92389214, 385055341, 56742915, 803341674, 837907634, 728867715,
                    20958651, 167651719, 345626668, 701905050, 932332403, 572486583, 603363649,
                    967330688, 484233747, 859566856, 446838995, 375409782, 220949961, 72860128,
                    998899684, 615754807, 383344277, 36322529, 154308670, 335291837, 927055440,
                    28020467, 558059248, 999492426, 991026255, 30205761, 884639109, 61689648,
                    742973721, 395173120, 38459914, 705636911, 30019578, 968014413, 126489328,
                    738983100, 793184186, 871576545, 768870427, 955396670, 328003949, 786890382,
                    450361695, 994581348, 158169007, 309034664, 388541713, 142633427, 390169457,
                    161995664, 906356894, 379954831, 448138536
                ],
                943223529
            ),
            46
        );
    }
}

/**
 * [3286] Find a Safe Walk Through a Grid
 *
 * You are given an m x n binary matrix grid and an integer health.
 * You start on the upper-left corner (0, 0) and would like to get to the lower-right corner (m - 1, n - 1).
 * You can move up, down, left, or right from one cell to another adjacent cell as long as your health remains positive.
 * Cells (i, j) with grid[i][j] = 1 are considered unsafe and reduce your health by 1.
 * Return true if you can reach the final cell with a health value of 1 or more, and false otherwise.
 *  
 * <strong class="example">Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">grid = [[0,1,0,0,0],[0,1,0,1,0],[0,0,0,1,0]], health = 1</span>
 * Output: <span class="example-io">true</span>
 * Explanation:
 * The final cell can be reached safely by walking along the gray cells below.
 * <img alt="" src="https://assets.leetcode.com/uploads/2024/08/04/3868_examples_1drawio.png" style="width: 301px; height: 121px;" /></div>
 * <strong class="example">Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">grid = [[0,1,1,0,0,0],[1,0,1,0,0,0],[0,1,1,1,0,1],[0,0,1,0,1,0]], health = 3</span>
 * Output: <span class="example-io">false</span>
 * Explanation:
 * A minimum of 4 health points is needed to reach the final cell safely.
 * <img alt="" src="https://assets.leetcode.com/uploads/2024/08/04/3868_examples_2drawio.png" style="width: 361px; height: 161px;" /></div>
 * <strong class="example">Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">grid = [[1,1,1],[1,0,1],[1,1,1]], health = 5</span>
 * Output: <span class="example-io">true</span>
 * Explanation:
 * The final cell can be reached safely by walking along the gray cells below.
 * <img alt="" src="https://assets.leetcode.com/uploads/2024/08/04/3868_examples_3drawio.png" style="width: 181px; height: 121px;" />
 * Any path that does not go through the cell (1, 1) is unsafe since your health will drop to 0 when reaching the final cell.
 * </div>
 *  
 * Constraints:
 *
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 50
 * 	<font face="monospace">2 <= m * n</font>
 * 	1 <= health <= m + n
 * 	grid[i][j] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-a-safe-walk-through-a-grid/
// discuss: https://leetcode.com/problems/find-a-safe-walk-through-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let height = grid.len();
        let width = grid[0].len();

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let start = (0, 0);
        let end = (height - 1, width - 1);

        const INF: i32 = i32::MAX;
        let mut dist = vec![vec![INF; width]; height];

        dist[start.0][start.1] = grid[start.0][start.1];

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in &directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < height as i32 && ny >= 0 && ny < width as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    let weight = grid[nx][ny];
                    let new_dist = dist[x][y] + weight;

                    if new_dist < dist[nx][ny] {
                        dist[nx][ny] = new_dist;

                        if weight == 0 {
                            queue.push_front((nx, ny));
                        } else {
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }

        dist[end.0][end.1] < health
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3286() {
        assert_eq!(
            Solution::find_safe_walk(
                vec![
                    vec![0, 1, 0, 0, 0],
                    vec![0, 1, 0, 1, 0],
                    vec![0, 0, 0, 1, 0]
                ],
                1
            ),
            true
        );
        assert_eq!(
            Solution::find_safe_walk(
                vec![
                    vec![0, 1, 1, 0, 0, 0],
                    vec![1, 0, 1, 0, 0, 0],
                    vec![0, 1, 1, 1, 0, 1],
                    vec![0, 0, 1, 0, 1, 0]
                ],
                3
            ),
            false
        );
        assert_eq!(
            Solution::find_safe_walk(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]], 5),
            true
        );
    }
}

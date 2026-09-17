/**
 * [32] Longest Valid Parentheses
 *
 * Given a string containing just the characters '(' and ')', return the length of the longest valid (well-formed) parentheses <span data-keyword="substring-nonempty">substring</span>.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "(()"
 * Output: 2
 * Explanation: The longest valid parentheses substring is "()".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = ")()())"
 * Output: 4
 * Explanation: The longest valid parentheses substring is "()()".
 *
 * <strong class="example">Example 3:
 *
 * Input: s = ""
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 3 * 10^4
 * 	s[i] is '(', or ')'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-valid-parentheses/
// discuss: https://leetcode.com/problems/longest-valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
// (()
// 0 1 0
// 1 2 0
// 2 2 1
//
// 0 0 1
// 1 1 1 -> +2
// 2 2 1

// )()())
// 0 0 1/0
// 1 1 0
// 2 1 1 -> +2
// 3 2 1
// 4 2 2 -> +2
// 5 2 2
//
// 0 0 1
// 1 0 2
// 2 1 2
// 3 1 3
//

// ()(()
// 0 1 0
// 1 1 1 +2
// 2 2 1
// 3 3 1
// 4 3 2
//
// 0 0 1
// 0 1 1 +2
// 0 2 1 - > 0 0
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut open = 0;
        let mut close = 0;
        let mut counter = 0;
        let mut max = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => {
                    open += 1;
                }
                ')' => {
                    close += 1;
                }
                _ => {
                    // cant be there
                }
            }
            if open == close {
                counter += 2;
                max = std::cmp::max(max, counter);
            }
            if close > open {
                open = 0;
                close = 0;
                counter = 0;
            }
        }
        open = 0;
        close = 0;
        counter = 0;

        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '(' => {
                    open += 1;
                }
                ')' => {
                    close += 1;
                }
                _ => {
                    // cant be there
                }
            }
            if open == close {
                counter += 2;
                max = std::cmp::max(max, counter);
            }
            if open > close {
                open = 0;
                close = 0;
                counter = 0;
            }
        }
        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_32() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_owned()), 2);

        assert_eq!(Solution::longest_valid_parentheses(")()())".to_owned()), 4);

        assert_eq!(Solution::longest_valid_parentheses("".to_owned()), 0);

        assert_eq!(Solution::longest_valid_parentheses("()(()".to_owned()), 2);

        assert_eq!(
            Solution::longest_valid_parentheses("))))())()()(()".to_owned()),
            4
        );
    }
}

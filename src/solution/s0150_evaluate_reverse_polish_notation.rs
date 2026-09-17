/**
 * [150] Evaluate Reverse Polish Notation
 *
 * You are given an array of strings tokens that represents an arithmetic expression in a <a href="http://en.wikipedia.org/wiki/Reverse_Polish_notation" target="_blank">Reverse Polish Notation</a>.
 * Evaluate the expression. Return an integer that represents the value of the expression.
 * Note that:
 *
 * 	The valid operators are '+', '-', '*', and '/'.
 * 	Each operand may be an integer or another expression.
 * 	The division between two integers always truncates toward zero.
 * 	There will not be any division by zero.
 * 	The input represents a valid arithmetic expression in a reverse polish notation.
 * 	The answer and all the intermediate calculations can be represented in a 32-bit integer.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input: tokens = ["2","1","+","3","*"]
 * Output: 9
 * Explanation: ((2 + 1) * 3) = 9
 *
 * <strong class="example">Example 2:
 *
 * Input: tokens = ["4","13","5","/","+"]
 * Output: 6
 * Explanation: (4 + (13 / 5)) = 6
 *
 * <strong class="example">Example 3:
 *
 * Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
 * Output: 22
 * Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
 * = ((10 * (6 / (12 * -11))) + 17) + 5
 * = ((10 * (6 / -132)) + 17) + 5
 * = ((10 * 0) + 17) + 5
 * = (0 + 17) + 5
 * = 17 + 5
 * = 22
 *
 *  
 * Constraints:
 *
 * 	1 <= tokens.length <= 10^4
 * 	tokens[i] is either an operator: "+", "-", "*", or "/", or an integer in the range [-200, 200].
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/evaluate-reverse-polish-notation/
// discuss: https://leetcode.com/problems/evaluate-reverse-polish-notation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let b: i32 = stack.pop().unwrap();
                    let a: i32 = stack.pop().unwrap();
                    stack.push(b + a);
                }
                "-" => {
                    let b: i32 = stack.pop().unwrap();
                    let a: i32 = stack.pop().unwrap();
                    stack.push(a - b);
                }
                "*" => {
                    let b: i32 = stack.pop().unwrap();
                    let a: i32 = stack.pop().unwrap();
                    stack.push(b * a);
                }
                "/" => {
                    let b: i32 = stack.pop().unwrap();
                    let a: i32 = stack.pop().unwrap();
                    stack.push(a / b);
                }
                _ => {
                    stack.push(token.parse().unwrap());
                }
            }
        }
        return stack.pop().unwrap();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_150() {
        assert_eq!(
            Solution::eval_rpn(
                vec!["2", "1", "+", "3", "*"]
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect()
            ),
            9
        );

        assert_eq!(
            Solution::eval_rpn(
                vec!["4", "13", "5", "/", "+"]
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect()
            ),
            6
        );

        assert_eq!(
            Solution::eval_rpn(
                vec![
                    "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
                ]
                .into_iter()
                .map(|v| v.to_string())
                .collect()
            ),
            22
        )
    }
}

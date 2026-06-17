/**
 * [3614] Process String with Special Operations II
 *
 * You are given a string s consisting of lowercase English letters and the special characters: '*', '#', and '%'.
 * You are also given an integer k.
 * Build a new string result by processing s according to the following rules from left to right:
 *
 * 	If the letter is a lowercase English letter append it to result.
 * 	A '*' removes the last character from result, if it exists.
 * 	A '#' duplicates the current result and appends it to itself.
 * 	A '%' reverses the current result.
 *
 * Return the k^th character of the final string result. If k is out of the bounds of result, return '.'.
 *  
 * <strong class="example">Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">s = "a#b%*", k = 1</span>
 * Output: <span class="example-io">"a"</span>
 * Explanation:
 * <table style="border: 1px solid black;">
 * 	<thead>
 * 		<tr>
 * 			<th style="border: 1px solid black;">i</th>
 * 			<th style="border: 1px solid black;">s[i]</th>
 * 			<th style="border: 1px solid black;">Operation</th>
 * 			<th style="border: 1px solid black;">Current result</th>
 * 		</tr>
 * 	</thead>
 * 	<tbody>
 * 		<tr>
 * 			<td style="border: 1px solid black;">0</td>
 * 			<td style="border: 1px solid black;">'a'</td>
 * 			<td style="border: 1px solid black;">Append 'a'</td>
 * 			<td style="border: 1px solid black;">"a"</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">1</td>
 * 			<td style="border: 1px solid black;">'#'</td>
 * 			<td style="border: 1px solid black;">Duplicate result</td>
 * 			<td style="border: 1px solid black;">"aa"</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">2</td>
 * 			<td style="border: 1px solid black;">'b'</td>
 * 			<td style="border: 1px solid black;">Append 'b'</td>
 * 			<td style="border: 1px solid black;">"aab"</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">3</td>
 * 			<td style="border: 1px solid black;">'%'</td>
 * 			<td style="border: 1px solid black;">Reverse result</td>
 * 			<td style="border: 1px solid black;">"baa"</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">4</td>
 * 			<td style="border: 1px solid black;">'*'</td>
 * 			<td style="border: 1px solid black;">Remove the last character</td>
 * 			<td style="border: 1px solid black;">"ba"</td>
 * 		</tr>
 * 	</tbody>
 * </table>
 * The final result is "ba". The character at index k = 1 is 'a'.
 * </div>
 * <strong class="example">Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">s = "cd%#*#", k = 3</span>
 * Output: <span class="example-io">"d"</span>
 * Explanation:
 * <table style="border: 1px solid black;">
 * 	<thead>
 * 		<tr>
 * 			<th style="border: 1px solid black;">i</th>
 * 			<th style="border: 1px solid black;">s[i]</th>
 * 			<th style="border: 1px solid black;">Operation</th>
 * 			<th style="border: 1px solid black;">Current result</th>
 * 		</tr>
 * 	</thead>
 * 	<tbody>
 * 		<tr>
 * 			<td style="border: 1px solid black;">0</td>
 * 			<td style="border: 1px solid black;">'c'</td>
 * 			<td style="border: 1px solid black;">Append 'c'</td>
 * 			<td style="border: 1px solid black;">"c"</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">1</td>
 * 			<td style="border: 1px solid black;">'d'</td>
 * 			<td style="border: 1px solid black;">Append 'd'</td>
 * 			<td style="border: 1px solid black;">"cd"</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">2</td>
 * 			<td style="border: 1px solid black;">'%'</td>
 * 			<td style="border: 1px solid black;">Reverse result</td>
 * 			<td style="border: 1px solid black;">"dc"</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">3</td>
 * 			<td style="border: 1px solid black;">'#'</td>
 * 			<td style="border: 1px solid black;">Duplicate result</td>
 * 			<td style="border: 1px solid black;">"dcdc"</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">4</td>
 * 			<td style="border: 1px solid black;">'*'</td>
 * 			<td style="border: 1px solid black;">Remove the last character</td>
 * 			<td style="border: 1px solid black;">"dcd"</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">5</td>
 * 			<td style="border: 1px solid black;">'#'</td>
 * 			<td style="border: 1px solid black;">Duplicate result</td>
 * 			<td style="border: 1px solid black;">"dcddcd"</td>
 * 		</tr>
 * 	</tbody>
 * </table>
 * The final result is "dcddcd". The character at index k = 3 is 'd'.
 * </div>
 * <strong class="example">Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">s = "z*#", k = 0</span>
 * Output: <span class="example-io">"."</span>
 * Explanation:
 * <table style="border: 1px solid black;">
 * 	<thead>
 * 		<tr>
 * 			<th style="border: 1px solid black;">i</th>
 * 			<th style="border: 1px solid black;">s[i]</th>
 * 			<th style="border: 1px solid black;">Operation</th>
 * 			<th style="border: 1px solid black;">Current result</th>
 * 		</tr>
 * 	</thead>
 * 	<tbody>
 * 		<tr>
 * 			<td style="border: 1px solid black;">0</td>
 * 			<td style="border: 1px solid black;">'z'</td>
 * 			<td style="border: 1px solid black;">Append 'z'</td>
 * 			<td style="border: 1px solid black;">"z"</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">1</td>
 * 			<td style="border: 1px solid black;">'*'</td>
 * 			<td style="border: 1px solid black;">Remove the last character</td>
 * 			<td style="border: 1px solid black;">""</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 1px solid black;">2</td>
 * 			<td style="border: 1px solid black;">'#'</td>
 * 			<td style="border: 1px solid black;">Duplicate the string</td>
 * 			<td style="border: 1px solid black;">""</td>
 * 		</tr>
 * 	</tbody>
 * </table>
 * The final result is "". Since index k = 0 is out of bounds, the output is '.'.
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of only lowercase English letters and special characters '*', '#', and '%'.
 * 	0 <= k <= 10^15
 * 	The length of result after processing s will not exceed 10^15.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/process-string-with-special-operations-ii/
// discuss: https://leetcode.com/problems/process-string-with-special-operations-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn process_str(s: String, k: i64) -> char {
        let chars: Vec<char> = s.chars().collect();

        // Forward pass: compute final length after all operations
        let mut len = 0i64;
        for c in &chars {
            match c {
                '*' => {
                    if len > 0 {
                        len -= 1;
                    }
                }
                '#' => {
                    len *= 2;
                }
                '%' => {
                    // Reversal doesn't change length
                }
                _ => {
                    len += 1;
                }
            }
        }

        // If k is out of bounds, return '.'
        if k >= len {
            return '.';
        }

        // Backward pass: trace backwards to find which character ends up at position k
        let mut pos = k;
        let mut cur_len = len;

        for i in (0..chars.len()).rev() {
            match chars[i] {
                '*' => {
                    // Before this removal, length was cur_len + 1
                    // Character at position cur_len (last position) was removed
                    if pos == cur_len {
                        return '.'; // This is the removed character
                    }
                    cur_len += 1;
                }
                '#' => {
                    // Before duplication, length was cur_len / 2
                    // Use modulus to map position back to original half
                    cur_len /= 2;
                    pos %= cur_len;
                }
                '%' => {
                    // Before reversal, position was mirrored
                    pos = cur_len - 1 - pos;
                }
                _ => {
                    // Regular character was added at position cur_len - 1
                    if pos == cur_len - 1 {
                        return chars[i]; // Found the character!
                    }
                    cur_len -= 1;
                }
            }
        }

        '.'
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3614() {
        assert_eq!(Solution::process_str("a#b%*".into(), 1), 'a');
        assert_eq!(Solution::process_str("cd%#*#".into(), 3), 'd');
        assert_eq!(Solution::process_str("z*#".into(), 0), '.');
    }
}

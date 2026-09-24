/**
 * [443] String Compression
 *
 * Given an array of characters chars, compress it using the following algorithm:
 * Begin with an empty string s. For each group of consecutive repeating characters in chars:
 *
 * 	If the group's length is 1, append the character to s.
 * 	Otherwise, append the character followed by the group's length.
 *
 * The compressed string s should not be returned separately, but instead, be stored in the input character array chars. Note that group lengths that are 10 or longer will be split into multiple characters in chars.
 * After you are done modifying the input array, return the new length of the array.
 * You must write an algorithm that uses only constant extra space.
 * Note: The characters in the array beyond the returned length do not matter and should be ignored.
 *  
 * <strong class="example">Example 1:
 *
 * Input: chars = ["a","a","b","b","c","c","c"]
 * Output: 6
 * Explanation: The groups are "aa", "bb", and "ccc". This compresses to "a2b2c3".
 * After modifying the input array in-place, the first 6 characters of chars should be ["a","2","b","2","c","3"].
 *
 * <strong class="example">Example 2:
 *
 * Input: chars = ["a"]
 * Output: 1
 * Explanation: The only group is "a", which remains uncompressed since it is a single character.
 * After modifying the input array in-place, the first character of chars should be ["a"].
 *
 * <strong class="example">Example 3:
 *
 * Input: chars = ["a","b","b","b","b","b","b","b","b","b","b","b","b"]
 * Output: 4
 * Explanation: The groups are "a" and "bbbbbbbbbbbb". This compresses to "ab12".
 * After modifying the input array in-place, the first 4 characters of chars should be ["a","b","1","2"].
 *
 *  
 * Constraints:
 *
 * 	1 <= chars.length <= 2000
 * 	chars[i] is a lowercase English letter, uppercase English letter, digit, or symbol.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-compression/
// discuss: https://leetcode.com/problems/string-compression/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() == 0 {
            return 0;
        } else if chars.len() == 1 {
            return 1;
        }
        let mut current_char = chars[0];
        let mut current_counter = 0;
        let mut current_index = 0; // where do we overide now
        for i in 0..chars.len() {
            if chars[i] == current_char {
                current_counter += 1;
            } else {
                // new group

                // start to write after current symbol
                current_index += 1;
                if current_counter > 1 {
                    let number = current_counter.to_string();
                    // write number
                    for (j, c) in number.chars().enumerate() {
                        chars[current_index + j] = c;
                    }
                    current_index += number.len();
                }
                current_char = chars[i];
                chars[current_index] = current_char;
                current_counter = 1;
            }
        }
        // last one
        current_index += 1;
        if current_counter > 1 {
            let number = current_counter.to_string();
            // write number
            for (j, c) in number.chars().enumerate() {
                chars[current_index + j] = c;
            }
            current_index += number.len();
        }
        // if current_index < chars.len() {
        //     chars[current_index] = current_char;
        // }

        current_index as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_443() {
        let mut example1_source = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let example1_result = vec!['a', '2', 'b', '2', 'c', '2'];
        assert_eq!(Solution::compress(&mut example1_source), 6);
        assert!(example1_result.starts_with(&example1_result));

        let mut example2_source = vec!['a'];
        let example2_result = vec!['a'];
        assert_eq!(Solution::compress(&mut example2_source), 1);
        assert!(example2_result.starts_with(&example2_result));

        let mut example3_source = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let example3_result = vec!['a', 'b', '1', '2'];
        assert_eq!(Solution::compress(&mut example3_source), 4);
        assert!(example3_result.starts_with(&example3_result));

        let mut example4_source = vec!['a', 'a'];
        let example4_result = vec!['a', '2'];
        assert_eq!(Solution::compress(&mut example4_source), 2);
        assert!(example4_result.starts_with(&example4_result));

        let mut example5_source = vec!['a', 'b', 'c'];
        let example5_result = vec!['a', 'b', 'c'];
        assert_eq!(Solution::compress(&mut example5_source), 3);
        assert!(example5_result.starts_with(&example5_result));
    }
}

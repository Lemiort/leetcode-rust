/**
 * [680] Valid Palindrome II
 *
 * Given a string s, return true if the s can be palindrome after deleting at most one character from it.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "aba"
 * Output: true
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "abca"
 * Output: true
 * Explanation: You could delete the character 'c'.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "abc"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-palindrome-ii/
// discuss: https://leetcode.com/problems/valid-palindrome-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
// aba
// 0 2
// 1 1
// right - left = 0

// abca
// 0 3
// 1 2
// right - left = 1

// bddb
// 0 3
// 1 2
//

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let chars = s.as_bytes();
        let mut left = 0;
        let mut right = s.len() - 1;
        let mut errors = 0;
        while right > left {
            if chars[left] != chars[right] {
                return Self::is_palydrome(chars, left + 1, right)
                    || Self::is_palydrome(chars, left, right - 1);
            }
            left += 1;
            right -= 1;
        }
        true
    }

    fn is_palydrome(chars: &[u8], mut left: usize, mut right: usize) -> bool {
        while right > left {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_680() {
        assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abc".to_string()), false);
        assert_eq!(Solution::valid_palindrome("bddb".to_string()), true);

        assert_eq!(Solution::valid_palindrome("deeee".to_string()), true);
    }
}

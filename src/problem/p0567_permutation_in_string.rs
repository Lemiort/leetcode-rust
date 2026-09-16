/**
 * [567] Permutation in String
 *
 * Given two strings s1 and s2, return true if s2 contains a <span data-keyword="permutation-string">permutation</span> of s1, or false otherwise.
 * In other words, return true if one of s1's permutations is the substring of s2.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s1 = "ab", s2 = "eidbaooo"
 * Output: true
 * Explanation: s2 contains one permutation of s1 ("ba").
 *
 * <strong class="example">Example 2:
 *
 * Input: s1 = "ab", s2 = "eidboaoo"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= s1.length, s2.length <= 10^4
 * 	s1 and s2 consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutation-in-string/
// discuss: https://leetcode.com/problems/permutation-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1b = s1.as_bytes();
        let mut goal = [0usize; 26];
        let mut window = [0usize; 26];
        for c in s1b {
            goal[(c - b'a') as usize] += 1;
        }
        let source = s2.as_bytes();
        let mut left = 0;
        for right in 0..source.len() {
            let c = source[right];
            window[(c - b'a') as usize] += 1;

            if right - left + 1 == s1b.len() {
                if window == goal {
                    return true;
                }
                let left_char = source[left];
                window[(left_char - b'a') as usize] -= 1;
                left += 1;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_567() {
        assert_eq!(
            Solution::check_inclusion("ab".to_owned(), "eidbaooo".to_owned()),
            true
        );
        assert_eq!(
            Solution::check_inclusion("ab".to_owned(), "eidboaoo".to_owned()),
            false
        );
    }
}

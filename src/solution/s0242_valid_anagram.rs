/**
 * [242] Valid Anagram
 *
 * Given two strings s and t, return true if t is an <span data-keyword="anagram">anagram</span> of s, and false otherwise.
 *  
 * <strong class="example">Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">s = "anagram", t = "nagaram"</span>
 * Output: <span class="example-io">true</span>
 * </div>
 * <strong class="example">Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">s = "rat", t = "car"</span>
 * Output: <span class="example-io">false</span>
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= s.length, t.length <= 5 * 10^4
 * 	s and t consist of lowercase English letters.
 *
 *  
 * Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-anagram/
// discuss: https://leetcode.com/problems/valid-anagram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let n = s.len();
        let sb = s.as_bytes();
        let tb = t.as_bytes();
        let mut s_map = vec![0usize; 26];
        let mut t_map = vec![0usize; 26];
        for i in 0..n {
            let s_char = sb[i];
            let t_char = tb[i];
            s_map[(s_char - b'a') as usize] += 1;
            t_map[(t_char - b'a') as usize] += 1;
        }
        return s_map == t_map;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242() {
        assert_eq!(
            Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()),
            true
        );

        assert_eq!(
            Solution::is_anagram("rat".to_owned(), "car".to_owned()),
            false
        );
    }
}

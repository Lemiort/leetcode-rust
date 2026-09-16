/**
 * [438] Find All Anagrams in a String
 *
 * Given two strings s and p, return an array of all the start indices of p's <span data-keyword="anagram">anagrams</span> in s. You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "cbaebabacd", p = "abc"
 * Output: [0,6]
 * Explanation:
 * The substring with start index = 0 is "cba", which is an anagram of "abc".
 * The substring with start index = 6 is "bac", which is an anagram of "abc".
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "abab", p = "ab"
 * Output: [0,1,2]
 * Explanation:
 * The substring with start index = 0 is "ab", which is an anagram of "ab".
 * The substring with start index = 1 is "ba", which is an anagram of "ab".
 * The substring with start index = 2 is "ab", which is an anagram of "ab".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length, p.length <= 3 * 10^4
 * 	s and p consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-all-anagrams-in-a-string/
// discuss: https://leetcode.com/problems/find-all-anagrams-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = Vec::new();
        let mut target = [0usize; 26];
        let pb = p.as_bytes();
        for c in pb {
            target[(c - b'a') as usize] += 1;
        }
        let mut current = [0usize; 26];
        let mut left = 0;
        let sb = s.as_bytes();
        for right in 0..sb.len() {
            let c = sb[right];
            current[(c - b'a') as usize] += 1;
            if current == target {
                result.push(left as i32);
            }
            if right - left + 1 == p.len() {
                let left_char = sb[left];
                current[(left_char - b'a') as usize] -= 1;
                left += 1;
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_438() {
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_owned(), "abc".to_owned()),
            vec![0, 6]
        );
        assert_eq!(
            Solution::find_anagrams("abab".to_owned(), "ab".to_owned()),
            vec![0, 1, 2]
        );
    }
}

/**
 * [71] Simplify Path
 *
 * You are given an absolute path for a Unix-style file system, which always begins with a slash '/'. Your task is to transform this absolute path into its simplified canonical path.
 * The rules of a Unix-style file system are as follows:
 *
 * 	A single period '.' represents the current directory.
 * 	A double period '..' represents the previous/parent directory.
 * 	Multiple consecutive slashes such as '//' and '///' are treated as a single slash '/'.
 * 	Any sequence of periods that does not match the rules above should be treated as a valid directory or file name. For example, '...' and '....' are valid directory or file names.
 *
 * The simplified canonical path should follow these rules:
 *
 * 	The path must start with a single slash '/'.
 * 	Directories within the path must be separated by exactly one slash '/'.
 * 	The path must not end with a slash '/', unless it is the root directory.
 * 	The path must not have any single or double periods ('.' and '..') used to denote current or parent directories.
 *
 * Return the simplified canonical path.
 *  
 * <strong class="example">Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">path = "/home/"</span>
 * Output: <span class="example-io">"/home"</span>
 * Explanation:
 * The trailing slash should be removed.
 * </div>
 * <strong class="example">Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">path = "/home//foo/"</span>
 * Output: <span class="example-io">"/home/foo"</span>
 * Explanation:
 * Multiple consecutive slashes are replaced by a single one.
 * </div>
 * <strong class="example">Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">path = "/home/user/Documents/../Pictures"</span>
 * Output: <span class="example-io">"/home/user/Pictures"</span>
 * Explanation:
 * A double period ".." refers to the directory up a level (the parent directory).
 * </div>
 * <strong class="example">Example 4:
 * <div class="example-block">
 * Input: <span class="example-io">path = "/../"</span>
 * Output: <span class="example-io">"/"</span>
 * Explanation:
 * Going one level up from the root directory is not possible.
 * </div>
 * <strong class="example">Example 5:
 * <div class="example-block">
 * Input: <span class="example-io">path = "/.../a/../b/c/../d/./"</span>
 * Output: <span class="example-io">"/.../b/d"</span>
 * Explanation:
 * "..." is a valid name for a directory in this problem.
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= path.length <= 3000
 * 	path consists of English letters, digits, period '.', slash '/' or '_'.
 * 	path is a valid absolute Unix path.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/simplify-path/
// discuss: https://leetcode.com/problems/simplify-path/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let path_split = path.split("/");
        let mut path_stack = Vec::new();
        for word in path_split {
            match word {
                ".." => {
                    path_stack.pop();
                }
                "." => {
                    //skip
                }
                "" => {
                    // handling of //, ///
                }
                _ => path_stack.push(word),
            }
        }
        let mut result = path_stack.join("/");
        result.insert(0, '/');
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_71() {
        assert_eq!(Solution::simplify_path("/home/".to_owned()), "/home");
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_owned()),
            "/home/foo"
        );
        assert_eq!(
            Solution::simplify_path("/home/user/Documents/../Pictures".to_owned()),
            "/home/user/Pictures"
        );
        assert_eq!(Solution::simplify_path("/../".to_owned()), "/");
        assert_eq!(
            Solution::simplify_path("/.../a/../b/c/../d/./".to_owned()),
            "/.../b/d"
        );
    }
}

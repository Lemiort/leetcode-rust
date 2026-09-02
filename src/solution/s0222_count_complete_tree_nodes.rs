/**
 * [222] Count Complete Tree Nodes
 *
 * Given the root of a complete binary tree, return the number of the nodes in the tree.
 * According to <a href="http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees" target="_blank">Wikipedia</a>, every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between 1 and 2^h nodes inclusive at the last level h.
 * Design an algorithm that runs in less than <code data-stringify-type="code">O(n) time complexity.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/complete.jpg" style="width: 372px; height: 302px;" />
 * Input: root = [1,2,3,4,5,6]
 * Output: 6
 *
 * <strong class="example">Example 2:
 *
 * Input: root = []
 * Output: 0
 *
 * <strong class="example">Example 3:
 *
 * Input: root = [1]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 5 * 10^4].
 * 	0 <= Node.val <= 5 * 10^4
 * 	The tree is guaranteed to be complete.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/count-complete-tree-nodes/
// discuss: https://leetcode.com/problems/count-complete-tree-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_height = Solution::height(node.borrow().left.clone(), true);
            let right_height = Solution::height(node.borrow().right.clone(), false);
            if left_height == right_height {
                (1 << (left_height + 1)) - 1
            } else {
                1 + Solution::count_nodes(node.borrow().left.clone())
                    + Solution::count_nodes(node.borrow().right.clone())
            }
        } else {
            0
        }
    }

    pub fn height(root: Option<Rc<RefCell<TreeNode>>>, go_left: bool) -> i32 {
        let mut height = 0;
        let mut node = root;
        while let Some(n) = node {
            height += 1;
            node = if go_left {
                n.borrow().left.clone()
            } else {
                n.borrow().right.clone()
            };
        }
        height
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_222() {
        assert_eq!(
            Solution::count_nodes(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6)
            ])),
            6
        );
        //assert_eq!(Solution::count_nodes(to_tree(vec![])), 0);
        assert_eq!(Solution::count_nodes(to_tree(vec![Some(1)])), 1);
    }
}

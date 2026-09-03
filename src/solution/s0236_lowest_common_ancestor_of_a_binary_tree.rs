/**
 * [236] Lowest Common Ancestor of a Binary Tree
 *
 * Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
 * According to the <a href="https://en.wikipedia.org/wiki/Lowest_common_ancestor" target="_blank">definition of LCA on Wikipedia</a>: &ldquo;The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).&rdquo;
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
 * Output: 3
 * Explanation: The LCA of nodes 5 and 1 is 3.
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
 * Output: 5
 * Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
 *
 * <strong class="example">Example 3:
 *
 * Input: root = [1,2], p = 1, q = 2
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [2, 10^5].
 * 	-10^9 <= Node.val <= 10^9
 * 	All Node.val are unique.
 * 	p != q
 * 	p and q will exist in the tree.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
// discuss: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.and_then(|node| {
            let node_val = node.borrow().val;

            // if current node is p or q, return current node
            if let Some(pv) = p.as_ref().map(|x| x.borrow().val) {
                if node_val == pv {
                    return Some(node.clone());
                }
            }
            if let Some(qv) = q.as_ref().map(|x| x.borrow().val) {
                if node_val == qv {
                    return Some(node.clone());
                }
            }

            // Search in left and right subtrees
            let left = {
                let node_ref = node.borrow();
                Self::lowest_common_ancestor(node_ref.left.clone(), p.clone(), q.clone())
            };
            let right = {
                let node_ref = node.borrow();
                Self::lowest_common_ancestor(node_ref.right.clone(), p, q)
            };

            // If both left and right return a result, current node is the LCA
            // If only one side has a result, that result is the LCA
            // If neither has a result, return None
            match (left, right) {
                (Some(l), Some(r)) => Some(node.clone()),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None,
            }
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_236() {
        // Test case 1: p=5, q=1, expected LCA=3
        let result1 = Solution::lowest_common_ancestor(
            to_tree(vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ]),
            to_tree(vec![Some(5)]),
            to_tree(vec![Some(1)]),
        );
        assert_eq!(result1.as_ref().map(|n| n.borrow().val), Some(3));

        // Test case 2: p=5, q=4, expected LCA=5
        let result2 = Solution::lowest_common_ancestor(
            to_tree(vec![
                Some(3),
                Some(5),
                Some(1),
                Some(6),
                Some(2),
                Some(0),
                Some(8),
                None,
                None,
                Some(7),
                Some(4),
            ]),
            to_tree(vec![Some(5)]),
            to_tree(vec![Some(4)]),
        );
        assert_eq!(result2.as_ref().map(|n| n.borrow().val), Some(5));
    }
}

/**
 * [863] All Nodes Distance K in Binary Tree
 *
 * Given the root of a binary tree, the value of a target node target, and an integer k, return an array of the values of all nodes that have a distance k from the target node.
 * You can return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/28/sketch0.png" style="width: 500px; height: 429px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
 * Output: [7,4,1]
 * Explanation: The nodes that are a distance 2 from the target node (with value 5) have values 7, 4, and 1.
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [1], target = 1, k = 3
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 500].
 * 	0 <= Node.val <= 500
 * 	All the values Node.val are unique.
 * 	target is the value of one of the nodes in the tree.
 * 	0 <= k <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/
// discuss: https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut result = Vec::new();
        let mut parent_map = std::collections::HashMap::new();
        Self::build_parent_map(&root, &mut parent_map);

        let target_val = target.as_ref().map(|t| t.borrow().val).unwrap_or(-1);
        let actual_target = Self::find_node(&root, target_val);

        // collect nodes in the subtree of target
        Self::collect_nodes_at_distance_k(actual_target.clone(), k, &mut result);

        // collect nodes by going up from target's parent
        if let Some(t) = actual_target {
            if let Some(parent) = parent_map.get(&t.borrow().val) {
                Self::collect_nodes_at_distance_k_from_parent(
                    parent.clone(),
                    k - 1,
                    &parent_map,
                    &mut result,
                    Some(t.borrow().val),
                );
            }
        }

        return result;
    }

    pub fn find_node(
        node: &Option<Rc<RefCell<TreeNode>>>,
        target_val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node {
            if n.borrow().val == target_val {
                return Some(n.clone());
            }
            if let Some(found) = Self::find_node(&n.borrow().left, target_val) {
                return Some(found);
            }
            if let Some(found) = Self::find_node(&n.borrow().right, target_val) {
                return Some(found);
            }
        }
        None
    }

    pub fn build_parent_map(
        node: &Option<Rc<RefCell<TreeNode>>>,
        parent_map: &mut std::collections::HashMap<i32, Option<Rc<RefCell<TreeNode>>>>,
    ) {
        if let Some(n) = node {
            if let Some(left) = n.borrow().left.clone() {
                parent_map.insert(left.borrow().val, Some(n.clone()));
                Self::build_parent_map(&Some(left), parent_map);
            }
            if let Some(right) = n.borrow().right.clone() {
                parent_map.insert(right.borrow().val, Some(n.clone()));
                Self::build_parent_map(&Some(right), parent_map);
            }
        }
    }

    pub fn collect_nodes_at_distance_k(
        node: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
        result: &mut Vec<i32>,
    ) {
        if let Some(n) = node {
            if k == 0 {
                result.push(n.borrow().val);
            } else {
                Self::collect_nodes_at_distance_k(n.borrow().left.clone(), k - 1, result);
                Self::collect_nodes_at_distance_k(n.borrow().right.clone(), k - 1, result);
            }
        }
    }

    pub fn collect_nodes_at_distance_k_from_parent(
        node: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
        parent_map: &std::collections::HashMap<i32, Option<Rc<RefCell<TreeNode>>>>,
        result: &mut Vec<i32>,
        excluded: Option<i32>,
    ) {
        if let Some(n) = node {
            let node_val = n.borrow().val;
            if k == 0 {
                result.push(node_val);
            } else {
                let borrowed = n.borrow();
                if let Some(left) = borrowed.left.clone() {
                    // exclude parent node
                    if excluded.is_none() || left.borrow().val != excluded.unwrap() {
                        Self::collect_nodes_at_distance_k(Some(left), k - 1, result);
                    }
                }
                if let Some(right) = borrowed.right.clone() {
                    // exclude parent node
                    if excluded.is_none() || right.borrow().val != excluded.unwrap() {
                        Self::collect_nodes_at_distance_k(Some(right), k - 1, result);
                    }
                }

                if let Some(parent) = parent_map.get(&node_val) {
                    Self::collect_nodes_at_distance_k_from_parent(
                        parent.clone(),
                        k - 1,
                        parent_map,
                        result,
                        Some(node_val),
                    );
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_863() {
        assert_eq!(
            Solution::distance_k(
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
                    Some(4)
                ]),
                to_tree(vec![Some(5)]),
                2
            ),
            vec![7, 4, 1]
        );

        assert!(Solution::distance_k(to_tree(vec![Some(1)]), to_tree(vec![Some(1)]), 3).is_empty(),);
    }
}

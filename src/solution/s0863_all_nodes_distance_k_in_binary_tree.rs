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
        mut k: i32,
    ) -> Vec<i32> {
        let mut result = Vec::new();
        let mut adjacency = std::collections::HashMap::new();
        let mut q = std::collections::VecDeque::new();
        q.push_back(root.unwrap());
        // fill adjacency list
        while let Some(ref node_ref) = q.pop_back() {
            let node = node_ref.borrow();
            if let Some(ref left) = node.left {
                q.push_back(left.clone());
                adjacency
                    .entry(node.val)
                    .or_insert_with(Vec::new)
                    .push(left.borrow().val);
                adjacency
                    .entry(left.borrow().val)
                    .or_insert_with(Vec::new)
                    .push(node.val);
            }
            if let Some(ref right) = node.right {
                q.push_back(right.clone());
                adjacency
                    .entry(node.val)
                    .or_insert_with(Vec::new)
                    .push(right.borrow().val);
                adjacency
                    .entry(right.borrow().val)
                    .or_insert_with(Vec::new)
                    .push(node.val);
            }
        }

        let mut q = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let target = target.unwrap().borrow().val;
        visited.insert(target);
        q.push_front(target);

        // extract all nodes at distance k from target
        while k > 0 {
            let num_elements = q.len();
            k -= 1;
            // go over all nodes in the current level
            for _ in 0..num_elements {
                let node = q.pop_back().unwrap();
                // go over all adjacent nodes
                if let Some(adjacent_nodes) = adjacency.get(&node) {
                    for &adjacent_node in adjacent_nodes {
                        if !visited.contains(&adjacent_node) {
                            visited.insert(adjacent_node);
                            q.push_front(adjacent_node);
                        }
                    }
                }
            }
        }

        // write all nodes in the current level to result
        while let Some(node) = q.pop_back() {
            result.push(node);
        }
        return result;
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
            vec![1, 7, 4]
        );

        assert!(Solution::distance_k(to_tree(vec![Some(1)]), to_tree(vec![Some(1)]), 3).is_empty(),);
    }
}

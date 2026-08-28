/**
 * [652] Find Duplicate Subtrees
 *
 * Given the root of a binary tree, return all duplicate subtrees.
 * For each kind of duplicate subtrees, you only need to return the root node of any one of them.
 * Two trees are duplicate if they have the same structure with the same node values.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e1.jpg" style="width: 450px; height: 354px;" />
 * Input: root = [1,2,3,4,null,2,4,null,null,4]
 * Output: [[2,4],[4]]
 *
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e2.jpg" style="width: 321px; height: 201px;" />
 * Input: root = [2,1,1]
 * Output: [[1]]
 *
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e33.jpg" style="width: 450px; height: 303px;" />
 * Input: root = [2,2,2,3,null,3,null]
 * Output: [[2,3],[3]]
 *
 *  
 * Constraints:
 *
 * 	The number of the nodes in the tree will be in the range [1, 5000]
 * 	-200 <= Node.val <= 200
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/find-duplicate-subtrees/
// discuss: https://leetcode.com/problems/find-duplicate-subtrees/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Subtree {
    val: i32,
    left: usize,
    right: usize,
}

type SubtreeId = usize;

impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        // Assign each subtree a canonical ID based on its value and child IDs.
        // A subtree is duplicate when its ID is encountered for the second time.
        fn visit(
            node: Option<Rc<RefCell<TreeNode>>>,
            ids: &mut HashMap<Subtree, usize>,
            counts: &mut HashMap<SubtreeId, usize>,
            duplicates: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
            next_id: &mut usize,
        ) -> SubtreeId {
            let Some(node) = node else {
                return 0;
            };

            let (value, left, right) = {
                let node_ref = node.borrow();
                (
                    node_ref.val,
                    visit(node_ref.left.clone(), ids, counts, duplicates, next_id),
                    visit(node_ref.right.clone(), ids, counts, duplicates, next_id),
                )
            };
            let key = Subtree {
                val: value,
                left,
                right,
            };
            let id: SubtreeId = if let Some(&id) = ids.get(&key) {
                id
            } else {
                let id: SubtreeId = *next_id;
                *next_id += 1;
                ids.insert(key, id);
                id
            };

            let count = counts.entry(id).or_insert(0);
            *count += 1;
            if *count == 2 {
                duplicates.push(Some(node));
            }
            id
        }

        let mut ids = HashMap::new();
        let mut counts = HashMap::new();
        let mut duplicates = Vec::new();
        let mut next_id = 1;
        visit(root, &mut ids, &mut counts, &mut duplicates, &mut next_id);
        duplicates
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    fn signature(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => "#".to_string(),
            Some(node) => {
                let node = node.borrow();
                format!(
                    "{} {} {}",
                    node.val,
                    signature(&node.left),
                    signature(&node.right)
                )
            }
        }
    }

    fn duplicate_signatures(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = Solution::find_duplicate_subtrees(root)
            .iter()
            .map(signature)
            .collect::<Vec<_>>();
        result.sort();
        result
    }

    #[test]
    fn test_652() {
        let mut expected = vec![
            signature(&to_tree(vec![Some(2), Some(4), None])),
            signature(&to_tree(vec![Some(4)])),
        ];
        expected.sort();
        assert_eq!(
            duplicate_signatures(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                None,
                Some(2),
                Some(4),
                None,
                None,
                Some(4),
            ])),
            expected
        );
        assert_eq!(
            duplicate_signatures(to_tree(vec![Some(2), Some(1), Some(1)])),
            vec![signature(&to_tree(vec![Some(1)]))]
        );
        assert_eq!(
            duplicate_signatures(to_tree(vec![
                Some(2),
                Some(2),
                Some(2),
                Some(3),
                None,
                Some(3),
                None,
            ])),
            vec![
                signature(&to_tree(vec![Some(2), Some(3), None])),
                signature(&to_tree(vec![Some(3)])),
            ]
        );
    }
}

/*
 * @lc app=leetcode id=515 lang=rust
 *
 * [515] Find Largest Value in Each Tree Row
 *
 * https://leetcode.com/problems/find-largest-value-in-each-tree-row/description/
 *
 * algorithms
 * Medium (58.62%)
 * Likes:    592
 * Dislikes: 49
 * Total Accepted:    74.9K
 * Total Submissions: 127.4K
 * Testcase Example:  '[1,3,2,5,3,null,9]'
 *
 * You need to find the largest value in each row of a binary tree.
 *
 * Example:
 *
 * Input:
 *
 * ⁠         1
 * ⁠        / \
 * ⁠       3   2
 * ⁠      / \   \
 * ⁠     5   3   9
 *
 * Output: [1, 3, 9]
 *
 *
 *
 */

// @lc code=start
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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        pub fn fill_vec_result(
            tree: &Option<Rc<RefCell<TreeNode>>>,
            level: usize,
            result: &mut Vec<i32>,
        ) {
            match tree {
                Some(node) => {
                    let len = result.len();
                    let node = node.borrow();
                    let value = node.val.to_owned();
                    if len <= level {
                        result.push(value);
                    } else {
                        result[level] = result[level].max(value);
                    }
                    fill_vec_result(&node.left, level + 1, result);
                    fill_vec_result(&node.right, level + 1, result);
                }
                None => {}
            };
        }
        let mut result = vec![];
        fill_vec_result(&root, 0, &mut result);
        result
    }
}
// @lc code=end

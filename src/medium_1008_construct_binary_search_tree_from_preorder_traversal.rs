/*
 * @lc app=leetcode id=1008 lang=rust
 *
 * [1008] Construct Binary Search Tree from Preorder Traversal
 *
 * https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/description/
 *
 * algorithms
 * Medium (73.73%)
 * Likes:    402
 * Dislikes: 15
 * Total Accepted:    27.3K
 * Total Submissions: 37K
 * Testcase Example:  '[8,5,1,7,10,12]'
 *
 * Return the root node of a binary search tree that matches the given preorder
 * traversal.
 *
 * (Recall that a binary search tree is a binary tree where for every node, any
 * descendant of node.left has a value < node.val, and any descendant of
 * node.right has a value > node.val.  Also recall that a preorder traversal
 * displays the value of the node first, then traverses node.left, then
 * traverses node.right.)
 *
 *
 *
 * Example 1:
 *
 *
 * Input: [8,5,1,7,10,12]
 * Output: [8,5,10,1,7,null,12]
 *
 *
 *
 *
 *
 * Note:
 *
 *
 * 1 <= preorder.length <= 100
 * The values of preorder are distinct.
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
///思路，不断将数组拆分为根结点，左，右，知道最后只剩下不多于一个的元素
impl Solution {
    pub fn bst_from_preorder(mut preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = preorder.len();
        match len {
            0 => None,
            1 => {
                let root = preorder[0];
                Some(Rc::new(RefCell::new(TreeNode::new(root))))
            }
            other => {
                //找到根结点
                let (root, children) = preorder.split_at_mut(1);
                let root = root[0];
                let (mut left_children, mut right_children) =
                    children.iter().partition(|&&n| n < root);
                let mut sub_tree = Rc::new(RefCell::new(TreeNode::new(root)));
                {
                    let mut borrow_sub_tree = sub_tree.borrow_mut();
                    borrow_sub_tree.left = Solution::bst_from_preorder(left_children);
                    borrow_sub_tree.right = Solution::bst_from_preorder(right_children);
                }
                Some(sub_tree)
            }
        }
    }
}
// @lc code=end

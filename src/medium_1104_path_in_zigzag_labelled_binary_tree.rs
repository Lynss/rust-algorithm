/*
 * @lc app=leetcode id=1104 lang=rust
 *
 * [1104] Path In Zigzag Labelled Binary Tree
 *
 * https://leetcode.com/problems/path-in-zigzag-labelled-binary-tree/description/
 *
 * algorithms
 * Medium (70.66%)
 * Likes:    159
 * Dislikes: 104
 * Total Accepted:    10.6K
 * Total Submissions: 15K
 * Testcase Example:  '14'
 *
 * In an infinite binary tree where every node has two children, the nodes are
 * labelled in row order.
 *
 * In the odd numbered rows (ie., the first, third, fifth,...), the labelling
 * is left to right, while in the even numbered rows (second, fourth,
 * sixth,...), the labelling is right to left.
 *
 *
 *
 * Given the label of a node in this tree, return the labels in the path from
 * the root of the tree to the node with that label.
 *
 *
 * Example 1:
 *
 *
 * Input: label = 14
 * Output: [1,3,4,14]
 *
 *
 * Example 2:
 *
 *
 * Input: label = 26
 * Output: [1,2,6,10,26]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= label <= 10^6
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    ///思路 因为树本身其实是能确定的，所以先根据树找到他的层级，然后一层层的上朔到跟节点
    /// 1 <= label <= 10^6 所以其实最大层数能够确定了,0 到 19 层
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut result = vec![];
        //找到对应的级别
        let level = (0..20)
            .find(|&l| label >= 2_i32.pow(l as u32) && label < 2_i32.pow(l as u32 + 1))
            .unwrap();
        let min = 2_i32.pow(level);
        //位置需要根据奇偶进行修正
        let level_num = 2_i32.pow(level as u32 + 1) - 2_i32.pow(level as u32);
        let pos = if level % 2 == 1 {
            level_num - 1 + min - label
        } else {
            label - min
        };
        //定义一个能够根据级别和位置找到上级的方法
        pub fn find_parent(pos: i32, l: i32, result: &mut Vec<i32>) {
            if l <= 0 {
                result.push(1);
                return;
            }
            let current_value = if l % 2 == 1 {
                2_i32.pow(l as u32 + 1) - 1 - pos
            } else {
                2_i32.pow(l as u32) + pos
            };
            result.push(current_value);
            let parent_level = l - 1;
            let parent_pos = pos / 2;
            find_parent(parent_pos, parent_level, result)
        }
        find_parent(pos, level as i32, &mut result);
        result.reverse();
        result
    }
}
// @lc code=end

pub fn main() {
    let test = 14;
    Solution::path_in_zig_zag_tree(test);
}

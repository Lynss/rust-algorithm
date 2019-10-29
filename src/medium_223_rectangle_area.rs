/*
 * @lc app=leetcode id=223 lang=rust
 *
 * [223] Rectangle Area
 *
 * https://leetcode.com/problems/rectangle-area/description/
 *
 * algorithms
 * Medium (36.49%)
 * Likes:    294
 * Dislikes: 569
 * Total Accepted:    94.6K
 * Total Submissions: 258.7K
 * Testcase Example:  '-3\n0\n3\n4\n0\n-1\n9\n2'
 *
 * Find the total area covered by two rectilinear rectangles in a 2D plane.
 *
 * Each rectangle is defined by its bottom left corner and top right corner as
 * shown in the figure.
 *
 *
 *
 * Example:
 *
 *
 * Input: A = -3, B = 0, C = 3, D = 4, E = 0, F = -1, G = 9, H = 2
 * Output: 45
 *
 * Note:
 *
 * Assume that the total area is never beyond the maximum possible value of
 * int.
 *
 */

// @lc code=start
///思路，先求两矩形的面积之和，减去他们的重叠面积
/// 因此，这里主要是进行重叠区域的判断，即x或y轴上都有大于0的重叠区域
impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        use std::cmp;
        let sum = (c - a) * (d - b) + (g - e) * (h - f);
        let x_vec = vec![a, c, e, g];
        let y_vec = vec![b, d, f, h];
        fn get_overlap(a_i: i32, c_i: i32, e_i: i32, g_i: i32) -> i32 {
            if a_i > g_i || c_i < e_i {
                0
            } else if e_i > a_i {
                cmp::min(g_i, c_i) - e_i
            } else {
                cmp::min(g_i, c_i) - a_i
            }
        }
        let x_overlap = get_overlap(a, c, e, g);
        let y_overlap = get_overlap(b, d, f, h);
        sum - x_overlap * y_overlap
    }
}
// @lc code=end

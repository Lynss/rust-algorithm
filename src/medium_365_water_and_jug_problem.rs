/*
 * @lc app=leetcode id=365 lang=rust
 *
 * [365] Water and Jug Problem
 *
 * https://leetcode.com/problems/water-and-jug-problem/description/
 *
 * algorithms
 * Medium (29.53%)
 * Likes:    200
 * Dislikes: 585
 * Total Accepted:    31.9K
 * Total Submissions: 107.7K
 * Testcase Example:  '3\n5\n4'
 *
 * You are given two jugs with capacities x and y litres. There is an infinite
 * amount of water supply available. You need to determine whether it is
 * possible to measure exactly z litres using these two jugs.
 *
 * If z liters of water is measurable, you must have z liters of water
 * contained within one or both buckets by the end.
 *
 * Operations allowed:
 *
 *
 * Fill any of the jugs completely with water.
 * Empty any of the jugs.
 * Pour water from one jug into another till the other jug is completely full
 * or the first jug itself is empty.
 *
 *
 * Example 1: (From the famous "Die Hard" example)
 *
 *
 * Input: x = 3, y = 5, z = 4
 * Output: True
 *
 *
 * Example 2:
 *
 *
 * Input: x = 2, y = 6, z = 5
 * Output: False
 *
 */

// @lc code=start
///思路，首先考虑是否能满足z <= x + y 以及是否可能为 0
/// 两桶水最终都可以转换为1或2或他们相等的本身，如果需要的是奇数，则需要求能转换为1
impl Solution {
    pub fn can_measure_water(mut x: i32, mut y: i32, z: i32) -> bool {
        z <= x + y && {
            if x == 0 || y == 0 {
                if x == 0 && y == 0 {
                    z == 0
                } else {
                    z % (x + y) == 0
                }
            } else {
                while x != y && (x > 2 || y > 2) {
                    if x > y {
                        x = x - y;
                    } else {
                        y = y - x;
                    }
                }
                if x == y {
                    z % x == 0
                } else if z % 2 != 0 {
                    x == 1 || y == 1
                } else {
                    true
                }
            }
        }
    }
}
// @lc code=end

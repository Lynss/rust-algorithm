/*
 * @lc app=leetcode id=693 lang=rust
 *
 * [693] Binary Number with Alternating Bits
 *
 * https://leetcode.com/problems/binary-number-with-alternating-bits/description/
 *
 * algorithms
 * Easy (58.37%)
 * Likes:    329
 * Dislikes: 73
 * Total Accepted:    47.9K
 * Total Submissions: 82K
 * Testcase Example:  '5'
 *
 * Given a positive integer, check whether it has alternating bits: namely, if
 * two adjacent bits will always have different values.
 *
 * Example 1:
 *
 * Input: 5
 * Output: True
 * Explanation:
 * The binary representation of 5 is: 101
 *
 *
 *
 * Example 2:
 *
 * Input: 7
 * Output: False
 * Explanation:
 * The binary representation of 7 is: 111.
 *
 *
 *
 * Example 3:
 *
 * Input: 11
 * Output: False
 * Explanation:
 * The binary representation of 11 is: 1011.
 *
 *
 *
 * Example 4:
 *
 * Input: 10
 * Output: True
 * Explanation:
 * The binary representation of 10 is: 1010.
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut before = -1;
        let mut k = n / 2;
        let mut j = n - 2 * k;
        let mut result = true;
        while k > 0 || j > 0 {
            if j == before {
                result = false;
                break;
            }
            let temp = k;
            before = j;
            k = temp / 2;
            j = temp - 2 * k;
        }
        result
    }
}
// @lc code=end

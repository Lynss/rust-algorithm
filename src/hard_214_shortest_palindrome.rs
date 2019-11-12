/*
 * @lc app=leetcode id=214 lang=rust
 *
 * [214] Shortest Palindrome
 *
 * https://leetcode.com/problems/shortest-palindrome/description/
 *
 * algorithms
 * Hard (28.28%)
 * Likes:    804
 * Dislikes: 92
 * Total Accepted:    83.7K
 * Total Submissions: 294.5K
 * Testcase Example:  '"aacecaaa"'
 *
 * Given a string s, you are allowed to convert it to a palindrome by adding
 * characters in front of it. Find and return the shortest palindrome you can
 * find by performing this transformation.
 *
 * Example 1:
 *
 *
 * Input: "aacecaaa"
 * Output: "aaacecaaa"
 *
 *
 * Example 2:
 *
 *
 * Input: "abcd"
 * Output: "dcbabcd"
 */

// @lc code=start
///思路，需要构建最短的回文，就需要从最后到开头的找到最长的回文串
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let len = s.len();
        if len == 0 {
            return "".to_owned();
        }
        let origin = s;
        let rev: String = origin.chars().rev().collect();
        let target = (0..len)
            .find(|&i| {
                let origin_part = &origin[0..len - i];
                let rev_part = &rev[i..len];
                rev_part == origin_part
            })
            .expect("it won't happen");
        let mut result = rev;
        result.push_str(&origin[len - target..len]);
        result
    }
}
// @lc code=end

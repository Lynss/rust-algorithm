/*
 * @lc app=leetcode id=1044 lang=rust
 *
 * [1044] Longest Duplicate Substring
 *
 * https://leetcode.com/problems/longest-duplicate-substring/description/
 *
 * algorithms
 * Hard (23.43%)
 * Likes:    123
 * Dislikes: 91
 * Total Accepted:    4.1K
 * Total Submissions: 17.1K
 * Testcase Example:  '"banana"'
 *
 * Given a string S, consider all duplicated substrings: (contiguous)
 * substrings of S that occur 2 or more times.  (The occurrences may overlap.)
 *
 * Return any duplicated substring that has the longest possible length.  (If S
 * does not have a duplicated substring, the answer is "".)
 *
 *
 *
 * Example 1:
 *
 *
 * Input: "banana"
 * Output: "ana"
 *
 *
 * Example 2:
 *
 *
 * Input: "abcd"
 * Output: ""
 *
 *
 *
 *
 * Note:
 *
 *
 * 2 <= S.length <= 10^5
 * S consists of lowercase English letters.
 *
 */

struct Solution;

// @lc code=start
///思路：创建一个同样的字符串，并向右移动一位，然后和原字符串进行比较，逐渐移动位置，并找到每次的最大重复长度及对应的字符串
///超时。。。
impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let len = s.len();
        //开始从一个长度进行判断
        let mut start = 1;
        //前一个是否相同
        let mut before_same = false;
        //重复字段
        let mut overlap = "".to_owned();
        //最大值
        let mut max = 0;
        //最终结果
        let mut result = "".to_owned();
        while start < len - max {
            for i in 0..(len - start) {
                let origin_c = s[start + i];
                let compare_c = s[i];
                if origin_c != compare_c {
                    before_same = false;
                    continue;
                }
                if before_same {
                    overlap.push(origin_c);
                } else {
                    before_same = true;
                    overlap = origin_c.to_string();
                }
                let overlap_len = overlap.len();
                if overlap_len > max {
                    result = overlap.clone();
                    max = result.len();
                }
            }
            start += 1;
        }
        result
    }
}
// @lc code=end

pub fn main() {
    let a = "banana".to_owned();
    Solution::longest_dup_substring(a);
}

/*
 * @lc app=leetcode id=937 lang=rust
 *
 * [937] Reorder Data in Log Files
 *
 * https://leetcode.com/problems/reorder-data-in-log-files/description/
 *
 * algorithms
 * Easy (54.29%)
 * Likes:    338
 * Dislikes: 1047
 * Total Accepted:    65.6K
 * Total Submissions: 121.6K
 * Testcase Example:  '["dig1 8 1 5 1","let1 art can","dig2 3 6","let2 own kit dig","let3 art zero"]'
 *
 * You have an array of logs.  Each log is a space delimited string of words.
 *
 * For each log, the first word in each log is an alphanumeric identifier.
 * Then, either:
 *
 *
 * Each word after the identifier will consist only of lowercase letters,
 * or;
 * Each word after the identifier will consist only of digits.
 *
 *
 * We will call these two varieties of logs letter-logs and digit-logs.  It is
 * guaranteed that each log has at least one word after its identifier.
 *
 * Reorder the logs so that all of the letter-logs come before any digit-log.
 * The letter-logs are ordered lexicographically ignoring identifier, with the
 * identifier used in case of ties.  The digit-logs should be put in their
 * original order.
 *
 * Return the final order of the logs.
 *
 *
 * Example 1:
 * Input: logs = ["dig1 8 1 5 1","let1 art can","dig2 3 6","let2 own kit
 * dig","let3 art zero"]
 * Output: ["let1 art can","let3 art zero","let2 own kit dig","dig1 8 1 5
 * 1","dig2 3 6"]
 *
 *
 * Constraints:
 *
 *
 * 0 <= logs.length <= 100
 * 3 <= logs[i].length <= 100
 * logs[i] is guaranteed to have an identifier, and a word after the
 * identifier.
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        use std::cmp::Ordering;
        let mut logs = logs.into_iter().enumerate().collect::<Vec<_>>();
        let is_num = |x: char| x.to_string().parse::<i32>().is_ok();
        logs.sort_by(|(i, il), (j, jl)| {
            let i_partition_index =
                il.chars().enumerate().find(|(_, x)| x == &' ').unwrap().0;
            let j_partition_index =
                jl.chars().enumerate().find(|(_, x)| x == &' ').unwrap().0;
            let il = il.chars().collect::<Vec<_>>();
            let jl = jl.chars().collect::<Vec<_>>();
            let (il_before, il_after) = il.split_at(i_partition_index);
            let (jl_before, jl_after) = jl.split_at(j_partition_index);
            let il_after = &il_after[1..];
            let jl_after = &jl_after[1..];
            let ilf = il_after[0];
            let jlf = jl_after[0];
            if is_num(ilf) && is_num(jlf) {
                i.cmp(j)
            } else if is_num(ilf) {
                Ordering::Greater
            } else if is_num(jlf) {
                Ordering::Less
            } else {
                il_after.cmp(jl_after).then(il_before.cmp(jl_before))
            }
        });
        logs.into_iter().map(|(_, log)| log).collect()
    }
}
// @lc code=end

pub fn main() {
    let test = vec![
        "dig1 8 1 5 1".to_owned(),
        "let1 art can".to_owned(),
        "dig2 3 6".to_owned(),
        "let2 own kit dig".to_owned(),
        "let3 art zero".to_owned(),
    ];
    dbg!(Solution::reorder_log_files(test));
}

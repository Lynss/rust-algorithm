/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 *
 * https://leetcode.com/problems/3sum/description/
 *
 * algorithms
 * Medium (24.92%)
 * Likes:    4888
 * Dislikes: 583
 * Total Accepted:    701.7K
 * Total Submissions: 2.8M
 * Testcase Example:  '[-1,0,1,2,-1,-4]'
 *
 * Given an array nums of n integers, are there elements a, b, c in nums such
 * that a + b + c = 0? Find all unique triplets in the array which gives the
 * sum of zero.
 *
 * Note:
 *
 * The solution set must not contain duplicate triplets.
 *
 * Example:
 *
 *
 * Given array nums = [-1, 0, 1, 2, -1, -4],
 *
 * A solution set is:
 * [
 * ⁠ [-1, 0, 1],
 * ⁠ [-1, -1, 2]
 * ]
 *
 */

// @lc code=start
///思路,将 3 个之和为 0 一步步替换为 2 个之和,单个值
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let len = nums.len();
        if len < 3 {
            return vec![];
        }
        let mut records = HashSet::new();
        nums.par_iter()
            .enumerate()
            .take(len - 2)
            .for_each(|(i, &num_i)| {
                let total = 0 - num_i;
                nums.par_iter()
                    .enumerate()
                    .take(len - 1)
                    .skip(i + 1)
                    .for_each(|(j, num_j)| {
                        let total = total - num_j;
                        nums.par_iter()
                            .skip(j + 1)
                            .filter(|num_k| num_k == &&total)
                            .for_each(|num_k| {
                                let mut tmp_result =
                                    vec![num_i.clone(), num_j.clone(), num_k.clone()];
                                tmp_result.sort();
                                records.insert(tmp_result);
                            })
                    })
            });
        records.into_iter().collect()
    }
}
// @lc code=end

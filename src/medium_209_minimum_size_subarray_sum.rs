/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 *
 * https://leetcode.com/problems/minimum-size-subarray-sum/description/
 *
 * algorithms
 * Medium (35.76%)
 * Likes:    1474
 * Dislikes: 83
 * Total Accepted:    207.1K
 * Total Submissions: 575.8K
 * Testcase Example:  '7\n[2,3,1,2,4,3]'
 *
 * Given an array of n positive integers and a positive integer s, find the
 * minimal length of a contiguous subarray of which the sum ≥ s. If there isn't
 * one, return 0 instead.
 *
 * Example:
 *
 *
 * Input: s = 7, nums = [2,3,1,2,4,3]
 * Output: 2
 * Explanation: the subarray [4,3] has the minimal length under the problem
 * constraint.
 *
 * Follow up:
 *
 * If you have figured out the O(n) solution, try coding another solution of
 * which the time complexity is O(n log n).
 *
 */
pub struct Solution;

// @lc code=start
///思路，找到所有可能的组合，找到最小的
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let mut maybe = vec![];
        let mut result = vec![];
        for num in nums {
            if num >= s {
                return 1;
            }
            // num < s 时
            let sum = maybe.iter().sum::<i32>();
            let mut temp_sum = sum + num;
            let maybe_len = maybe.len();
            if temp_sum < s {
                maybe.push(num);
                continue;
            }
            for i in 0..maybe_len {
                temp_sum -= maybe[i];
                if temp_sum < s {
                    result.push(maybe.len() - i + 1);
                    maybe.push(num);
                    maybe = maybe.split_off(i + 2);
                    break;
                }
            }
        }
        result.iter().min().get_or_insert(&0).to_owned() as i32
    }
}
// @lc code=end
pub fn main() {
    let test = vec![2, 3, 1, 2, 4, 3];
    Solution::min_sub_array_len(7, test);
}

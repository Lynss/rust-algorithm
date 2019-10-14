/*
 * @lc app=leetcode id=974 lang=rust
 *
 * [974] Subarray Sums Divisible by K
 *
 * https://leetcode.com/problems/subarray-sums-divisible-by-k/description/
 *
 * algorithms
 * Medium (46.83%)
 * Likes:    443
 * Dislikes: 40
 * Total Accepted:    18.7K
 * Total Submissions: 40K
 * Testcase Example:  '[4,5,0,-2,-3,1]\n5'
 *
 * Given an array A of integers, return the number of (contiguous, non-empty)
 * subarrays that have a sum divisible by K.
 *
 *
 *
 *
 * Example 1:
 *
 *
 * Input: A = [4,5,0,-2,-3,1], K = 5
 * Output: 7
 * Explanation: There are 7 subarrays with a sum divisible by K = 5:
 * [4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2,
 * -3]
 *
 *
 *
 *
 * Note:
 *
 *
 * 1 <= A.length <= 30000
 * -10000 <= A[i] <= 10000
 * 2 <= K <= 10000
 *
 *
 */
// @lc code=start
///思路：只要两次和的mod相等，说明存在可以整除的，例如，3 % 5 = 3; (3+1) % 5 = 4; (3+1+9)%5 = 3,说明1+9能整除5
impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut mod_map = HashMap::new();
        //mod 为0直接说明有能够整除的；
        mod_map.insert(0, 1);
        let mut result = 0;
        let mut sum = 0;
        for i in a {
            sum = (sum + i) % k;
            if sum < 0 {
                sum += k
            }
            let has_count = mod_map.entry(sum).or_insert(0);
            result += *has_count;
            *has_count += 1;
        }
        result
    }
}
// @lc code=end

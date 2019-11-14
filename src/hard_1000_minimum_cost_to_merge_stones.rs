/*
 * @lc app=leetcode id=1000 lang=rust
 *
 * [1000] Minimum Cost to Merge Stones
 *
 * https://leetcode.com/problems/minimum-cost-to-merge-stones/description/
 *
 * algorithms
 * Hard (36.20%)
 * Likes:    345
 * Dislikes: 29
 * Total Accepted:    7.9K
 * Total Submissions: 21.5K
 * Testcase Example:  '[3,2,4,1]\n2'
 *
 * There are N piles of stones arranged in a row.  The i-th pile has stones[i]
 * stones.
 *
 * A move consists of merging exactly K consecutive piles into one pile, and
 * the cost of this move is equal to the total number of stones in these K
 * piles.
 *
 * Find the minimum cost to merge all piles of stones into one pile.  If it is
 * impossible, return -1.
 *
 *
 *
 *
 * Example 1:
 *
 *
 * Input: stones = [3,2,4,1], K = 2
 * Output: 20
 * Explanation:
 * We start with [3, 2, 4, 1].
 * We merge [3, 2] for a cost of 5, and we are left with [5, 4, 1].
 * We merge [4, 1] for a cost of 5, and we are left with [5, 5].
 * We merge [5, 5] for a cost of 10, and we are left with [10].
 * The total cost was 20, and this is the minimum possible.
 *
 *
 *
 * Example 2:
 *
 *
 * Input: stones = [3,2,4,1], K = 3
 * Output: -1
 * Explanation: After any merge operation, there are 2 piles left, and we can't
 * merge anymore.  So the task is impossible.
 *
 *
 *
 * Example 3:
 *
 *
 * Input: stones = [3,5,1,2,6], K = 3
 * Output: 25
 * Explanation:
 * We start with [3, 5, 1, 2, 6].
 * We merge [5, 1, 2] for a cost of 8, and we are left with [3, 8, 6].
 * We merge [3, 8, 6] for a cost of 17, and we are left with [17].
 * The total cost was 25, and this is the minimum possible.
 *
 *
 *
 *
 * Note:
 *
 *
 * 1 <= stones.length <= 30
 * 2 <= K <= 30
 * 1 <= stones[i] <= 100
 *
 *
 *
 *
 */

struct Solution;

// @lc code=start
///思路，石头数需满足(k-1)*n + k = len，否则认为无法移动。
/// 适合用动态规划来进行解决
/// dp(i,j,m)定义为将j-i块石头，每次移动k块，最后剩下m块
/// 此题相当于 dp(0,len,1)
/// 如果不满足 len - a*(k-1)=m return -1
/// 如果 m = 1 dp(i,j,1) = dp(i,j,k-1)+ sum(i,j);
/// dp(0,len,m) = min(dp(0,mid,1) + dp(mid,len,m-1)) ,如果有-1 直接等于 -1
impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        use std::f64::INFINITY;
        let len = stones.len();
        let mut records = HashMap::new();
        pub fn dp(
            i: i32,
            j: i32,
            m: i32,
            k: i32,
            stones: &Vec<i32>,
            records: &mut HashMap<(i32, i32, i32), f64>,
        ) -> f64 {
            let key = (i, j, m);
            if !records.contains_key(&key) {
                let new_value = if (j - i - m) % (k - 1) != 0 {
                    INFINITY
                } else if j - i == m {
                    0.0
                } else if j - i < m {
                    INFINITY
                } else if m == 1 {
                    dp(i, j, k, k, &stones, records)
                        + stones[i as usize..j as usize].iter().sum::<i32>() as f64
                } else {
                    ((i + 1)..j)
                        .step_by(k as usize - 1)
                        .map(|mid| {
                            dp(i, mid, 1, k, stones, records)
                                + dp(mid, j, m - 1, k, stones, records)
                        })
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap_or(INFINITY)
                };
                records.insert(key, new_value);
            }
            records.get(&(i, j, m)).unwrap().to_owned()
        }
        let result = dp(0, len as i32, 1, k, &stones, &mut records);
        dbg!(&result);
        if result == INFINITY {
            -1
        } else {
            result as i32
        }
    }
}
// @lc code=end

pub fn main() {
    let test = vec![
        95, 54, 31, 48, 44, 96, 99, 20, 51, 54, 18, 85, 25, 84, 91, 48, 40, 72, 22,
    ];
    Solution::merge_stones(test, 2);
}

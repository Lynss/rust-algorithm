/*
 * @lc app=leetcode id=554 lang=rust
 *
 * [554] Brick Wall
 *
 * https://leetcode.com/problems/brick-wall/description/
 *
 * algorithms
 * Medium (48.30%)
 * Likes:    664
 * Dislikes: 34
 * Total Accepted:    45.4K
 * Total Submissions: 93.2K
 * Testcase Example:  '[[1,2,2,1],[3,1,2],[1,3,2],[2,4],[3,1,2],[1,3,1,1]]'
 *
 * There is a brick wall in front of you. The wall is rectangular and has
 * several rows of bricks. The bricks have the same height but different width.
 * You want to draw a vertical line from the top to the bottom and cross the
 * least bricks.
 *
 * The brick wall is represented by a list of rows. Each row is a list of
 * integers representing the width of each brick in this row from left to
 * right.
 *
 * If your line go through the edge of a brick, then the brick is not
 * considered as crossed. You need to find out how to draw the line to cross
 * the least bricks and return the number of crossed bricks.
 *
 * You cannot draw a line just along one of the two vertical edges of the wall,
 * in which case the line will obviously cross no bricks.
 *
 *
 *
 * Example:
 *
 *
 * Input: [[1,2,2,1],
 * ⁠       [3,1,2],
 * ⁠       [1,3,2],
 * ⁠       [2,4],
 * ⁠       [3,1,2],
 * ⁠       [1,3,1,1]]
 *
 * Output: 2
 *
 * Explanation:
 *
 *
 *
 *
 *
 * Note:
 *
 *
 * The width sum of bricks in different rows are the same and won't exceed
 * INT_MAX.
 * The number of bricks in each row is in range [1,10,000]. The height of wall
 * is in range [1,10,000]. Total number of bricks of the wall won't exceed
 * 20,000.
 *
 *
 */
struct Solution;

// @lc code=start
///首先将每一层的转缝隙给算出来，找出来出现最多的缝隙
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let row_count = wall.len() as i32;
        let sum_width = wall[0].iter().sum::<i32>() as usize;
        let mut counter = HashMap::new();
        wall.iter().for_each(|row| {
            let mut before = 0;
            row.iter().for_each(|&i| {
                before += i as usize;
                if before < sum_width {
                    let countered = counter.entry(before).or_insert(0);
                    *countered += 1;
                }
            });
        });
        row_count
            - counter
                .into_iter()
                .map(|(key, value)| value)
                .take(sum_width)
                .max()
                .unwrap()
    }
}
// @lc code=end

pub fn main() {
    let test = vec![
        vec![1, 2, 2, 1],
        vec![3, 1, 2],
        vec![1, 3, 2],
        vec![2, 4],
        vec![3, 1, 2],
        vec![1, 3, 1, 1],
    ];
    dbg!(Solution::least_bricks(test));
}

/*
 * @lc app=leetcode id=1024 lang=rust
 *
 * [1024] Video Stitching
 *
 * https://leetcode.com/problems/video-stitching/description/
 *
 * algorithms
 * Medium (47.48%)
 * Likes:    239
 * Dislikes: 16
 * Total Accepted:    12.9K
 * Total Submissions: 27.1K
 * Testcase Example:  '[[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]]\n10'
 *
 * You are given a series of video clips from a sporting event that lasted T
 * seconds.  These video clips can be overlapping with each other and have
 * varied lengths.
 *
 * Each video clip clips[i] is an interval: it starts at time clips[i][0] and
 * ends at time clips[i][1].  We can cut these clips into segments freely: for
 * example, a clip [0, 7] can be cut into segments [0, 1] + [1, 3] + [3, 7].
 *
 * Return the minimum number of clips needed so that we can cut the clips into
 * segments that cover the entire sporting event ([0, T]).  If the task is
 * impossible, return -1.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: clips = [[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]], T = 10
 * Output: 3
 * Explanation:
 * We take the clips [0,2], [8,10], [1,9]; a total of 3 clips.
 * Then, we can reconstruct the sporting event as follows:
 * We cut [1,9] into segments [1,2] + [2,8] + [8,9].
 * Now we have segments [0,2] + [2,8] + [8,10] which cover the sporting event
 * [0, 10].
 *
 *
 * Example 2:
 *
 *
 * Input: clips = [[0,1],[1,2]], T = 5
 * Output: -1
 * Explanation:
 * We can't cover [0,5] with only [0,1] and [0,2].
 *
 *
 * Example 3:
 *
 *
 * Input: clips =
 * [[0,1],[6,8],[0,2],[5,6],[0,4],[0,3],[6,7],[1,3],[4,7],[1,4],[2,5],[2,6],[3,4],[4,5],[5,7],[6,9]],
 * T = 9
 * Output: 3
 * Explanation:
 * We can take clips [0,4], [4,7], and [6,9].
 *
 *
 * Example 4:
 *
 *
 * Input: clips = [[0,4],[2,8]], T = 5
 * Output: 2
 * Explanation:
 * Notice you can have extra video after the event ends.
 *
 *
 *
 *
 * Note:
 *
 *
 * 1 <= clips.length <= 100
 * 0 <= clips[i][0], clips[i][1] <= 100
 * 0 <= T <= 100
 *
 *
 */

pub struct Solution;

// @lc code=start
///思路，先找到起点和终点，然后再根据起点的终点和终点的起点找新的,优先选择范围大的
///需要额外注意的是，足够就够，也就是可能同时存在两个最优解，在同一个循环中被找到，这时也需要进行处理
impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        use std::collections::HashSet;
        let mut result = HashSet::new();
        let mut find = false;
        let start = 0;
        let end = t;
        fn find_be(
            mut start: i32,
            mut end: i32,
            find: &mut bool,
            result: &mut HashSet<usize>,
            clips: &Vec<Vec<i32>>,
        ) {
            if start >= end {
                *find = true;
                return;
            }
            let mut begin_end = start;
            let mut end_begin = end;
            let mut target_begin: i32 = -1;
            let mut target_end: i32 = -1;

            let start_rec = start;
            let end_rec = end;
            clips.iter().enumerate().for_each(|(i, clip)| {
                let cb = clip[0];
                let ce = clip[1];
                if cb <= start_rec && ce > begin_end {
                    if begin_end >= end_rec {
                        *find = true;
                        result.insert(i);
                    } else {
                        begin_end = ce;
                        start = ce;
                        target_begin = i as i32;
                    }
                }
                if ce >= end_rec && cb < end_begin {
                    if end_begin <= start_rec {
                        *find = true;
                        result.insert(i);
                    } else {
                        end_begin = cb;
                        end = cb;
                        target_end = i as i32;
                    }
                }
            });
            if *find {
                return;
            }
            if target_begin == -1 || target_end == -1 {
                *find = false;
                return;
            }
            result.insert(target_begin as usize);
            result.insert(target_end as usize);
            find_be(start, end, find, result, clips);
        }
        find_be(start, end, &mut find, &mut result, &clips);
        if !find {
            -1
        } else {
            result.len() as i32
        }
    }
}

// @lc code=end
pub fn main() {
    let test = vec![
        vec![0, 1],
        vec![6, 8],
        vec![0, 2],
        vec![5, 6],
        vec![0, 4],
        vec![0, 3],
        vec![6, 7],
        vec![1, 3],
        vec![4, 7],
        vec![1, 4],
        vec![2, 5],
        vec![2, 6],
        vec![3, 4],
        vec![4, 5],
        vec![5, 7],
        vec![6, 9],
    ];
    Solution::video_stitching(test, 9);
}

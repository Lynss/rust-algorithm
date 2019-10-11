///https://leetcode.com/problems/max-chunks-to-make-sorted/
/// 思路，前n块必须要有前n小的，然后依次的划分，确保每一块 [x..end]都有前end小的
#[allow(dead_code)]
pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut end = i + 1;
    let len = arr.len();
    let mut result = 0;
    while end <= len {
        let chunks = Vec::from(&arr[i..end]);
        let max = chunks.iter().max().unwrap().to_owned() as usize;
        //有前end小
        if max < end {
            i = end;
            end = i + 1;
            result += 1;
            continue;
        }
        end = max + 1;
    }
    result
}

#[test]
fn test_max_chunks_to_sorted() {
    assert_eq!(max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4)
}

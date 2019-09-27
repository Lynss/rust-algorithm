///https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero/
/// 思路：即要求转为2进制后存在每一位都有某索引对应位数为0
#[allow(dead_code)]
pub fn count_triplets(a: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in &a {
        for j in &a {
            for k in &a {
                if i & j & k == 0 {
                    result += 1;
                }
            }
        }
    }
    result
}

#[test]
fn test_count_triplets() {
    assert_eq!(count_triplets(vec![2, 1, 3]), 12)
}

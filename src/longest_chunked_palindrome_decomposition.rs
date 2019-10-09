///https://leetcode.com/problems/longest-chunked-palindrome-decomposition/
///思路：首尾两个指针同时进行，找到一样的就说明多了一个回文块
#[allow(dead_code)]
pub fn longest_decomposition(text: String) -> i32 {
    let mut result = 0;
    let mut begin = 0;
    let mut end = text.len();
    let mut i = 1;
    while begin + i <= end - i {
        if &text[begin..begin + i] == &text[(end - i)..end] {
            result += 2;
            begin = begin + i;
            end = end - i;
            i = 1;
            continue;
        }
        i += 1;
    }
    if begin == end {
        result
    } else {
        result + 1
    }
}

#[test]
fn test_longest_decomposition() {
    assert_eq!(
        longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".to_owned()),
        7
    );
    assert_eq!(longest_decomposition("aa".to_owned()), 2);
    assert_eq!(longest_decomposition("elvtoelvto".to_owned()), 2)
}

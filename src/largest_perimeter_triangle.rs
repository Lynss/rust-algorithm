///https://leetcode.com/problems/largest-perimeter-triangle/
///思路：三角形中最长的那条边要小于等于其他两边之和
#[allow(dead_code)]
pub fn largest_perimeter(a: Vec<i32>) -> i32 {
    let mut b = a;
    b.sort_by(|i, j| j.cmp(i));
    let mut perimeter = 0;
    let mut i = 0;
    let len = b.len();
    while len - i >= 3 {
        if b[i] < b[i + 1] + b[i + 2] {
            perimeter = b[i] + b[i + 1] + b[i + 2];
            break;
        }
        i += 1;
    }
    perimeter
}

#[test]
fn test_largest_perimeter() {
    assert_eq!(largest_perimeter(vec![3, 6, 2, 3]), 8);
}

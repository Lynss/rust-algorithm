pub fn broken_calc(mut x: i32, mut y: i32) -> i32 {
    if x >= y {
        x - y
    } else {
        1 + y % 2 + broken_calc(x, (y + 1) / 2)
    }
}

#[test]
pub fn test_broken_calc() {
    assert_eq!(2, broken_calc(2, 3));
    assert_eq!(2, broken_calc(5, 8));
    assert_eq!(2, broken_calc(5, 9));
    assert_eq!(5, broken_calc(5, 11));
}

///https://leetcode.com/problems/reaching-points/
///思路:从终点往起点找，知道又一个相等，观察另一个是否有可能相等
#[allow(dead_code)]
pub fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
    while sx < tx && sy < ty {
        if tx > ty {
            tx %= ty
        } else {
            ty %= tx
        }
    }
    (sx == tx && ty > sy && (ty - sy) % tx == 0) || (sy == ty && tx > sx && (tx - sx) % ty == 0)
}

#[test]
fn test_reaching_points() {
    assert!(reaching_points(1, 2, 3, 5));
}

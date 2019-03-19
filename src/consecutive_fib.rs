pub fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut x = 0;
    let mut y = 1;
    while prod > x * y {
        y = x + y;
        x = y - x;
    }
    (x, y, x * y == prod)
}

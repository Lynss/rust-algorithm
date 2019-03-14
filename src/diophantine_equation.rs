pub fn solequa(n: u64) -> Vec<(u64, u64)> {
    let mut result = vec![];
    for i in 1..((n as f64).sqrt() as u64 + 1) {
        if n % i == 0 && (i + n / i) % 2 == 0 && (n / i -i) % 4 == 0 {
            result.push(((i + n / i) / 2, ( n / i-i) / 2))
        }
    }
    result
}

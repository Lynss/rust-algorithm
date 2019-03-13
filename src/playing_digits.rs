pub fn dig_pow(n: i64, p: i32) -> i64 {
    let m = n.to_string();
    match m.char_indices().fold(0_i64, |acc, (c, i)| {
        let i = i.to_string().parse::<i64>().unwrap();
        acc + i.pow(p as u32 + c as u32)
    }) {
        d  if d  % n == 0 => d  / n,
        _ => -1
    }
}

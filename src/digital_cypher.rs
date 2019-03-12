fn encode(msg: String, n: i32) -> Vec<i32> {
    msg.chars()
        .zip(n.to_string().chars().cycle())
        .map(|(a, b)| a as i32 + b as i32 - 'a' as i32 - '1' as i32 + 2)
        .collect()
}

pub fn count_sheep(n: u32) -> String {
    (1..n + 1).map(|i| format!("{} sheep...", i)).collect()
}
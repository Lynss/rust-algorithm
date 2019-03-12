fn xo(string: &'static str) -> bool {
    string.chars().fold(0, |acc, c| {
        match c.to_lowercase().to_string().as_str() {
            "o" => acc + 1,
            "x" => acc - 1,
            _ => acc
        }
    }) == 0
}
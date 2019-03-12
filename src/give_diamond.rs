pub fn print(n: i32) -> Option<String> {
    match n {
        neg if neg < 0 => None,
        even if even % 2 == 0 => None,
        pos => {
            let mut n = pos as usize;
            let mut i = 0;
            let mut result = "*".repeat(n) + "\n";
            while n > 1 {
                i += 1;
                n -=  2;
                let line = " ".repeat(i) + "*".repeat(n).as_str() + "\n";
                result.insert_str(0, line.as_str());
                result.push_str(line.as_str());
            }
            Some(result)
        }
    }
}
fn trotter(n: i32) -> i32 {
    match n {
        0 => -1,
        p => {
            let mut target = [false; 10];
            let last = (1..)
                .find(|i| {
                    (i * p).to_string().chars().for_each(|c| {
                        let c = c.to_string().parse::<usize>().unwrap();
                        target[c] = true;
                    });
                    target.iter().all(|i| *i)
                })
                .unwrap_or(0);
            last * p
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(trotter(1692), 5076);
        assert_eq!(trotter(2), 90);
        assert_eq!(trotter(7), 70);
    }
}

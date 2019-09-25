#[allow(dead_code)]
pub fn count_arrangement(n: i32) -> i32 {
    let mut used = vec![false; (n + 1) as usize];
    pub fn dfs(used: &mut Vec<bool>, pos: usize) -> i32 {
        let mut count = 0;
        let n = used.len() - 1;
        if pos > n {
            count = 1;
        } else {
            (1..n + 1).for_each(|i| {
                if !used[i] && (i % pos == 0 || pos % i == 0) {
                    used[i] = true;
                    count += dfs(used, pos + 1);
                    used[i] = false;
                }
            });
        }
        count
    }
    dfs(&mut used, 1)
}

#[test]
pub fn test_count_arrangement() {
    assert_eq!(2, count_arrangement(2));
    assert_eq!(36, count_arrangement(6));
}

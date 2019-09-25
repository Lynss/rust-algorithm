#[allow(dead_code)]
pub fn num_tile_possibilities(tiles: String) -> i32 {
    let mut counter = [0; 26];
    tiles.chars().for_each(|c| {
        counter[c as usize - 'A' as usize] += 1;
    });
    fn dfs(counter: &mut [i32]) -> i32 {
        let mut sum = 0;
        (0..26).for_each(|i| {
            let n = counter[i];
            if n != 0 {
                sum += 1;
                counter[i] -= 1;
                sum += dfs(counter);
                counter[i] += 1;
            }
        });
        sum
    };
    dfs(&mut counter)
}

#[test]
pub fn test_num_tile_possibilities() {
    assert_eq!(8, num_tile_possibilities(String::from("AAB")));
    assert_eq!(188, num_tile_possibilities(String::from("AAABBC")));
}

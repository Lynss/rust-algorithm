pub fn min_distance(word1: String, word2: String) -> i32 {
    use std::cmp;
    let word1 = word1.chars().collect::<Vec<_>>();
    let word2 = word2.chars().collect::<Vec<_>>();
    let m = word1.len();
    let n = word2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..m + 1 {
        dp[i][0] = i
    }
    for j in 0..n + 1 {
        dp[0][j] = j
    }
    for i in 1..m + 1 {
        for j in 1..n + 1 {
            if word1[i-1] == word2[j-1] {
                dp[i][j] = dp[i - 1][j - 1]
            } else {
                dp[i][j] = cmp::min(cmp::min(dp[i - 1][j - 1], dp[i - 1][j]), dp[i][j - 1]) + 1
            }
        }
    }
    dp[m][n] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(
            min_distance("intention".to_owned(), "execution".to_owned()),
            5
        );
    }
}

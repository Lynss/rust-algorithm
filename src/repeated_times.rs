use std::collections::HashMap;

///4 <= A.length <= 10000
///0 <= A[i] < 10000
///A.length is even
pub fn repeated_n_times(a: Vec<i32>) -> i32 {
    let length = a.len();
    let n = length / 2;
    let mut found = -1;
    let mut result_map = HashMap::new();
    for i in a {
        let count = result_map.entry(i).or_insert(0);
        *count += 1;
        if *count == n {
            found = i;
            break;
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated_n_times() {
        assert_eq!(repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}

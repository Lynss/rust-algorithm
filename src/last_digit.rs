//fn last_digit(lst: &[u64]) -> u64 {
//    lst.iter().rev().fold(1, |acc, &x| {
//        if acc == 0 {
//            return 1;
//        }
//        let e: u64 = if acc % 4 ==0 {
//            4
//        } else {
//            acc % 4
//        };
//        (x as f64).powi(e as i32) as u64
//    }) % 10
//}
//
fn last_digit(lst: &[u64]) -> u64 {
    let f = |x, y| std::cmp::min(x % y + y, x);
    lst.iter().rev().fold(1, |v, &n| f(n, 20).pow(f(v, 4) as u32)) % 10
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        let tests = vec![
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6)
        ];

        for test in tests {
            dbg!(&test.0);

            assert_eq!(last_digit(&test.0), test.1);
        }
    }
}
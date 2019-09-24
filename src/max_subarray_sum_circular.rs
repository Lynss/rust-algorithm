pub fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
    use std::cmp;
    let mut total = 0;
    let mut min_sum = 30000;
    let mut cur_min = 0;
    let mut max_sum = -30000;
    let mut cur_max = 0;
    a.iter().for_each(|&i| {
        total += i;
        cur_min = cmp::min(cur_min + i, i);
        min_sum = cmp::min(min_sum, cur_min);
        cur_max = cmp::max(cur_max + i, i);
        max_sum = cmp::max(max_sum, cur_max);
    });
    if max_sum > 0 {
        cmp::max(max_sum, total - min_sum)
    } else {
        max_sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_subarray_sum_circular() {
        assert_eq!(max_subarray_sum_circular(vec![5, -3, 5]), 10);
    }
}

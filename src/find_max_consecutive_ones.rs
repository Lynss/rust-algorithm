pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    use std::cmp;
    let mut max_num = 0;
    let mut counter = 0;
    for current in nums {
        if current == 1{
            counter += 1;
            max_num = cmp::max(max_num, counter);
        }else{
            counter = 0;
        }
    }
    max_num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_max_consecutive_ones() {
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    }
}

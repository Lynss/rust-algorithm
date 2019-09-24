pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    if nums.len() <= 1 {
        result.push(nums);
        return result;
    }
    let mut head = nums;
    let tail = head.split_off(1);
    let nums = permute(tail);
    let head = head[0];
    for num in nums {
        let len = num.len();
        (0..len + 1).for_each(|i| {
            let mut num = num.clone();
            num.insert(i, head);
            result.push(num);
        });
    }
    result
}

#[test]
pub fn test_permute() {
    assert_eq!(permute(vec![1, 2]), vec![vec![1, 2], vec![2, 1]])
}

pub fn find_min(nums: Vec<i32>) -> i32 {
    let l = nums.len();
    let mut before = nums[0];
    if nums[0] >= nums[l - 1] {
        for num in nums {
            if before > num {
                before = num;
                break;
            }
        }
    }
    before
}

#[test]
pub fn test_find_min() {
    assert_eq!(find_min(vec![2, 2, 2, 0, 1]), 0);
    assert_eq!(find_min(vec![1,3,5]), 1);
}

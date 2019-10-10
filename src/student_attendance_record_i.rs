///https://leetcode.com/problems/student-attendance-record-i/
///即字符串不能含有“LLL”或者两个“A”
#[allow(dead_code)]
pub fn check_record(s: String) -> bool {
    let mut warned = 0;
    let mut late_counter = 0;
    s.find(|c| {
        if c == 'L' {
            late_counter += 1;
        } else {
            late_counter = 0;
            if c == 'A' {
                warned += 1;
            }
        }
        late_counter >= 3 || warned >= 2
    })
    .is_none()
}

#[test]
fn test_check_record() {
    assert!(!check_record("PPALLL".to_owned()));
}

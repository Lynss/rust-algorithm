pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
    use std::cmp;
    use std::collections::HashSet;
    let len = fronts.len();
    let mut not = HashSet::new();
    let mut satisifies = vec![];
    (0..len).for_each(|i| {
        let front = fronts[i];
        let back = backs[i];
        if front == back {
            not.insert(front);
        } else {
            satisifies.push((front, back))
        };
    });
    satisifies.iter().fold(0, |acc, &(front, back)| {
        let front = if not.contains(&front) { 2001 } else { front };
        let back = if not.contains(&back) { 2001 } else { back };
        let min = cmp::min(back, front);
        if min == 2001 {
            acc
        } else if acc == 0 {
            min
        } else {
            cmp::min(min, acc)
        }
    })
}

#[test]
fn test_flipgame() {
    assert_eq!(2, flipgame(vec![1, 2, 4, 4, 7], vec![1, 3, 4, 1, 3]));
    assert_eq!(2, flipgame(vec![1, 2], vec![1, 1]));
}

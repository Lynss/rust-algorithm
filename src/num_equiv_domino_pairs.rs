///思路：这里值得注意的是若有三个相等的是3*(3-1)/2对，四个相等的是4*(4-1)/2队
pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
    let mut domino_arr = [0; 100];
    let mut result = 0;
    dominoes.iter().for_each(|domino| {
        let first = domino[0];
        let second = domino[1];
        let index = if first >= second {
            first * 10 + second
        } else {
            second * 10 + first
        };
        result += domino_arr[index as usize];
        domino_arr[index as usize] += 1;
    });
    result
}

#[test]
fn test_num_equiv_domino_pairs() {
    println!("run test_num_equiv_domino");
    assert_eq!(
        num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
        1
    )
}

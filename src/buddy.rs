
use std::collections::HashMap;
fn produce_sum(n:i64)->i64{
    let sqrt = (n as f64).sqrt() as i64 + 1;
    let mut temp_vec: Vec<i64> = vec![];
    (2..sqrt).for_each(|m| {
        let o = n / m;
        if m * o == n {
            temp_vec.push(m);
            if m != o {
                temp_vec.push(o);
            }
        };
    });
    temp_vec.iter().sum::<i64>()
}

fn buddy(start: i64, limit: i64) -> Option<(i64, i64)> {
    let mut dp = HashMap::<i64, i64>::new();
    for n in start..limit + 1 {
        let sum = produce_sum(n);
        if sum <= n {
            continue;
        }
        if !dp.contains_key(&sum) {
            dp.insert(sum, produce_sum(sum));
        }
        if dp[&sum] == n {
            return Some((n, sum));
        }
    }
    None
}
fn dotest(start: i64, limit: i64, exp: Option<(i64, i64)>) -> () {
    println!("start:{}", start);
    println!("limit:{}", limit);
    let ans = buddy(start, limit);
    println!("actual:\n{:?}", ans);
    println!("expect:\n{:?}", exp);
    println!("{}", ans == exp);
    assert_eq!(ans, exp);
    println!("{}", "-");
}

#[test]
fn basic_tests() {
    dotest(5339, 7733,  Some((5775, 6128)));
}

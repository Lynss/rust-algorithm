#[allow(dead_code)]
pub fn fraction_addition(expression: String) -> String {
    use std::collections::VecDeque;
    let mut operators = VecDeque::new();
    operators.push_front('+');
    operators.extend(expression.chars().filter(|&c| c == '-' || c == '+'));
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    };
    expression
        .split(|c| c == '-' || c == '+')
        .fold("0/1".to_owned(), move |acc, next| {
            let temp = acc.split("/").collect::<Vec<_>>();
            let acc_numerator = temp[0];
            let acc_denominator = temp[1];

            let next = if next == "" { "0/1" } else { next };
            let temp = next.split("/").collect::<Vec<_>>();
            let numerator = temp[0];
            let denominator = temp[1];

            let acc_denominator = acc_denominator.to_owned().parse::<i64>().unwrap();
            let denominator = denominator.to_owned().parse::<i64>().unwrap();
            let acc_numerator = acc_numerator.to_owned().parse::<i64>().unwrap();
            let numerator = numerator.to_owned().parse::<i64>().unwrap();

            let a_n = acc_numerator * denominator;
            let n = numerator * acc_denominator;
            let operator = operators.pop_front().unwrap();

            let mut n = if operator == '-' { a_n - n } else { a_n + n };
            let mut d = acc_denominator * denominator;

            let nd_d = gcd(n, d);
            let is_ne = if d / nd_d < 0 { -1 } else { 1 };

            d = d / nd_d * is_ne;
            n = n / nd_d * is_ne;
            format!("{}/{}", n, d)
        })
}

#[test]
fn test_fraction_addition() {
    assert_eq!(
        fraction_addition("-1/2+1/2+1/3".to_owned()),
        "1/3".to_owned()
    );
    assert_eq!(fraction_addition("1/3-1/2".to_owned()), "-1/6".to_owned());
}

#[allow(dead_code)]
pub fn complex_number_multiply(a: String, b: String) -> String {
    let get_kb = |e: String| {
        let e_vec = e.split(|c| c == '+' || c == 'i').collect::<Vec<_>>();
        let k = e_vec[0].parse::<i32>().unwrap();
        let b = e_vec[1].parse::<i32>().unwrap();
        (k, b)
    };
    let (ak, ab) = get_kb(a);
    let (bk, bb) = get_kb(b);
    let ip = (ak * bk) - (ab * bb);
    let ik = ab * bk + bb * ak;
    format!("{}+{}i", ip, ik)
}

#[test]
fn test_complex_number_multiply() {
    assert_eq!(
        complex_number_multiply("1+-1i".to_owned(), "1+-1i".to_owned()),
        "0+-2i"
    )
}

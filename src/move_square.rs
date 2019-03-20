fn diag_1_sym(s: &str) -> String {
    let sc:Vec<_> = s.split_whitespace().collect();
    let n = sc.len();
    let t: Vec<Vec<_>> = sc.iter().map(|s| s.chars().collect()).collect();
    (0..n).map(|i| {
        (0..n).map(|j| {
            t[j][i]
        }).collect::<String>()
    }).collect::<Vec<_>>().join("\n")
}
fn rot_90_clock(s: &str) -> String {
    let sc:Vec<_> = s.split_whitespace().collect();
    let n = sc.len();
    let t: Vec<Vec<_>> = sc.iter().map(|s| s.chars().collect()).collect();
    (0..n).map(|i| {
        (0..n).rev().map(|j| {
            t[j][i]
        }).collect::<String>()
    }).collect::<Vec<_>>().join("\n")
}
fn selfie_and_diag1(s: &str) -> String {
    let sc:Vec<_> = s.split_whitespace().collect();
    let n = sc.len();
    let t: Vec<Vec<_>> = sc.iter().map(|s| s.chars().collect()).collect();
    (0..n).map(|i| {
        format!("{}|{}",t[i].iter().collect::<String>(),(0..n).map(|j| {
            t[j][i]
        }).collect::<String>())
    }).collect::<Vec<_>>().join("\n")
}
fn oper(fct: fn(&str) -> String, s: &str) -> String {
    fct(s)
}

pub fn print_ms_result() {
    let s = "abcd\nefgh\nijkl\nmnop";
    println!("{}", oper(diag_1_sym, s));
    println!("{}", oper(rot_90_clock, s));
    println!("{}", oper(selfie_and_diag1, s));
}

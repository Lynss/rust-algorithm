fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let mut result = 0;
    if new <= old {
        return (result, old + saving - new);
    }
    loop {
        result += 1;
        let d = (saving  * result) as f64
            - increase(result, perc / 100.0) * (new - old) as f64;
        if d >= 0.0 {
            return (result, d.round() as i32);
        }
    }
}

fn increase(month: i32, perc: f64) -> f64 {
    let i = month / 2;
    let k = if month%2 == 0 { 1.0 - perc - 0.005 * i as f64 } else { 1.0 };
    (1..i + 1).fold(
        (1.0 - perc) /k,
        |acc, j| acc * (1.0 - perc - 0.005 * j as f64).powi(2),
    )
}

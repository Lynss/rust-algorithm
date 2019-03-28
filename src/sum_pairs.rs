use std::collections::HashMap;

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut min = std::usize::MAX;
    let mut result: Option<(i8, i8)> = None;
    let mut markers = HashMap::new();
    let mut index: usize = 0;
    for &x in ints {
        if min == 1 {
            break;
        }
        let d = s - x;
        if let Some(&v) = markers.get(&d) {
            if (index - v) < min {
                min = index - v;
                result = Some((d, x));
            }
        }
        *markers.entry(x).or_insert(index) = index;
        index += 1;
    }
    result
}

pub fn print_sum_pairs() {
    let test = [1, 2, 3, 1, 8, 6, 7, 3, 15];
    dbg!(sum_pairs(&test, 8));
}

use std::collections::HashSet;

pub fn push_dominoes(dominoes: String) -> String {
    let len = dominoes.len();
    let mut result = vec![0.0; len];
    let mut markers = HashSet::new();
    dominoes.char_indices().for_each(|(i, c)| {
        if c == 'L' || c == 'R' {
            markers.insert(i);
        }
    });
    for (index, c) in dominoes.char_indices() {
        match c {
            'L' => {
                result[index] = 1.0;
                for i in (0..index).rev() {
                    if markers.contains(&i) {
                        break;
                    };
                    result[i] += 1.0 / (index - i) as f64;
                }
            }
            'R' => {
                result[index] = -1.0;
                for i in (index + 1)..len {
                    if markers.contains(&i) {
                        break;
                    };
                    result[i] -= 1.0 / (i - index) as f64;
                }
            }
            _ => {}
        };
    }
    result
        .iter()
        .map(|item| match item {
            p if *p > 0.0 => 'L',
            p if *p < 0.0 => 'R',
            _ => '.',
        }).collect()
}

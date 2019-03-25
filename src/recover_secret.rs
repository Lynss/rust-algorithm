use std::collections::{HashMap, HashSet};

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut markers: HashMap<char, HashSet<char>> = HashMap::new();
    triplets.iter().for_each(|t| {
        markers
            .entry(t[0])
            .or_insert(HashSet::new())
            .extend(&t[1..3]);
        markers
            .entry(t[1])
            .or_insert(HashSet::new())
            .extend(&t[2..3]);
        markers.entry(t[2]).or_insert(HashSet::new());
    });
    let mut rev_result = Vec::<char>::new();
    while !markers.is_empty() {
        markers.retain(|c, hs| {
            let t = rev_result.iter().cloned().collect();
            *hs = hs.difference(&t).cloned().collect();
            !hs.is_empty() || {
                rev_result.push(c.clone());
                false
            }
        });
    }
    rev_result.iter().rev().cloned().collect()
}

pub fn rs_print() {
    let test = vec![
        ['t', 'u', 'p'],
        ['w', 'h', 'i'],
        ['t', 's', 'u'],
        ['a', 't', 's'],
        ['h', 'a', 'p'],
        ['t', 'i', 's'],
        ['w', 'h', 's'],
    ];
    println!("{}", recover_secret(test));
}

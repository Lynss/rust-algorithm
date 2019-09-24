use std::collections::HashMap;

fn tour(frnds: &[&str], fr_twns: HashMap<&str, &str>, dist: HashMap<&str, f64>) -> i32 {
    let length = fr_twns
        .iter()
        .filter(|(key, _)| frnds.contains(key))
        .collect::<Vec<_>>()
        .len();
    let initial_distance =
        dist[fr_twns[frnds[0]]] + dist.get(fr_twns[frnds[length - 1]]).unwrap_or(&0f64);
    (1..length).fold(initial_distance, |acc, index| {
        let current_town = fr_twns[frnds[index]];
        let direct_distance = dist[current_town];
        let before_town = fr_twns[frnds[index - 1]];
        let before_distance = dist[before_town];
        let distance =
            ((direct_distance * direct_distance - before_distance * before_distance) as f64).sqrt();
        acc + distance
    }) as i32
}

pub fn testing(
    frnds: &[&str],
    fr_twns: HashMap<&str, &str>,
    dist: HashMap<&str, f64>,
    exp: i32,
) -> () {
    assert_eq!(tour(&frnds, fr_twns, dist), exp)
}

#[test]
fn tests_tour() {
    let friends = ["A1", "A2", "A3", "A4"];
    let fr_towns = hash_map! { "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4" };
    let dst = hash_map! { "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0 };
    testing(&friends, fr_towns, dst, 889);
}

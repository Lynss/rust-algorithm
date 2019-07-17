pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    use std::collections::HashMap;
    let mut sort_map = HashMap::new();
    sort_map.insert(' ', 0);
    order.char_indices().for_each(|(i, c)| {
        sort_map.insert(c, i);
    });
    words.iter().is_sorted_by_key(|w| {
        w.chars()
            .map(|ref c| sort_map[c].to_string())
            .collect::<String>()
    })
}

#[test]
pub fn test_is_alien_sorted() {
    assert!(is_alien_sorted(
        vec![String::from("hello"), String::from("leetcode")],
        String::from("hlabcdefgijkmnopqrstuvwxyz")
    ));
    assert!(!is_alien_sorted(
        vec![
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("word"),
            String::from("world")
        ],
        String::from("worldabcefghijkmnpqstuvxyz")
    ));
    assert!(!is_alien_sorted(
        vec![String::from("apple"), String::from("app")],
        String::from("abcdefghijklmnopqrstuvwxyz")
    ));
}

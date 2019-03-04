use std::collections::HashMap;
pub fn first_uniq_char(s: String) -> i32 {
    let mut markers = HashMap::new();
    s.char_indices().for_each(|(i, c)| {
        let c = markers.entry(c).or_insert((0, i));
        (*c).0 += 1;
    });
    *markers
        .iter()
        .filter(|(_, (count, _))| *count == 1)
        .map(|(_, (_, index))| *index as i32)
        .min()
        .get_or_insert(-1)
}

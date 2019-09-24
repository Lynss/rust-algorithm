pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut before = chars[0];
    let mut counter = 1;
    let mut result = vec![before];
    chars.iter().skip(1).for_each(|&c| {
        if c == before {
            counter += 1
        } else {
            if counter > 1 {
                result.extend(counter.to_string().chars())
            }
            before = c;
            counter = 1;
            result.push(before);
        }
    });
    if counter > 1 {
        result.extend(counter.to_string().chars())
    }
    chars.clone_from(&result);
    result.len() as i32
}

#[test]
fn test_compress() {
    let mut a = vec!['a'];
    assert_eq!(1, compress(&mut a));
    let mut b = vec![
        'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
    ];
    assert_eq!(4, compress(&mut b));
    let mut c = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    assert_eq!(6, compress(&mut c));
}

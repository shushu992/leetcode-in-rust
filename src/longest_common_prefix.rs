/**
 * https://leetcode.com/problems/longest-common-prefix/
 */
#[allow(unused)]
fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut breakpoint = strs[0].len();

    for string in &strs[1..] {
        if breakpoint > string.len() {
            breakpoint = string.len();
        }

        let char_indices = &mut string.char_indices().take(breakpoint);

        loop {
            let a = char_indices.next();
            if a.is_none() {
                break;
            }

            let (pos, char) = a.unwrap();

            let b = strs[0].chars().nth(pos);
            if b.is_none() {
                break;
            }

            if b.unwrap() != char {
                breakpoint = pos;
                break;
            }
        };
    }

    strs[0].chars().take(breakpoint).collect::<String>()
}

#[test]
fn test_1() {
    let strs = vec![
        "".to_string(),
    ];
    let expect = "".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

#[test]
fn test_2() {
    let strs = vec![
        "dog".to_string(),
    ];
    let expect = "dog".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

#[test]
fn test_3() {
    let strs = vec![
        "".to_string(),
        "dog".to_string(),
    ];
    let expect = "".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

#[test]
fn test_4() {
    let strs = vec![
        "dog".to_string(),
        "".to_string(),
    ];
    let expect = "".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

#[test]
fn test_5() {
    let strs = vec![
        "flower".to_string(),
        "flower".to_string(),
        "flower".to_string(),
    ];
    let expect = "flower".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

#[test]
fn test_6() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let expect = "fl".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

#[test]
fn test_7() {
    let strs = vec![
        "dog".to_string(),
        "racecar".to_string(),
        "car".to_string(),
    ];
    let expect = "".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

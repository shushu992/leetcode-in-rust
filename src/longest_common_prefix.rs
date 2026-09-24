/**
 * https://leetcode.com/problems/longest-common-prefix/
 */
#[allow(unused)]
fn longest_common_prefix(strs: Vec<String>) -> String {
    // 寻找最短的字符串长度，最终结果不可能大于这个长度
    let min_str_len = strs.iter()
        .map(|x| x.len())
        .min()
        .unwrap();

    let mut i = 0;
    let mut arr_char_indices: Vec<_> = strs.iter()
        .map(|x| x.char_indices())
        .collect();

    'outer: while i < min_str_len {
        // 以第一个str为基准
        let c0 = arr_char_indices[0].next().unwrap();

        // 因此不遍历第一个
        for ci in &mut arr_char_indices[1..] {
            let c1 = ci.next().unwrap();
            if c0.1 != c1.1 {
                break 'outer;
            }
        }

        i += 1;
    }

    strs[0].chars()
        .take(i)
        .collect::<String>()
}

#[test]
fn test_1() {
    let strs = vec!["".to_string()];
    let expect = "".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

#[test]
fn test_2() {
    let strs = vec!["dog".to_string()];
    let expect = "dog".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

#[test]
fn test_3() {
    let strs = vec!["".to_string(), "dog".to_string()];
    let expect = "".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

#[test]
fn test_4() {
    let strs = vec!["dog".to_string(), "".to_string()];
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
    let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    let expect = "".to_string();
    assert_eq!(longest_common_prefix(strs), expect);
}

/**
 * https://leetcode.com/problems/longest-substring-without-repeating-characters/
 *
 * Constraints:
 * 0 <= s.length <= 5 * 10^4
 * s consists of English letters, digits, symbols and spaces.
 */
#[allow(unused)]
fn length_of_longest_substring(s: String) -> i32 {
    let bytes = s.into_bytes();
    let mut arr = Vec::<usize>::with_capacity(bytes.len());

    'pos1: for (pos1, b1) in bytes.iter().enumerate() {
        arr.push(0);

        'pos2: for pos2 in pos1 + 1..=pos1 + 128 {
            match bytes.get(pos2) {
                None => {
                    arr[pos1] = bytes.len();
                    break 'pos2;
                }
                Some(b2) if *b1 == *b2 => {
                    arr[pos1] = pos2;
                    break 'pos2;
                }
                _ => (), // continue
            }
        }
    }

    let mut max = 0;

    'pos1: for pos1 in 0..bytes.len() {
        let span = arr[pos1] - pos1;

        if span <= max {
            continue;
        }

        let mut min = arr[pos1];

        'pos2: for pos2 in pos1 + 1..arr[pos1] {
            if min > arr[pos2] {
                min = arr[pos2];
            }
        }

        if max < min - pos1 {
            max = min - pos1;
        }
    }

    max as i32
}

#[test]
fn test_01() {
    let input = "abcabcbb";
    assert_eq!(length_of_longest_substring(input.to_string()), 3);
}

#[test]
fn test_02() {
    let input = "bbbbb";
    assert_eq!(length_of_longest_substring(input.to_string()), 1);
}

#[test]
fn test_03() {
    // pwwkew
    // 012345
    // 625666
    let input = "pwwkew";
    assert_eq!(length_of_longest_substring(input.to_string()), 3);
}

#[test]
fn test_04() {
    // cdd
    // 012
    // 323
    let input = "cdd";
    assert_eq!(length_of_longest_substring(input.to_string()), 2);
}

#[test]
fn test_05() {
    // cddd
    // 0123
    // 4234
    let input = "cdd";
    assert_eq!(length_of_longest_substring(input.to_string()), 2);
}

#[test]
fn test_06() {
    let input = "abcdedcba";
    assert_eq!(length_of_longest_substring(input.to_string()), 5);
}

#[test]
fn test_07() {
    let input = "abcdeedcba";
    assert_eq!(length_of_longest_substring(input.to_string()), 5);
}

#[test]
fn test_08() {
    let input = "abcb";
    assert_eq!(length_of_longest_substring(input.to_string()), 3);
}

#[test]
fn test_10() {
    let input = "";
    assert_eq!(length_of_longest_substring(input.to_string()), 0);
}

#[test]
fn test_11() {
    let input = "a";
    assert_eq!(length_of_longest_substring(input.to_string()), 1);
}

#[test]
fn test_12() {
    let input = "aa";
    assert_eq!(length_of_longest_substring(input.to_string()), 1);
}

#[test]
fn test_13() {
    let input = "ab";
    // ab
    // 01
    assert_eq!(length_of_longest_substring(input.to_string()), 2);
}

#[test]
fn test_14() {
    let input = "aba";
    assert_eq!(length_of_longest_substring(input.to_string()), 2);
}

#[test]
fn test_15() {
    let input = "abc";
    // abc
    // 012
    assert_eq!(length_of_longest_substring(input.to_string()), 3);
}

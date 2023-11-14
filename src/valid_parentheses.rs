/**
 * https://leetcode.com/problems/valid-parentheses/
 */
#[allow(unused)]
fn is_valid(s: String) -> bool {
    let mut stack = Vec::<char>::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '(' => stack.push('('),
            '{' => stack.push('{'),
            '[' => stack.push('['),
            ')' => match stack.pop() {
                Some('(') => (),
                _ => return false,
            },
            '}' => match stack.pop() {
                Some('{') => (),
                _ => return false,
            },
            ']' => match stack.pop() {
                Some('[') => (),
                _ => return false,
            },
            _ => return false,
        };
    }

    stack.is_empty()
}

#[test]
fn test_1() {
    assert!(is_valid("".to_string()));
}

#[test]
fn test_2() {
    assert!(is_valid("()".to_string()));
}

#[test]
fn test_3() {
    assert!(is_valid("()[]".to_string()));
}

#[test]
fn test_4() {
    assert!(is_valid("()[]{}".to_string()));
}

#[test]
fn test_5() {
    assert!(!is_valid("(".to_string()));
}

#[test]
fn test_6() {
    assert!(!is_valid("}".to_string()));
}

#[test]
fn test7() {
    assert!(!is_valid("(]".to_string()));
}

#[test]
fn test_8() {
    assert!(!is_valid("([){]}".to_string()));
}

#[test]
fn test_9() {
    assert!(!is_valid("()[]{".to_string()));
}

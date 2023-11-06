/**
 * https://leetcode.com/problems/generate-parentheses/
 *
 * Constraints:
 * 1 <= n <= 8
 */
#[allow(unused)]
fn generate_parenthesis(n: i32) -> Vec<String> {
    vec![]
}

fn add_a_parenthesis(s: String) -> Vec<String> {
    vec![]
}

#[test]
fn test_1() {
    let n = 1;
    let expected = vec!["()"];
    assert_eq!(generate_parenthesis(n), expected);
}

#[test]
fn test_2() {
    let n = 2;
    let expected = vec!["()", "()()", "(())"];
    assert_eq!(generate_parenthesis(n), expected);
}

#[test]
fn test_3() {
    let n = 3;
    let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
    assert_eq!(generate_parenthesis(n), expected);
}
/**
 * https://leetcode.com/problems/container-with-most-water/
 *
 * Constraints:
 * n == height.length
 * 2 <= n <= 10^5
 * 0 <= height[i] <= 10^4
 *
 * The algorithm blow is sadly wrong :(
 */
#[allow(unused)]
fn max_area(height: Vec<i32>) -> i32 {
    // todo
    0
}

#[test]
fn test_1() {
    let input = vec![0, 0];
    assert_eq!(max_area(input), 0);
}

#[test]
fn test_2() {
    let input = vec![1, 1];
    assert_eq!(max_area(input), 1);
}

#[test]
fn test_3() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_area(input), 49);
}

#[test]
fn test_4() {
    let input = vec![0, 1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_area(input), 49);
}

#[test]
fn test_5() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 0];
    assert_eq!(max_area(input), 49);
}

#[test]
fn test_6() {
    let input = vec![0, 1, 8, 6, 2, 5, 4, 8, 3, 7, 0];
    assert_eq!(max_area(input), 49);
}

#[test]
fn test_7() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 0, 7];
    assert_eq!(max_area(input), 56);
}

#[test]
fn test_8() {
    let input = vec![2, 3, 4, 5, 18, 17, 6];
    assert_eq!(max_area(input), 17);
}

#[test]
fn test_9() {
    let input = vec![1, 2, 1];
    assert_eq!(max_area(input), 2);
}

#[test]
fn test_10() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 25, 7];
    assert_eq!(max_area(input), 49); // Output: 48
}

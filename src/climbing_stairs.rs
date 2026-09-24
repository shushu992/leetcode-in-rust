/**
 * https://leetcode.com/problems/climbing-stairs/
 *
 * Constraints:
 * 1 <= n <= 45
 */
#[allow(unused)]
fn climb_stairs(n: u8) -> u32 {
    let mut a = 1;
    let mut b = 1;

    for _ in 2..=n {
        (a, b) = (b, a + b);
    }

    b
}

#[test]
fn test_1() {
    // 1
    assert_eq!(climb_stairs(1), 1);
}

#[test]
fn test_2() {
    // 1 + 1
    // 2
    assert_eq!(climb_stairs(2), 2);
}

#[test]
fn test_3() {
    // 1 + 1
    // 1 + 2
    // 2 + 1
    assert_eq!(climb_stairs(3), 3);
}

#[test]
fn test_4() {
    // 1 + 1 + 1 + 1
    // 1 + 1 + 2
    // 1 + 2 + 1
    // 2 + 1 + 1
    // 2 + 2
    assert_eq!(climb_stairs(4), 5);
}

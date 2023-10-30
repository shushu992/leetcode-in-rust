/**
 * https://leetcode.com/problems/climbing-stairs/
 *
 * Constraints:
 * 1 <= n <= 45
 */
#[allow(unused)]
fn climb_stairs(n: u8) -> u32 {
    let arr = &mut Vec::<u32>::with_capacity(45);

    arr.push(0);
    arr.push(1);
    arr.push(2);

    for i in 3..=n {
        let sum = arr[(i - 1) as usize] + arr[(i - 2) as usize];
        arr.push(sum);
    }

    arr[n as usize]
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

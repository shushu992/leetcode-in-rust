/**
 * https://leetcode.com/problems/single-number/
 */
#[allow(unused)]
fn single_number(nums: Vec<i32>) -> i32 {
    let mut sum = 0_i32;

    for n in nums {
        sum ^= n;
    }

    sum
}

#[test]
fn test_1() {
    assert_eq!(single_number([2, 2, 1].to_vec()), 1);
}

#[test]
fn test_2() {
    assert_eq!(single_number([4, 1, 2, 1, 2].to_vec()), 4);
}

#[test]
fn test_3() {
    assert_eq!(single_number([1].to_vec()), 1);
}

/**
 * https://leetcode.com/problems/majority-element/
 *
 * Constraints:
 * 1 <= nums.length <= 5 * 10^4
 * -10^9 <= nums[i] <= 10^9
 */
#[allow(unused)]
fn majority_element(nums: Vec<i32>) -> i32 {
    let mut n = nums[0];
    let mut c = 1;

    for n1 in &nums[1..] {
        if c == 0 {
            n = *n1;
            c = 1;
        } else if n == *n1 {
            c += 1;
        } else {
            c -= 1;
        }
    }

    n
}

#[test]
fn test_1() {
    assert_eq!(majority_element(vec![2]), 2);
}

#[test]
fn test_2() {
    assert_eq!(majority_element(vec![3, 2, 3]), 3);
}

#[test]
fn test_3() {
    assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}

#[test]
fn test_4() {
    assert_eq!(majority_element(vec![7, 6, 6, 1, 6, 3, 6]), 6);
}

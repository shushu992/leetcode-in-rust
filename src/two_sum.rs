use std::ops::Rem;

/**
 * https://leetcode.com/problems/two-sum/
 *
 * Constraints:
 * 2 <= nums.length <= 10^4
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 */
#[allow(unused)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut buckets: [Vec<(usize, i32)>; 256] = std::array::from_fn(|_| vec![]);

    for (pos1, &num1) in nums.iter().enumerate() {
        let remainder = (num1.rem(256) + 256).rem(256);
        let complement = ((target - remainder).rem(256) + 256).rem(256);

        for &(pos2, num2) in buckets[complement as usize].iter() {
            if num1 + num2 == target {
                return vec![pos1 as i32, pos2 as i32];
            }
        }

        buckets[remainder as usize].push((pos1, num1));
    }

    vec![]
}

#[test]
fn test_1() {
    let mut result = two_sum([2, 7, 11, 15].to_vec(), 9);
    result.sort();

    let mut expect = [0, 1].to_vec();
    expect.sort();

    assert_eq!(result, expect);
}

#[test]
fn test_2() {
    let mut result = two_sum([3, 2, 4].to_vec(), 6);
    result.sort();

    let mut expect = [1, 2].to_vec();
    expect.sort();

    assert_eq!(result, expect);
}

#[test]
fn test_3() {
    let mut result = two_sum([3, 3].to_vec(), 6);
    result.sort();

    let mut expect = [0, 1].to_vec();
    expect.sort();

    assert_eq!(result, expect);
}

#[test]
fn test_4() {
    let mut result = two_sum([-1, -2, -3, -4, -5].to_vec(), -8);
    result.sort();

    let mut expect = [2, 4].to_vec();
    expect.sort();

    assert_eq!(result, expect);
}

#[test]
fn test_5() {
    let mut result = two_sum([-1158379, 3950888].to_vec(), 2792509);
    result.sort();

    let mut expect = [0, 1].to_vec();
    expect.sort();

    assert_eq!(result, expect);
}

#[test]
fn test_6() {
    let mut result = two_sum([-2, 3].to_vec(), 1);
    result.sort();

    let mut expect = [0, 1].to_vec();
    expect.sort();

    assert_eq!(result, expect);
}

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
    assert!(2 <= nums.len() && nums.len() <= 10000);

    let mut buckets: [Vec<(i32, usize)>; 4096] = std::array::from_fn(|_| vec![]);

    for (pos1, &num1) in nums.iter().enumerate() {
        let remainder = num1.rem(4096).unsigned_abs() as usize;

        let complement = target.abs_diff(remainder as i32).rem(4096) as usize;
        for &(num2, pos2) in buckets[complement].iter() {
            if num1 + num2 == target {
                return vec![pos1 as i32, pos2 as i32];
            }
        }

        buckets[remainder].push((num1, pos1));
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

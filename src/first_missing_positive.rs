/**
 * https://leetcode.com/problems/first-missing-positive/
 *
 * Constraints:
 * 1 <= nums.length <= 10^5
 * -2^31 <= nums[i] <= 2^31 - 1
 */
#[allow(unused)]
fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let mut size = nums.len();
    let mut count = 0_usize;

    loop {
        for i in 0..size {
            if 0 < nums[i] && nums[i] <= size as i32 {
                nums[i - count] = nums[i];
            } else {
                count += 1;
            }
        }

        if count == 0 {
            break;
        }

        size -= count;
        count = 0;
    }

    for i in 0..size {
        let seat = nums[i] as usize - 1;
        sit(&mut nums, seat);
    }

    for i in 0..size {
        if nums[i] != i as i32 + 1 {
            return i as i32 + 1;
        }
    }

    (size + 1) as i32
}

fn sit(nums: &mut Vec<i32>, seat: usize) {
    if nums[seat] == (seat + 1) as i32 {
        return;
    }

    let next = nums[seat] - 1;
    nums[seat] = (seat + 1) as i32;
    sit(nums, next as usize);
}

#[test]
fn test_1() { assert_eq!(first_missing_positive(vec![1, 2, 0]), 3); }

#[test]
fn test_2() { assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2); }

#[test]
fn test_3() { assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1); }

#[test]
fn test_10() { assert_eq!(first_missing_positive(vec![1, 2, 3]), 4); }

#[test]
fn test_11() { assert_eq!(first_missing_positive(vec![-1]), 1); }

#[test]
fn test_12() { assert_eq!(first_missing_positive(vec![0]), 1); }

#[test]
fn test_13() { assert_eq!(first_missing_positive(vec![1]), 2); }

#[test]
fn test_14() { assert_eq!(first_missing_positive(vec![2]), 1); }

#[test]
fn test_15() { assert_eq!(first_missing_positive(vec![2, 1]), 3); }

#[test]
fn test_16() { assert_eq!(first_missing_positive(vec![3, 2, 1]), 4); }

#[test]
fn test_17() { assert_eq!(first_missing_positive(vec![1, 2, 4, 5, 7, 8]), 3); }

#[test]
fn test_20() { assert_eq!(first_missing_positive(vec![1, 1]), 2); }

#[test]
fn test_21() { assert_eq!(first_missing_positive(vec![1, 1, 1]), 2); }

#[test]
fn test_22() { assert_eq!(first_missing_positive(vec![1, 1, 2]), 3); }

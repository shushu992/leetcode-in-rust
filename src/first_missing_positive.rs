/**
 * https://leetcode.com/problems/first-missing-positive/
 *
 * Constraints:
 * 1 <= nums.length <= 10^5
 * -2^31 <= nums[i] <= 2^31 - 1
 */
#[allow(unused)]
fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let mut max = nums.len();
    let mut count = 0_usize;

    loop {
        for i in 0..max {
            if 0 < nums[i] && nums[i] <= max as i32 {
                nums[i - count] = nums[i];
            } else {
                count += 1;
            }
        }

        if count == 0 {
            return (max + 1) as i32;
        }

        max -= count;
        count = 0;
    }

    // todo
}

#[test]
fn test_1() {
    assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
}

#[test]
fn test_2() {
    assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
}

#[test]
fn test_3() {
    assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}

#[test]
fn test_4() {
    assert_eq!(first_missing_positive(vec![1, 2, 3]), 4);
}

#[test]
fn test_5() {
    assert_eq!(first_missing_positive(vec![-1]), 1);
}

#[test]
fn test_6() {
    assert_eq!(first_missing_positive(vec![0]), 1);
}

#[test]
fn test_7() {
    assert_eq!(first_missing_positive(vec![1]), 2);
}

#[test]
fn test_8() {
    assert_eq!(first_missing_positive(vec![2]), 1);
}

#[test]
fn test_9() {
    assert_eq!(first_missing_positive(vec![1, 1]), 2);
}

#[test]
fn test_10() {
    assert_eq!(first_missing_positive(vec![1, 1, 1]), 2);
}

#[test]
fn test_11() {
    assert_eq!(first_missing_positive(vec![2, 1]), 3);
}

/**
 * https://leetcode.com/problems/first-missing-positive/
 *
 * Constraints:
 * 1 <= nums.length <= 10^5
 * -2^31 <= nums[i] <= 2^31 - 1
 */
#[allow(unused)]
fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    // 数字最大只能是这个
    let mut max_num: i32 = nums.len() as i32 - 1;

    // 遍历数组, 原地排序
    for i in 0..nums.len() {
        let n = nums[i];
        // 将数字n，放置到正确的位置n-1
        take_seat(&mut nums, n);
    }

    // 再次遍历，寻找结果
    for i in 0..nums.len() {
        if i as i32 + 1 != nums[i] {
            return i as i32 + 1;
        }
    }

    nums.len() as i32 + 1
}

/**
 * 将指定的数字n，放到它正确的位置n-1
 */
fn take_seat(nums: &mut Vec<i32>, n: i32) {
    // 如果数字超出范围，直接跳过
    if n < 1 || n > nums.len() as i32 {
        return;
    }

    // 保存该位置原来的数字
    let n1 = nums[n as usize - 1];

    // 如果该位置的数字和位置刚好对应
    if n1 == n {
        return;
    }

    // 先处理自己
    nums[n as usize - 1] = n;

    // 再处理原来的数字
    take_seat(nums, n1);
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
fn test_10() {
    assert_eq!(first_missing_positive(vec![1, 2, 3]), 4);
}

#[test]
fn test_11() {
    assert_eq!(first_missing_positive(vec![-1]), 1);
}

#[test]
fn test_12() {
    assert_eq!(first_missing_positive(vec![0]), 1);
}

#[test]
fn test_13() {
    assert_eq!(first_missing_positive(vec![1]), 2);
}

#[test]
fn test_14() {
    assert_eq!(first_missing_positive(vec![2]), 1);
}

#[test]
fn test_15() {
    assert_eq!(first_missing_positive(vec![2, 1]), 3);
}

#[test]
fn test_16() {
    assert_eq!(first_missing_positive(vec![3, 2, 1]), 4);
}

#[test]
fn test_17() {
    assert_eq!(first_missing_positive(vec![1, 2, 4, 5, 7, 8]), 3);
}

#[test]
fn test_20() {
    assert_eq!(first_missing_positive(vec![1, 1]), 2);
}

#[test]
fn test_21() {
    assert_eq!(first_missing_positive(vec![1, 1, 1]), 2);
}

#[test]
fn test_22() {
    assert_eq!(first_missing_positive(vec![1, 1, 2]), 3);
}

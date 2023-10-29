/**
 * https://leetcode.com/problems/move-zeroes/
 */
fn move_zeroes(nums: &mut Vec<i32>) {
    let mut zero_count = 0;
    let len = nums.len();

    for i in 0..len {
        if nums[i] == 0 {
            zero_count += 1;
        } else if zero_count != 0 {
            nums[i - zero_count] = nums[i];
        }
    }

    for n in &mut nums[len - zero_count..len] {
        *n = 0;
    }
}

#[test]
fn test_1() {
    let mut input = vec![0];
    let expect = vec![0];

    move_zeroes(&mut input);
    assert_eq!(input, expect);
}

#[test]
fn test_2() {
    let mut input = vec![1];
    let expect = vec![1];

    move_zeroes(&mut input);
    assert_eq!(input, expect);
}

#[test]
fn test_3() {
    let mut input = vec![0, 1, 0, 3, 12];
    let expect = vec![1, 3, 12, 0, 0];

    move_zeroes(&mut input);
    assert_eq!(input, expect);
}

#[test]
fn test_4() {
    let mut input = vec![0, -1, 0, -3, -12];
    let expect = vec![-1, -3, -12, 0, 0];

    move_zeroes(&mut input);
    assert_eq!(input, expect);
}

#[test]
fn test_5() {
    let mut input = vec![0, 1];
    let expect = vec![1, 0];

    move_zeroes(&mut input);
    assert_eq!(input, expect);
}

#[test]
fn test_6() {
    let mut input = vec![1, 0];
    let expect = vec![1, 0];

    move_zeroes(&mut input);
    assert_eq!(input, expect);
}
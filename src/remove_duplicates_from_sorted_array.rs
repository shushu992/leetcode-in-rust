/**
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array/
 *
 * Constraints:
 * 1 <= nums.length <= 3 * 10^4
 * -100 <= nums[i] <= 100
 * nums is sorted in non-decreasing order.
 */
#[allow(unused)]
fn remove_duplicates(nums: &mut Vec<u16>) -> usize {
    let mut count_duplicates = 0_usize;

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            count_duplicates += 1;
        }

        nums[i - count_duplicates] = nums[i];
    }

    nums.len() - count_duplicates
}

#[test]
fn test_1() {
    let mut nums = vec![1, 1, 2];
    let expected = [1, 2];

    remove_duplicates(&mut nums);
    assert_eq!(nums[0..expected.len()], expected);
}

#[test]
fn test_2() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let expected = [0, 1, 2, 3, 4];

    remove_duplicates(&mut nums);
    assert_eq!(nums[0..expected.len()], expected);
}

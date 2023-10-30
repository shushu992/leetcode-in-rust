/**
 * https://leetcode.com/problems/binary-search/
 *
 * Constraints:
 * 1 <= nums.length <= 10^4
 * -10^4 < nums[i], target < 10^4
 * All the integers in nums are unique.
 * nums is sorted in ascending order.
 */
#[allow(unused)]
fn search(nums: Vec<i32>, target: i32) -> i32 {
    search_sub(&nums, target, 0, nums.len()) as i32
}

fn search_sub(nums: &[i32], target: i32, from: usize, to: usize) -> isize {
    if from == to - 1 {
        if nums[from] == target {
            return from as isize;
        }
        return -1;
    }

    if (to - from) % 2 == 0 {
        let l = search_sub(nums, target, from, from + (to - from) / 2);
        if l >= 0 {
            return l;
        }

        let r = search_sub(nums, target, from + (to - from) / 2, to);
        if r >= 0 {
            return r;
        }
    } else {
        let l = search_sub(nums, target, from, from + (to - from + 1) / 2);
        if l >= 0 {
            return l;
        }

        let r = search_sub(nums, target, from + (to - from + 1) / 2, to);
        if r >= 0 {
            return r;
        }
    }

    -1
}


#[test]
fn test_1() {
    assert_eq!(search(vec![5], 5), 0);
}

#[test]
fn test_2() {
    assert_eq!(search(vec![5], 10), -1);
}

#[test]
fn test_3() {
    assert_eq!(search(vec![1, 5], 5), 1);
}

#[test]
fn test_4() {
    assert_eq!(search(vec![1, 5], 10), -1);
}

#[test]
fn test_5() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
}

#[test]
fn test_6() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}
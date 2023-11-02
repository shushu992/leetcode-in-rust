use std::cmp::min;

/**
 * https://leetcode.com/problems/container-with-most-water/
 *
 * Constraints:
 * n == height.length
 * 2 <= n <= 10^5
 * 0 <= height[i] <= 10^4
 *
 * The algorithm blow is sadly wrong :(
 */
#[allow(unused)]
fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0_usize;
    let mut r = height.len() - 1;

    loop {
        if l == r {
            return 0;
        }

        if height[l] > 0 {
            break;
        }

        l += 1;
    }

    loop {
        if r == l {
            return 0;
        }

        if height[r] > 0 {
            break;
        }

        r -= 1;
    }

    loop {
        let mut mutted = false;

        for l1 in l + 1..r {
            if height[l1] * (r as i32 - l1 as i32) > height[l] * (r as i32 - l as i32) {
                l = l1;
                mutted = true;
            }
        }

        for r1 in l + 1..r {
            if height[r1] * (r1 as i32 - l as i32) > height[r] * (r as i32 - l as i32) {
                r = r1;
                mutted = true;
            }
        }

        if !mutted {
            break;
        }
    }

    min(height[l], height[r]) * (r as i32 - l as i32)
}

#[test]
fn test_1() {
    let input = vec![0, 0];
    assert_eq!(max_area(input), 0);
}

#[test]
fn test_2() {
    let input = vec![1, 1];
    assert_eq!(max_area(input), 1);
}

#[test]
fn test_3() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_area(input), 49);
}

#[test]
fn test_4() {
    let input = vec![0, 1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_area(input), 49);
}

#[test]
fn test_5() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 0];
    assert_eq!(max_area(input), 49);
}

#[test]
fn test_6() {
    let input = vec![0, 1, 8, 6, 2, 5, 4, 8, 3, 7, 0];
    assert_eq!(max_area(input), 49);
}

#[test]
fn test_7() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 0, 7];
    assert_eq!(max_area(input), 56);
}

#[test]
fn test_8() {
    let input = vec![2, 3, 4, 5, 18, 17, 6];
    assert_eq!(max_area(input), 17);
}

#[test]
fn test_9() {
    let input = vec![1, 2, 1];
    assert_eq!(max_area(input), 2);
}

#[test]
fn test_10() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 25, 7];
    assert_eq!(max_area(input), 49); // Output: 48
}

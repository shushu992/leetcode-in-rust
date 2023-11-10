/**
 * https://leetcode.com/problems/rotate-image/
 *
 * Constraints:
 * n == matrix.length == matrix[i].length
 * 1 <= n <= 20
 * -1000 <= matrix[i][j] <= 1000
 */
#[allow(unused)]
fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();

    for x in 0..(n + 1) / 2 {
        for y in 0..(n + 1) / 2 {
            let mut tmp = matrix[x][y];
            // todo
        }
    }
}

#[test]
fn test_1() {
    let mut input = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    rotate(&mut input);
    assert_eq!(input, vec![
        vec![7, 4, 1],
        vec![8, 5, 2],
        vec![9, 6, 3],
    ]);
}

#[test]
fn test_2() {
    let mut input = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    rotate(&mut input);
    assert_eq!(input, vec![
        vec![15, 13, 2, 5],
        vec![14, 3, 4, 1],
        vec![12, 6, 8, 9],
        vec![16, 7, 10, 11],
    ]);
}
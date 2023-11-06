/**
 * https://leetcode.com/problems/roman-to-integer/
 *
 * Constraints:
 * 1 <= s.length <= 15
 * s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
 * It is guaranteed that s is a valid roman numeral in the range [1, 3999].
 */
#[allow(unused)]
fn roman_to_int(s: String) -> i32 {
    let bytes = s.into_bytes();
    let mut sum = 0;
    let mut i = 0;

    loop {
        match bytes.get(i) {
            Some(b'I') => match bytes.get(i + 1) {
                Some(b'V') => {
                    sum += 4;
                    i += 1;
                }
                Some(b'X') => {
                    sum += 9;
                    i += 1;
                }
                _ => sum += 1,
            },
            Some(b'V') => sum += 5,
            Some(b'X') => match bytes.get(i + 1) {
                Some(b'L') => {
                    sum += 40;
                    i += 1;
                }
                Some(b'C') => {
                    sum += 90;
                    i += 1;
                }
                _ => sum += 10,
            },
            Some(b'L') => sum += 50,
            Some(b'C') => match bytes.get(i + 1) {
                Some(b'D') => {
                    sum += 400;
                    i += 1;
                }
                Some(b'M') => {
                    sum += 900;
                    i += 1;
                }
                _ => sum += 100,
            },
            Some(b'D') => sum += 500,
            Some(b'M') => sum += 1000,
            _ => break,
        }

        i += 1;
    }

    sum
}

#[test]
fn test_1() {
    let s = "III";
    let expected = 3;
    assert_eq!(roman_to_int(s.to_string()), expected);
}

#[test]
fn test_2() {
    let s = "IV";
    let expected = 4;
    assert_eq!(roman_to_int(s.to_string()), expected);
}

#[test]
fn test_3() {
    let s = "LVIII";
    let expected = 58;
    assert_eq!(roman_to_int(s.to_string()), expected);
}

#[test]
fn test_4() {
    let s = "MCMXCIV";
    let expected = 1994;
    assert_eq!(roman_to_int(s.to_string()), expected);
}
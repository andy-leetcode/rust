pub struct SolutionUsingStr;

impl SolutionUsingStr {
    pub fn is_palindrome(x: i32) -> bool {
        let reversed_str = String::from_utf8(
            x.clone()
                .to_string()
                .chars()
                .rev()
                .map(|x| x as u8)
                .collect::<Vec<u8>>(),
        )
        .unwrap();
        (0..10).contains(&x) || (x >= 0 && reversed_str.parse::<i32>().unwrap() == x)
    }
}

/// Return the order in decimal units (eg 1B = 10^9, so return 9)
pub fn get_order(x: i32) -> i32 {
    (0..10)
        .into_iter()
        .rev()
        .filter(|i| x > 10_u32.pow(i.to_owned()).try_into().unwrap())
        .max()
        .unwrap() as i32
}

/// Return the digit representing the ith order in x (for get_order_digit(123, 3), return 1)
pub fn get_order_digit(x: i32, i: i32) -> i32 {
    let mut x = x;

    if x > 10_i32.pow(i as u32) {
        x -= 10_i32.pow(i as u32);
    }

    (x as f64 / 10_f64.powi(i - 1)) as i32
}

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let order = get_order(x);
        let reversed_digits = (0..=order)
            .into_iter()
            .rev()
            .map(|order| get_order_digit(x, order))
            .rev()
            .collect::<Vec<i32>>();
        let digits = &reversed_digits.into_iter().rev().collect::<Vec<i32>>();

        fn compare_digits(i: usize, d: &i32, digits: &Vec<i32>) -> u8 {
            let digit = digits[i];
            (d == &digit) as u8
        }

        let res = digits
            .into_iter()
            .enumerate()
            .map(|(i, d)| compare_digits(i, d, &reversed_digits))
            .reduce(|acc, x| acc * x)
            .unwrap();

        res == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_is_palindrome(x: i32) {
        assert!(Solution::is_palindrome(x))
    }

    fn make_test_is_not_palindrome(x: i32) {
        assert!(!Solution::is_palindrome(x))
    }

    #[test]
    fn test1() {
        make_test_is_palindrome(1);
    }

    #[test]
    fn test_neg1() {
        make_test_is_not_palindrome(-1);
    }

    #[test]
    fn test11() {
        make_test_is_palindrome(11);
    }

    #[test]
    fn test12() {
        make_test_is_not_palindrome(12);
    }

    #[test]
    fn test55() {
        make_test_is_palindrome(55);
    }

    #[test]
    fn test100() {
        make_test_is_not_palindrome(100);
    }

    #[test]
    fn test101() {
        make_test_is_palindrome(101);
    }

    #[test]
    fn test123454321() {
        make_test_is_palindrome(123454321);
    }

    #[test]
    fn test123455321() {
        make_test_is_not_palindrome(123455321);
    }

    #[test]
    fn test121() {
        make_test_is_palindrome(121);
    }

    #[test]
    fn test_neg121() {
        make_test_is_not_palindrome(-121);
    }

    #[test]
    fn test_10() {
        make_test_is_not_palindrome(10);
    }
}

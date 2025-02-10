// https://leetcode.com/problems/clear-digits/
// 10/02/2025

impl Solution {
    pub fn clear_digits(s: String) -> String {
        s.bytes()
            .fold(String::with_capacity(s.len()), |mut acc, c| {
                match c {
                    b'0'..=b'9' => _ = acc.pop(),
                    _ => acc.push(c as char),
                }

                acc
            })
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abc".to_string(), "abc".to_string())]
    #[case("cb34".to_string(), "".to_string())]
    fn test(#[case] s: String, #[case] expected: String) {
        let result = Solution::clear_digits(s);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ("abc".to_string(), "abc".to_string()),
        ("cb34".to_string(), "".to_string()),
    ];

    for (s, expected) in test_cases {
        let result = Solution::clear_digits(s.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            s, expected, result
        );
        assert_eq!(result, expected);
    }
}

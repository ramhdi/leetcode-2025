// https://leetcode.com/problems/maximum-score-after-splitting-a-string/
// 01/01/2025

#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let bytes = s.as_bytes();
        bytes
            .iter()
            .take(bytes.len() - 1)
            .enumerate()
            .map(|(i, _)| {
                let (left, right) = bytes.split_at(i + 1);
                (left.iter().filter(|&&b| b == b'0').count()
                    + right.iter().filter(|&&b| b == b'1').count()) as i32
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("011101", 5)]
    #[case("00111", 5)]
    #[case("1111", 3)]
    fn test_max_score(#[case] s: String, #[case] expected: i32) {
        assert_eq!(Solution::max_score(s), expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![("011101", 5), ("00111", 5), ("1111", 3)];

    for (input, expected) in test_cases {
        let result = Solution::max_score(input.to_string());
        println!("Input: {}, Expected: {}, Got: {}", input, expected, result);
        assert_eq!(result, expected);
    }
}

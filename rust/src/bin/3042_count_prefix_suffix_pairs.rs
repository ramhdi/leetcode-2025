// https://leetcode.com/problems/count-prefix-and-suffix-pairs-i/
// 08/01/2025

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;
        let n = words.len();

        for j in 1..n {
            for i in 0..j {
                if Self::is_prefix_and_suffix(&words[i], &words[j]) {
                    result += 1;
                }
            }
        }

        result
    }

    fn is_prefix_and_suffix(str1: &String, str2: &String) -> bool {
        str2.starts_with(str1) && str2.ends_with(str1)
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ["a", "aba", "ababa", "aa"].map(|e| e.to_string()).to_vec(),
        4,
    )]
    #[case(
        ["pa", "papa", "ma", "mama"].map(|e| e.to_string()).to_vec(),
        2,
    )]
    #[case(["abab", "ab"].map(|e| e.to_string()).to_vec(), 0)]
    fn test(#[case] words: Vec<String>, #[case] expected: i32) {
        let result = Solution::count_prefix_suffix_pairs(words);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            ["a", "aba", "ababa", "aa"].map(|e| e.to_string()).to_vec(),
            4,
        ),
        (
            ["pa", "papa", "ma", "mama"].map(|e| e.to_string()).to_vec(),
            2,
        ),
        (["abab", "ab"].map(|e| e.to_string()).to_vec(), 0),
    ];

    for (words, expected) in test_cases {
        let result = Solution::count_prefix_suffix_pairs(words.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            words, expected, result
        );
        assert_eq!(result, expected);
    }
}

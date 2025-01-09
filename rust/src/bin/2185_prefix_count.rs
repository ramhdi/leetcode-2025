// https://leetcode.com/problems/counting-words-with-a-given-prefix/
// 09/01/2025

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|&w| w.starts_with(&pref)).count() as i32
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
        ["pay", "attention", "practice", "attend"]
            .map(|e| e.to_string())
            .to_vec(),
        "at".to_string(),
        2,
    )]
    #[case(
        ["leetcode", "win", "loops", "success"]
            .map(|e| e.to_string())
            .to_vec(),
        "code".to_string(),
        0,
    )]
    fn test(#[case] words: Vec<String>, #[case] pref: String, #[case] expected: i32) {
        let result = Solution::prefix_count(words, pref);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            ["pay", "attention", "practice", "attend"]
                .map(|e| e.to_string())
                .to_vec(),
            "at".to_string(),
            2,
        ),
        (
            ["leetcode", "win", "loops", "success"]
                .map(|e| e.to_string())
                .to_vec(),
            "code".to_string(),
            0,
        ),
    ];

    for (words, pref, expected) in test_cases {
        let result = Solution::prefix_count(words.clone(), pref.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (words, pref),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}

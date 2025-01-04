// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/
// 04/01/2025

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let char_start_end = s.iter().enumerate().fold(
            [(None, None); 26],
            |mut acc: [(Option<usize>, Option<usize>); 26], (i, b)| {
                let char_idx = (b - b'a') as usize;
                if acc[char_idx].0.is_none() {
                    acc[char_idx] = (Some(i), None);
                } else {
                    acc[char_idx] = (acc[char_idx].0, Some(i));
                }

                acc
            },
        );

        char_start_end
            .iter()
            .map(|(start, end)| match (start, end) {
                (Some(start), Some(end)) => {
                    let mut char_exists = [0u8; 26];
                    for i in (start + 1)..*end {
                        char_exists[(s[i] - b'a') as usize] = 1;
                    }
                    char_exists.iter().sum::<u8>() as i32
                }
                _ => 0,
            })
            .sum()
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
        "aabca".to_string(),
        3
    )]
    #[case(
        "adc".to_string(),
        0
    )]
    #[case(
        "bbcbaba".to_string(),
        4
    )]
    fn test(#[case] s: String, #[case] expected: i32) {
        let result = Solution::count_palindromic_subsequence(s);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ("aabca".to_string(), 3),
        ("adc".to_string(), 0),
        ("bbcbaba".to_string(), 4),
    ];

    for (s, expected) in test_cases {
        let result = Solution::count_palindromic_subsequence(s.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            s, expected, result
        );
        assert_eq!(result, expected);
    }
}

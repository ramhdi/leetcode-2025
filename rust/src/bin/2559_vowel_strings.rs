// https://leetcode.com/problems/count-vowel-strings-in-ranges/
// 02/01/2025

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
        let prefix_sum: Vec<i32> = words
            .iter()
            .scan(0, |state, w| {
                let bytes = w.as_bytes();
                let (first, last) = (bytes[0], bytes[bytes.len() - 1]);
                *state += (VOWELS.contains(&first) && VOWELS.contains(&last)) as i32;
                Some(*state)
            })
            .collect();

        queries
            .iter()
            .map(|q| {
                let (start, end) = (q[0] as usize, q[1] as usize);
                if start == 0 {
                    prefix_sum[end]
                } else {
                    prefix_sum[end] - prefix_sum[start - 1]
                }
            })
            .collect()
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
        ["aba", "bcb", "ece", "aa", "e"]
            .map(|s| s.to_string())
            .to_vec(),
        [[0, 2], [1, 4], [1, 1]]
            .map(|q| q.to_vec())
            .to_vec(),
        [2, 3, 0].to_vec(),
    )]
    #[case(
        ["a", "e", "i"]
            .map(|s| s.to_string())
            .to_vec(),
        [[0, 2], [0, 1], [2, 2]]
            .map(|q| q.to_vec())
            .to_vec(),
        [3, 2, 1].to_vec(),
    )]
    fn test_vowel_strings(
        #[case] words: Vec<String>,
        #[case] queries: Vec<Vec<i32>>,
        #[case] expected: Vec<i32>,
    ) {
        let result = Solution::vowel_strings(words, queries);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            ["aba", "bcb", "ece", "aa", "e"]
                .map(|s| s.to_string())
                .to_vec(),
            [[0, 2], [1, 4], [1, 1]].map(|q| q.to_vec()).to_vec(),
            [2, 3, 0].to_vec(),
        ),
        (
            ["a", "e", "i"].map(|s| s.to_string()).to_vec(),
            [[0, 2], [0, 1], [2, 2]].map(|q| q.to_vec()).to_vec(),
            [3, 2, 1].to_vec(),
        ),
    ];

    for (words, queries, expected) in test_cases {
        let result = Solution::vowel_strings(words.clone(), queries.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (words, queries),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}

// https://leetcode.com/problems/word-subsets/
// 10/01/2025

// Time limit exceeded
// impl Solution {
//     pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
//         let words: Vec<[usize; 26]> = words1
//             .iter()
//             .map(|w| {
//                 w.bytes().fold([0; 26], |mut acc, b| {
//                     acc[(b - b'a') as usize] += 1;
//                     acc
//                 })
//             })
//             .collect();

//         let patterns: Vec<[usize; 26]> = words2
//             .iter()
//             .map(|w| {
//                 w.bytes().fold([0; 26], |mut acc, b| {
//                     acc[(b - b'a') as usize] += 1;
//                     acc
//                 })
//             })
//             .collect();

//         words1
//             .iter()
//             .zip(words.iter())
//             .filter_map(|(&ref s1, &s2)| {
//                 if patterns.iter().all(|p| Self::is_subset_of(p, &s2)) {
//                     Some(s1)
//                 } else {
//                     None
//                 }
//             })
//             .cloned()
//             .collect()
//     }

//     fn is_subset_of(pattern: &[usize], word: &[usize]) -> bool {
//         (0..pattern.len())
//             .into_iter()
//             .all(|i| word[i] >= pattern[i])
//     }
// }

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let max_pattern = words2.iter().fold([0; 26], |mut acc, pattern| {
            let pattern = pattern.bytes().fold([0; 26], |mut acc2, b| {
                acc2[(b - b'a') as usize] += 1;
                acc2
            });

            (0..26).for_each(|i| {
                acc[i] = acc[i].max(pattern[i]);
            });

            acc
        });

        words1
            .into_iter()
            .filter(|word| {
                let freq = word.bytes().fold([0; 26], |mut acc, b| {
                    acc[(b - b'a') as usize] += 1;
                    acc
                });

                (0..26).all(|i| freq[i] >= max_pattern[i])
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
        ["amazon", "apple", "facebook", "google", "leetcode"]
            .map(|e| e.to_string())
            .to_vec(),
        ["e", "o"].map(|e| e.to_string()).to_vec(),
        ["facebook", "google", "leetcode"]
            .map(|e| e.to_string())
            .to_vec(),
    )]
    #[case(
        ["amazon", "apple", "facebook", "google", "leetcode"]
            .map(|e| e.to_string())
            .to_vec(),
        ["l", "e"].map(|e| e.to_string()).to_vec(),
        ["apple", "google", "leetcode"]
            .map(|e| e.to_string())
            .to_vec(),
    )]
    fn test(
        #[case] words1: Vec<String>,
        #[case] words2: Vec<String>,
        #[case] expected: Vec<String>,
    ) {
        let result = Solution::word_subsets(words1, words2);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            ["amazon", "apple", "facebook", "google", "leetcode"]
                .map(|e| e.to_string())
                .to_vec(),
            ["e", "o"].map(|e| e.to_string()).to_vec(),
            ["facebook", "google", "leetcode"]
                .map(|e| e.to_string())
                .to_vec(),
        ),
        (
            ["amazon", "apple", "facebook", "google", "leetcode"]
                .map(|e| e.to_string())
                .to_vec(),
            ["l", "e"].map(|e| e.to_string()).to_vec(),
            ["apple", "google", "leetcode"]
                .map(|e| e.to_string())
                .to_vec(),
        ),
    ];

    for (words1, words2, expected) in test_cases {
        let result = Solution::word_subsets(words1.clone(), words2.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (words1, words2),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}

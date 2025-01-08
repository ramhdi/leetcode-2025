// https://leetcode.com/problems/string-matching-in-an-array/
// 07/01/2025

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        words
            .iter()
            .filter(|&w| words.iter().any(|w2| w.len() < w2.len() && w2.contains(w)))
            .cloned()
            .collect()
    }
}

// impl Solution {
//     pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
//         words.sort_unstable_by(|a, b| b.len().cmp(&a.len()));

//         words
//             .iter()
//             .enumerate()
//             .skip(1)
//             .filter_map(|(i, word)| {
//                 words[..i]
//                     .iter()
//                     .any(|w| w.contains(word))
//                     .then(|| word.clone())
//             })
//             .collect()
//     }
// }

// impl Solution {
//     // Compute the longest proper prefix that is also a proper suffix
//     fn compute_lps(pattern: &str) -> Vec<usize> {
//         let chars: Vec<char> = pattern.chars().collect();
//         let n = chars.len();
//         let mut lps = vec![0; n];
//         let mut len = 0;
//         let mut i = 1;

//         while i < n {
//             if chars[i] == chars[len] {
//                 len += 1;
//                 lps[i] = len;
//                 i += 1;
//             } else if len > 0 {
//                 len = lps[len - 1];
//             } else {
//                 lps[i] = 0;
//                 i += 1;
//             }
//         }
//         lps
//     }

//     // KMP search algorithm
//     fn kmp_search(text: &str, pattern: &str) -> bool {
//         if pattern.is_empty() {
//             return true;
//         }
//         if text.is_empty() {
//             return false;
//         }

//         let text_chars: Vec<char> = text.chars().collect();
//         let pattern_chars: Vec<char> = pattern.chars().collect();
//         let lps = Self::compute_lps(pattern);

//         let mut i = 0; // text index
//         let mut j = 0; // pattern index

//         while i < text_chars.len() {
//             if pattern_chars[j] == text_chars[i] {
//                 i += 1;
//                 j += 1;
//                 if j == pattern_chars.len() {
//                     return true;
//                 }
//             } else if j > 0 {
//                 j = lps[j - 1];
//             } else {
//                 i += 1;
//             }
//         }
//         false
//     }

//     pub fn string_matching(words: Vec<String>) -> Vec<String> {
//         words
//             .iter()
//             .filter(|&word| {
//                 words
//                     .iter()
//                     .any(|other| other != word && Self::kmp_search(other, word))
//             })
//             .cloned()
//             .collect()
//     }
// }

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ["mass", "as", "hero", "superhero"]
            .map(|e| e.to_string())
            .to_vec(),
        ["as", "hero"].map(|e| e.to_string()).to_vec(),
    )]
    #[case(
        ["leetcode", "et", "code"].map(|e| e.to_string()).to_vec(),
        ["et", "code"].map(|e| e.to_string()).to_vec(),
    )]
    #[case(
        ["blue", "green", "bu"].map(|e| e.to_string()).to_vec(),
        [].map(|e: &str| e.to_string()).to_vec(),
    )]
    fn test(#[case] words: Vec<String>, #[case] expected: Vec<String>) {
        let result = Solution::string_matching(words);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            ["mass", "as", "hero", "superhero"]
                .map(|e| e.to_string())
                .to_vec(),
            ["as", "hero"].map(|e| e.to_string()).to_vec(),
        ),
        (
            ["leetcode", "et", "code"].map(|e| e.to_string()).to_vec(),
            ["et", "code"].map(|e| e.to_string()).to_vec(),
        ),
        (
            ["blue", "green", "bu"].map(|e| e.to_string()).to_vec(),
            [].map(|e: &str| e.to_string()).to_vec(),
        ),
    ];

    for (words, expected) in test_cases {
        let result = Solution::string_matching(words.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            words, expected, result
        );
        assert_eq!(result, expected);
    }
}

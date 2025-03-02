// https://leetcode.com/problems/shortest-common-supersequence/
// 28/02/2025

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let (str1, str2) = (str1.as_bytes(), str2.as_bytes());
        let (m, n) = (str1.len(), str2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if str1[i - 1] == str2[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    std::cmp::max(dp[i - 1][j], dp[i][j - 1])
                }
            }
        }

        // println!("{:?}", dp);

        let mut result = Vec::<u8>::new();
        let (mut i, mut j) = (m, n);
        while i > 0 && j > 0 {
            if str1[i - 1] == str2[j - 1] {
                result.push(str1[i - 1]);
                (i -= 1, j -= 1);
            } else if dp[i - 1][j] > dp[i][j - 1] {
                result.push(str1[i - 1]);
                i -= 1;
            } else {
                result.push(str2[j - 1]);
                j -= 1;
            }
        }

        while i > 0 {
            result.push(str1[i - 1]);
            i -= 1;
        }

        while j > 0 {
            result.push(str2[j - 1]);
            j -= 1;
        }

        result.reverse();

        String::from_utf8(result).unwrap()
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abac".to_string(), "cab".to_string(), "cabac".to_string())]
    #[case(
        "aaaaaaaa".to_string(),
        "aaaaaaaa".to_string(),
        "aaaaaaaa".to_string(),
    )]
    fn test(#[case] str1: String, #[case] str2: String, #[case] expected: String) {
        let result = Solution::shortest_common_supersequence(str1, str2);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ("abac".to_string(), "cab".to_string(), "cabac".to_string()),
        (
            "aaaaaaaa".to_string(),
            "aaaaaaaa".to_string(),
            "aaaaaaaa".to_string(),
        ),
    ];

    for (str1, str2, expected) in test_cases {
        let result = Solution::shortest_common_supersequence(str1.clone(), str2.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (str1, str2),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}

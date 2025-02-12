// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
// 11/02/2025

// impl Solution {
//     pub fn remove_occurrences(s: String, part: String) -> String {
//         s.bytes()
//             .fold(String::with_capacity(s.len()), |mut acc, c| {
//                 acc.push(c as char);

//                 if acc.contains(&part) {
//                     for _ in 0..part.len() {
//                         acc.pop().unwrap();
//                     }
//                 }

//                 acc
//             })
//     }
// }

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut result = s;
        while let Some(pos) = result.find(&part) {
            result.replace_range(pos..pos + part.len(), "");
        }
        result
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
        "daabcbaabcbc".to_string(),
        "abc".to_string(),
        "dab".to_string(),
    )]
    #[case("axxxxyyyyb".to_string(), "xy".to_string(), "ab".to_string())]
    fn test(#[case] s: String, #[case] part: String, #[case] expected: String) {
        let result = Solution::remove_occurrences(s, part);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            "daabcbaabcbc".to_string(),
            "abc".to_string(),
            "dab".to_string(),
        ),
        ("axxxxyyyyb".to_string(), "xy".to_string(), "ab".to_string()),
    ];

    for (s, part, expected) in test_cases {
        let result = Solution::remove_occurrences(s.clone(), part.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (s, part),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}

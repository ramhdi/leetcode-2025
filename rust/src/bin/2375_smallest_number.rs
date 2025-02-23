// https://leetcode.com/problems/construct-smallest-number-from-di-string/
// 18/02/2025

// impl Solution {
//     pub fn smallest_number(pattern: String) -> String {
//         let n = pattern.len();
//         let mut result = String::new();
//         let mut used = [false; 9];
//         Self::backtrack(&pattern, &mut used, &mut result, n + 1);
//         result
//     }

//     fn backtrack(
//         pattern: &str,
//         used: &mut [bool; 9],
//         result: &mut String,
//         total_digits: usize,
//     ) -> bool {
//         if result.len() == total_digits {
//             return Self::check_pattern(result, pattern);
//         }

//         for i in 0..9 {
//             if !used[i] {
//                 used[i] = true;
//                 result.push((i as u8 + b'1') as char);

//                 if Self::backtrack(pattern, used, result, total_digits) {
//                     return true;
//                 }

//                 result.pop();
//                 used[i] = false;
//             }
//         }

//         false
//     }

//     fn check_pattern(result: &str, pattern: &str) -> bool {
//         let result_bytes = result.as_bytes();
//         let pattern_bytes = pattern.as_bytes();

//         for i in 0..pattern_bytes.len() {
//             if (pattern_bytes[i] == b'I' && result_bytes[i] >= result_bytes[i + 1])
//                 || (pattern_bytes[i] == b'D' && result_bytes[i] <= result_bytes[i + 1])
//             {
//                 return false;
//             }
//         }

//         true
//     }
// }

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut stack: Vec<usize> = vec![]; // Stack to handle decreasing sequences
        let mut result: Vec<usize> = vec![]; // Final result as a vector of numbers

        // Iterate through the pattern
        for (i, c) in pattern.chars().enumerate() {
            match c {
                // If the character is 'I' (increasing)
                'I' => {
                    // Push the current index + 1 to the result
                    result.push(i + 1);
                    // Pop all elements from the stack and append to the result
                    while let Some(num) = stack.pop() {
                        result.push(num);
                    }
                }
                // If the character is 'D' (decreasing)
                'D' => {
                    // Push the current index + 1 to the stack
                    stack.push(i + 1);
                }
                _ => unreachable!(),
            }
        }

        // Push the last number (total length + 1) to the result
        result.push(pattern.len() + 1);
        // Pop any remaining elements from the stack and append to the result
        while let Some(num) = stack.pop() {
            result.push(num);
        }

        // Convert the result vector of numbers into a string
        result
            .into_iter()
            .map(|i| i.to_string())
            .collect::<String>()
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("IIIDIDDD".to_string(), "123549876".to_string())]
    #[case("DDD".to_string(), "4321".to_string())]
    fn test(#[case] pattern: String, #[case] expected: String) {
        let result = Solution::smallest_number(pattern);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ("IIIDIDDD".to_string(), "123549876".to_string()),
        ("DDD".to_string(), "4321".to_string()),
    ];

    for (pattern, expected) in test_cases {
        let result = Solution::smallest_number(pattern.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            pattern, expected, result
        );
        assert_eq!(result, expected);
    }
}

// https://leetcode.com/problems/string-matching-in-an-array/
// 15/02/2025

// impl Solution {
//     pub fn punishment_number(n: i32) -> i32 {
//         (1..=n)
//             .filter_map(|i| {
//                 let sqr = i * i;
//                 if Self::check_partitions(sqr, i) {
//                     Some(sqr)
//                 } else {
//                     None
//                 }
//             })
//             .sum()
//     }

//     fn check_partitions(sqr: i32, target: i32) -> bool {
//         let s = sqr.to_string();
//         let mut current = Vec::new();
//         Self::backtrack(&s, 0, &mut current, target)
//     }

//     fn backtrack(s: &str, start: usize, current: &mut Vec<i32>, target: i32) -> bool {
//         if start == s.len() {
//             return current.iter().sum::<i32>() == target;
//         }
//         for end in (start + 1)..=s.len() {
//             let part = &s[start..end];
//             if let Ok(num) = part.parse::<i32>() {
//                 current.push(num);
//                 if Self::backtrack(s, end, current, target) {
//                     current.pop();
//                     return true;
//                 }
//                 current.pop();
//             }
//         }
//         false
//     }
// }

impl Solution {
    // Precomputed list of valid integers whose squares can be partitioned into substrings summing to the integer itself.
    const VALID_NUMBERS: [i32; 29] = [
        1, 9, 10, 36, 45, 55, 82, 91, 99, 100, 235, 297, 369, 370, 379, 414, 657, 675, 703, 756,
        792, 909, 918, 945, 964, 990, 991, 999, 1000,
    ];

    pub fn punishment_number(n: i32) -> i32 {
        // Find the index of the first number in VALID_NUMBERS that is greater than n.
        let end_index = Self::VALID_NUMBERS
            .iter()
            .position(|&x| x > n)
            .unwrap_or(Self::VALID_NUMBERS.len());

        // Sum the squares of all valid numbers up to the end_index.
        Self::VALID_NUMBERS
            .iter()
            .take(end_index)
            .map(|&val| val * val)
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
    #[case(10, 182)]
    #[case(37, 1478)]
    fn test(#[case] n: i32, #[case] expected: i32) {
        let result = Solution::punishment_number(n);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![(10, 182), (37, 1478)];

    for (n, expected) in test_cases {
        let result = Solution::punishment_number(n);
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            n, expected, result
        );
        assert_eq!(result, expected);
    }
}

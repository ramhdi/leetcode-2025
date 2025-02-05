// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/
// 05/02/2025

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let diff = s1
            .bytes()
            .zip(s2.bytes())
            .filter(|(c1, c2)| c1 != c2)
            .count();

        let (count1, count2) = (Self::count_char(&s1), Self::count_char(&s2));

        (diff == 0 || diff == 2) && count1 == count2
    }

    #[inline(always)]
    fn count_char(s: &String) -> [i32; 26] {
        s.bytes().fold([0; 26], |mut acc, c| {
            acc[(c - b'a') as usize] += 1;
            acc
        })
    }
}

// impl Solution {
//     pub fn are_almost_equal(s1: String, s2: String) -> bool {
//         if s1 == s2 {
//             return true;
//         }

//         let mismatches: Vec<_> = s1
//             .bytes()
//             .zip(s2.bytes())
//             .enumerate()
//             .filter(|(_, (c1, c2))| c1 != c2)
//             .collect();

//         mismatches.len() == 2
//             && mismatches[0].1 .0 == mismatches[1].1 .1
//             && mismatches[0].1 .1 == mismatches[1].1 .0
//     }
// }

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("bank".to_string(), "kanb".to_string(), true)]
    #[case("attack".to_string(), "defend".to_string(), false)]
    #[case("kelb".to_string(), "kelb".to_string(), true)]
    #[case("caa".to_string(), "aaz".to_string(), false)]
    fn test(#[case] s1: String, #[case] s2: String, #[case] expected: bool) {
        let result = Solution::are_almost_equal(s1, s2);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ("bank".to_string(), "kanb".to_string(), true),
        ("attack".to_string(), "defend".to_string(), false),
        ("kelb".to_string(), "kelb".to_string(), true),
    ];

    for (s1, s2, expected) in test_cases {
        let result = Solution::are_almost_equal(s1.clone(), s2.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (s1, s2),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}

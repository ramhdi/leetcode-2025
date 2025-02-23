// https://leetcode.com/problems/find-unique-binary-string/description/
// 20/02/2025

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        nums.iter()
            .enumerate()
            .fold(String::with_capacity(nums.len()), |mut acc, (i, num)| {
                match num.chars().nth(i) {
                    Some(c) => match c {
                        '1' => acc.push('0'),
                        '0' => acc.push('1'),
                        _ => (),
                    },
                    None => (),
                }
                acc
            })
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
        ["01", "10"].map(|e| e.to_string()).to_vec(),
        "11".to_string(),
    )]
    #[case(
        ["00", "01"].map(|e| e.to_string()).to_vec(),
        "11".to_string(),
    )]
    #[case(
        ["111", "011", "001"].map(|e| e.to_string()).to_vec(),
        "101".to_string(),
    )]
    fn test(#[case] nums: Vec<String>, #[case] expected: String) {
        let result = Solution::find_different_binary_string(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            ["01", "10"].map(|e| e.to_string()).to_vec(),
            "11".to_string(),
        ),
        (
            ["00", "01"].map(|e| e.to_string()).to_vec(),
            "11".to_string(),
        ),
        (
            ["111", "011", "001"].map(|e| e.to_string()).to_vec(),
            "101".to_string(),
        ),
    ];

    for (nums, expected) in test_cases {
        let result = Solution::find_different_binary_string(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}

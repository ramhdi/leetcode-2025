// https://leetcode.com/problems/special-array-i/
// 01/02/2025

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|w| (w[0] & 1) != (w[1] & 1))
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1].to_vec(), true)]
    #[case([2, 1, 4].to_vec(), true)]
    #[case([4, 3, 1, 6].to_vec(), false)]
    fn test(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let result = Solution::is_array_special(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([1].to_vec(), true),
        ([2, 1, 4].to_vec(), true),
        ([4, 3, 1, 6].to_vec(), false),
    ];

    for (nums, expected) in test_cases {
        let result = Solution::is_array_special(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}

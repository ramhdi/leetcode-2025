// https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/
// 01/02/2025

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        nums.windows(2)
            .fold((1, 1, 1), |(max_len, inc, dec), w| match w[0].cmp(&w[1]) {
                std::cmp::Ordering::Less => (max_len.max(inc + 1), inc + 1, 1),
                std::cmp::Ordering::Greater => (max_len.max(dec + 1), 1, dec + 1),
                std::cmp::Ordering::Equal => (max_len, 1, 1),
            })
            .0
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1, 4, 3, 3, 2].to_vec(), 2)]
    #[case([3, 3, 3, 3].to_vec(), 1)]
    #[case([3, 2, 1].to_vec(), 3)]
    fn test(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let result = Solution::longest_monotonic_subarray(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([1, 4, 3, 3, 2].to_vec(), 2),
        ([3, 3, 3, 3].to_vec(), 1),
        ([3, 2, 1].to_vec(), 3),
    ];

    for (nums, expected) in test_cases {
        let result = Solution::longest_monotonic_subarray(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}

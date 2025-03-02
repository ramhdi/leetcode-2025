// https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/
// 27/02/2025

use std::collections::HashMap;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let num_idx: HashMap<i32, usize> =
            arr.iter().enumerate().map(|(i, &num)| (num, i)).collect();
        let mut result = 0;

        let mut dp = vec![vec![2; n]; n];
        for j in 0..n {
            for i in 0..j {
                if let Some(&k) = num_idx.get(&(arr[j] - arr[i])) {
                    if k < i {
                        dp[i][j] = dp[k][i] + 1;
                        result = result.max(dp[i][j]);
                    }
                }
            }
        }

        if result < 3 {
            0
        } else {
            result
        }
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([1, 2, 3, 4, 5, 6, 7, 8].to_vec(), 5)]
    #[case([1, 3, 7, 11, 12, 14, 18].to_vec(), 3)]
    fn test(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let result = Solution::len_longest_fib_subseq(nums);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([1, 2, 3, 4, 5, 6, 7, 8].to_vec(), 5),
        ([1, 3, 7, 11, 12, 14, 18].to_vec(), 3),
    ];

    for (nums, expected) in test_cases {
        let result = Solution::len_longest_fib_subseq(nums.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            nums, expected, result
        );
        assert_eq!(result, expected);
    }
}

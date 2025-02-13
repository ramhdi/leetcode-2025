// https://leetcode.com/problems/minimum-operations-to-exceed-threshold-value-ii/
// 12/02/2025

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut pq = nums
            .iter()
            .map(|&e| Reverse(e as i64))
            .collect::<BinaryHeap<_>>();
        let mut result = 0;
        let k = k as i64;

        while pq.len() >= 2 {
            let x = pq.pop().unwrap().0;
            if x >= k {
                return result;
            }
            let y = pq.pop().unwrap().0;
            pq.push(Reverse(x * 2 + y));
            result += 1;
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
    #[case([2, 11, 10, 1, 3].to_vec(), 10, 2)]
    #[case([2, 11, 10, 1, 3].to_vec(), 10, 2)]
    #[case(
        [
            1000000000, 999999999, 1000000000, 999999999, 1000000000, 999999999,
        ]
        .to_vec(),
        1000000000,
        2,
    )]
    fn test(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([2, 11, 10, 1, 3].to_vec(), 10, 2),
        ([1, 1, 2, 4, 9].to_vec(), 20, 4),
        (
            [
                1000000000, 999999999, 1000000000, 999999999, 1000000000, 999999999,
            ]
            .to_vec(),
            1000000000,
            2,
        ),
    ];

    for (nums, k, expected) in test_cases {
        let result = Solution::min_operations(nums.clone(), k);
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            (nums, k),
            expected,
            result
        );
        assert_eq!(result, expected);
    }
}

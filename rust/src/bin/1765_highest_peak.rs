// https://leetcode.com/problems/map-of-highest-peak/
// 22/01/2025

use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (is_water.len(), is_water[0].len());
        let mut result = vec![vec![-1; n]; m];
        let mut queue = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    result[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();

            for (di, dj) in dirs.iter() {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                    let (ni, nj) = (ni as usize, nj as usize);

                    if result[ni][nj] == -1 {
                        result[ni][nj] = result[i][j] + 1;
                        queue.push_back((ni, nj));
                    }
                }
            }
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
        [[0, 1], [0, 0]].map(|e| e.to_vec()).to_vec(),
        [[1, 0], [2, 1]].map(|e| e.to_vec()).to_vec(),
    )]
    #[case(
        [[0, 0, 1], [1, 0, 0], [0, 0, 0]]
            .map(|e| e.to_vec())
            .to_vec(),
        [[1, 1, 0], [0, 1, 1], [1, 2, 2]]
            .map(|e| e.to_vec())
            .to_vec(),
    )]
    fn test(#[case] is_water: Vec<Vec<i32>>, #[case] expected: Vec<Vec<i32>>) {
        let result = Solution::highest_peak(is_water);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            [[0, 1], [0, 0]].map(|e| e.to_vec()).to_vec(),
            [[1, 0], [2, 1]].map(|e| e.to_vec()).to_vec(),
        ),
        (
            [[0, 0, 1], [1, 0, 0], [0, 0, 0]]
                .map(|e| e.to_vec())
                .to_vec(),
            [[1, 1, 0], [0, 1, 1], [1, 2, 2]]
                .map(|e| e.to_vec())
                .to_vec(),
        ),
    ];

    for (is_water, expected) in test_cases {
        let result = Solution::highest_peak(is_water.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            is_water, expected, result
        );
        assert_eq!(result, expected);
    }
}

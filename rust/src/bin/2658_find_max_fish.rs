// https://leetcode.com/problems/maximum-number-of-fish-in-a-grid/
// 28/01/2025

use std::collections::HashSet;

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut fish_pos: Vec<(i32, i32)> = Vec::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] > 0 {
                    fish_pos.push((i as i32, j as i32));
                }
            }
        }

        for (i, j) in fish_pos {
            let mut fish = 0;
            Self::dfs(i, j, &grid, &mut visited, &mut fish);

            result = result.max(fish);
        }

        result
    }

    fn dfs(
        i: i32,
        j: i32,
        grid: &Vec<Vec<i32>>,
        visited: &mut HashSet<(i32, i32)>,
        fish: &mut i32,
    ) {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        if !visited.contains(&(i, j)) {
            visited.insert((i, j));
            *fish += grid[i as usize][j as usize];
            for &(di, dj) in directions.iter() {
                let (ni, nj) = (i + di, j + dj);
                if ni >= 0 && ni < m && nj >= 0 && nj < n && grid[ni as usize][nj as usize] > 0 {
                    Self::dfs(ni, nj, grid, visited, fish);
                }
            }
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
    #[case(
        [[0, 2, 1, 0], [4, 0, 0, 3], [1, 0, 0, 4], [0, 3, 2, 0]]
            .map(|e| e.to_vec())
            .to_vec(),
        7,
    )]
    #[case(
        [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]]
            .map(|e| e.to_vec())
            .to_vec(),
        1,
    )]
    fn test(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let result = Solution::find_max_fish(grid);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        (
            [[0, 2, 1, 0], [4, 0, 0, 3], [1, 0, 0, 4], [0, 3, 2, 0]]
                .map(|e| e.to_vec())
                .to_vec(),
            7,
        ),
        (
            [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]]
                .map(|e| e.to_vec())
                .to_vec(),
            1,
        ),
    ];

    for (grid, expected) in test_cases {
        let result = Solution::find_max_fish(grid.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            grid, expected, result
        );
        assert_eq!(result, expected);
    }
}

// https://leetcode.com/problems/making-a-large-island/
// 31/01/2025

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let grids = grid.iter().flatten();
        if grids.clone().all(|&g| g == 0) {
            return 1;
        }
        if grids.clone().all(|&g| g == 1) {
            return n.pow(2) as i32;
        }

        let mut grid = grid;
        let mut island_area: HashMap<i32, i32> = HashMap::new();
        let mut island_id = 2;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let area = Self::dfs(island_id, i as i32, j as i32, &mut grid);
                    island_area.insert(island_id, area);
                    island_id += 1;
                }
            }
        }

        let mut max_area = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    let mut surrounding_island: HashSet<i32> = HashSet::new();
                    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                    let (i, j) = (i as i32, j as i32);
                    for (di, dj) in dirs {
                        let (ni, nj) = (i + di, j + dj);
                        if ni >= 0
                            && ni < n as i32
                            && nj >= 0
                            && nj < n as i32
                            && grid[ni as usize][nj as usize] > 1
                        {
                            surrounding_island.insert(grid[ni as usize][nj as usize]);
                        }
                    }

                    let combined_area = 1 + surrounding_island
                        .iter()
                        .map(|id| island_area.get(&id).unwrap())
                        .sum::<i32>();
                    max_area = max_area.max(combined_area);
                }
            }
        }

        max_area
    }

    fn dfs(id: i32, i: i32, j: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut area = 1;

        grid[i as usize][j as usize] = id;

        for (di, dj) in dirs {
            let (ni, nj) = (i + di, j + dj);
            if ni >= 0 && ni < n && nj >= 0 && nj < n && grid[ni as usize][nj as usize] == 1 {
                area += Self::dfs(id, ni, nj, grid);
            }
        }

        area
    }
}

#[derive(Debug)]
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([[1, 0], [0, 1]].map(|e| e.to_vec()).to_vec(), 3)]
    #[case([[1, 1], [1, 0]].map(|e| e.to_vec()).to_vec(), 4)]
    #[case([[1, 1], [1, 1]].map(|e| e.to_vec()).to_vec(), 4)]
    fn test(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let result = Solution::largest_island(grid);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([[1, 0], [0, 1]].map(|e| e.to_vec()).to_vec(), 3),
        ([[1, 1], [1, 0]].map(|e| e.to_vec()).to_vec(), 4),
        ([[1, 1], [1, 1]].map(|e| e.to_vec()).to_vec(), 4),
    ];

    for (grid, expected) in test_cases {
        let result = Solution::largest_island(grid.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            grid, expected, result
        );
        assert_eq!(result, expected);
    }
}

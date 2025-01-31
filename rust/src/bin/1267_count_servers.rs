// https://leetcode.com/problems/count-servers-that-communicate/
// 23/01/2025

// impl Solution {
//     pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
//         let (m, n) = (grid.len(), grid[0].len());
//         let mut sum = 0;
//         let mut connected_computers = vec![vec![0; n]; m];
//         let mut result = 0;

//         for i in 0..m {
//             for j in 0..n {
//                 sum += grid[i][j];
//             }

//             if sum > 1 {
//                 for j in 0..n {
//                     connected_computers[i][j] = grid[i][j];
//                 }
//             }

//             sum = 0;
//         }

//         for j in 0..n {
//             for i in 0..m {
//                 sum += grid[i][j];
//             }

//             if sum > 1 {
//                 for i in 0..m {
//                     connected_computers[i][j] = grid[i][j];
//                 }
//             }

//             sum = 0;
//         }

//         for i in 0..m {
//             for j in 0..n {
//                 result += connected_computers[i][j];
//             }
//         }

//         result
//     }
// }

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let col_counts = (0..n)
            .map(|i| (0..m).map(|j| grid[j][i]).sum::<i32>())
            .collect::<Vec<_>>();

        let row_counts = (0..m)
            .map(|i| (0..n).map(|j| grid[i][j]).sum::<i32>())
            .collect::<Vec<_>>();

        (0..m)
            .flat_map(|i| (0..n).map(move |j| (i, j)))
            .filter(|&(i, j)| row_counts[i] > 1 || col_counts[j] > 1)
            .map(|(i, j)| grid[i][j])
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
    #[case([[1, 0], [0, 1]].map(|e| e.to_vec()).to_vec(), 0)]
    #[case([[1, 0], [1, 1]].map(|e| e.to_vec()).to_vec(), 3)]
    #[case(
        [[1, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 1]]
            .map(|e| e.to_vec())
            .to_vec(),
        4,
    )]
    fn test(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let result = Solution::count_servers(grid);
        assert_eq!(result, expected);
    }
}

#[cfg(not(test))]
fn main() {
    let test_cases = vec![
        ([[1, 0], [0, 1]].map(|e| e.to_vec()).to_vec(), 0),
        ([[1, 0], [1, 1]].map(|e| e.to_vec()).to_vec(), 3),
        (
            [[1, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 1]]
                .map(|e| e.to_vec())
                .to_vec(),
            4,
        ),
    ];

    for (grid, expected) in test_cases {
        let result = Solution::count_servers(grid.clone());
        println!(
            "Input: {:?}, Expected: {:?}, Got: {:?}",
            grid, expected, result
        );
        assert_eq!(result, expected);
    }
}
